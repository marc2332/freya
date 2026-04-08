use std::{
    collections::{
        HashMap,
        HashSet,
    },
    ops::Deref,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::Context;
use itertools::Itertools;
use object::{
    macho::{
        self,
    },
    read::File,
    write::{
        MachOBuildVersion,
        SectionId,
        StandardSection,
        Symbol,
        SymbolId,
        SymbolSection,
    },
    Endianness,
    Object,
    ObjectSection,
    ObjectSymbol,
    SymbolFlags,
    SymbolKind,
    SymbolScope,
};
use rayon::prelude::{
    IntoParallelRefIterator,
    ParallelIterator,
};
use subsecond_types::{
    AddressMap,
    JumpTable,
};
use target_lexicon::{
    Architecture,
    OperatingSystem,
    PointerWidth,
    Triple,
};
use thiserror::Error;

type Result<T, E = PatchError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum PatchError {
    #[error("Failed to read file: {0}")]
    ReadFs(#[from] std::io::Error),

    #[error("Failed to parse object file, {0}")]
    ParseObjectFile(#[from] object::read::Error),

    #[error("Failed to write object file: {0}")]
    WriteObjectFIle(#[from] object::write::Error),

    #[error("Failed to emit module: {0}")]
    RuntimeError(#[from] anyhow::Error),

    #[error("Failed to read module's PDB file: {0}")]
    PdbLoadError(#[from] pdb::Error),

    #[error("{0}")]
    InvalidModule(String),

    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
}

/// A cache for the hotpatching engine that stores the original module's parsed symbol table.
/// For large projects, this can shave up to 50% off the total patching time. Since we compile the base
/// module with every symbol in it, it can be quite large (hundreds of MB), so storing this here lets
/// us avoid re-parsing the module every time we want to patch it.
///
/// On the Dioxus Docsite, it dropped the patch time from 3s to 1.1s (!)
#[derive(Default)]
pub struct HotpatchModuleCache {
    pub path: PathBuf,

    // ... native stuff
    pub symbol_table: HashMap<String, CachedSymbol>,

    /// Contents of the .tdata section from the original binary (TLS initialization image).
    /// Used to provide correct init data for TLS symbol stubs instead of garbage addresses.
    pub tls_init_data: Vec<u8>,

    /// Map from `$tlv$init` symbol name to (offset_in_tdata, computed_size).
    /// On macOS, Mach-O nlist doesn't carry symbol sizes, so we compute them from
    /// adjacent symbol addresses in the `__thread_data` section. This lets us provide
    /// correctly-sized TLS init data in stubs instead of defaulting to pointer_width.
    pub tls_init_sizes: HashMap<String, (u64, u64)>,
}

pub struct CachedSymbol {
    pub address: u64,
    pub kind: SymbolKind,
    pub is_undefined: bool,
    pub is_weak: bool,
    pub size: u64,
    pub flags: SymbolFlags<SectionId, SymbolId>,
}

impl PartialEq for HotpatchModuleCache {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl std::fmt::Debug for HotpatchModuleCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HotpatchModuleCache")
            .field("_path", &self.path)
            .finish()
    }
}

impl HotpatchModuleCache {
    /// This caching step is crucial for performance on large projects. The original module can be
    /// quite large (hundreds of MB), so this step drastically speeds it up.
    pub fn new(original: &Path, triple: &Triple) -> Result<Self> {
        let cache = match triple.operating_system {
            OperatingSystem::Windows => {
                use pdb::FallibleIterator;

                // due to lifetimes, this code is unfortunately duplicated.
                // the pdb crate doesn't bind the lifetime of the items in the iterator to the symbol table,
                // so we're stuck with local lifetime.s
                let old_pdb_file = original.with_extension("pdb");
                let old_pdb_file_handle = std::fs::File::open(old_pdb_file)?;
                let mut pdb_file = pdb::PDB::open(old_pdb_file_handle)?;
                let global_symbols = pdb_file.global_symbols()?;
                let address_map = pdb_file.address_map()?;
                let mut symbol_table = HashMap::new();
                let mut symbols = global_symbols.iter();
                while let Ok(Some(symbol)) = symbols.next() {
                    match symbol.parse() {
                        Ok(pdb::SymbolData::Public(data)) => {
                            let rva = data.offset.to_rva(&address_map);
                            let is_undefined = rva.is_none();

                            // treat undefined symbols as 0 to match macho/elf
                            let rva = rva.unwrap_or_default();

                            symbol_table.insert(
                                data.name.to_string().to_string(),
                                CachedSymbol {
                                    address: rva.0 as u64,
                                    kind: if data.function {
                                        SymbolKind::Text
                                    } else {
                                        SymbolKind::Data
                                    },
                                    is_undefined,
                                    is_weak: false,
                                    size: 0,
                                    flags: SymbolFlags::None,
                                },
                            );
                        }

                        Ok(pdb::SymbolData::Data(data)) => {
                            let rva = data.offset.to_rva(&address_map);
                            let is_undefined = rva.is_none();

                            // treat undefined symbols as 0 to match macho/elf
                            let rva = rva.unwrap_or_default();

                            symbol_table.insert(
                                data.name.to_string().to_string(),
                                CachedSymbol {
                                    address: rva.0 as u64,
                                    kind: SymbolKind::Data,
                                    is_undefined,
                                    is_weak: false,
                                    size: 0,
                                    flags: SymbolFlags::None,
                                },
                            );
                        }

                        _ => {}
                    }
                }

                HotpatchModuleCache {
                    symbol_table,
                    path: original.to_path_buf(),
                    ..Default::default()
                }
            }

            _ => {
                let old_bytes = std::fs::read(original)?;
                let obj = File::parse(&old_bytes as &[u8])?;
                let symbol_table = obj
                    .symbols()
                    .filter_map(|s| {
                        let flags = match s.flags() {
                            SymbolFlags::None => SymbolFlags::None,
                            SymbolFlags::Elf { st_info, st_other } => {
                                SymbolFlags::Elf { st_info, st_other }
                            }
                            SymbolFlags::MachO { n_desc } => SymbolFlags::MachO { n_desc },
                            _ => SymbolFlags::None,
                        };

                        Some((
                            s.name().ok()?.to_string(),
                            CachedSymbol {
                                address: s.address(),
                                is_undefined: s.is_undefined(),
                                is_weak: s.is_weak(),
                                kind: s.kind(),
                                size: s.size(),
                                flags,
                            },
                        ))
                    })
                    .collect::<HashMap<_, _>>();

                // Extract TLS initialization data and section metadata.
                // This is used to correctly initialize TLS symbols in the stub
                // instead of writing bogus absolute addresses into .tdata.
                let tls_section = obj
                    .sections()
                    .find(|s| matches!(s.name(), Ok(".tdata" | "__thread_data")));

                let tls_init_data = tls_section
                    .as_ref()
                    .and_then(|s| s.data().ok())
                    .unwrap_or(&[])
                    .to_vec();

                // Build TLS init size map for macOS. Mach-O nlist doesn't carry symbol
                // sizes, so we compute them from adjacent symbols in __thread_data.
                // LLVM/rustc names init data symbols as `FOO$tlv$init` in __thread_data.
                let tls_data_addr = tls_section.as_ref().map(|s| s.address()).unwrap_or(0);
                let tls_data_size = tls_section.as_ref().map(|s| s.size()).unwrap_or(0);
                let tls_section_index = tls_section.as_ref().map(|s| s.index());

                let mut tls_init_syms: Vec<(u64, String)> = Vec::new();
                for sym in obj.symbols() {
                    if let (Some(section_idx), Ok(sname)) = (sym.section_index(), sym.name()) {
                        if Some(section_idx) == tls_section_index {
                            let offset = sym.address().saturating_sub(tls_data_addr);
                            tls_init_syms.push((offset, sname.to_string()));
                        }
                    }
                }
                tls_init_syms.sort_by_key(|(addr, _)| *addr);
                tls_init_syms.dedup_by_key(|(addr, _)| *addr);

                let mut tls_init_sizes: HashMap<String, (u64, u64)> = HashMap::new();
                for (i, (offset, sname)) in tls_init_syms.iter().enumerate() {
                    let size = if i + 1 < tls_init_syms.len() {
                        tls_init_syms[i + 1].0 - offset
                    } else {
                        tls_data_size.saturating_sub(*offset)
                    };
                    tls_init_sizes.insert(sname.clone(), (*offset, size));
                }

                HotpatchModuleCache {
                    symbol_table,
                    path: original.to_path_buf(),
                    tls_init_data,
                    tls_init_sizes,
                    ..Default::default()
                }
            }
        };

        Ok(cache)
    }
}

pub fn create_windows_jump_table(patch: &Path, cache: &HotpatchModuleCache) -> Result<JumpTable> {
    use pdb::FallibleIterator;
    let old_name_to_addr = &cache.symbol_table;

    let mut new_name_to_addr = HashMap::new();
    let new_pdb_file_handle = std::fs::File::open(patch.with_extension("pdb"))?;
    let mut pdb_file = pdb::PDB::open(new_pdb_file_handle)?;
    let symbol_table = pdb_file.global_symbols()?;
    let address_map = pdb_file.address_map()?;
    let mut symbol_iter = symbol_table.iter();
    while let Ok(Some(symbol)) = symbol_iter.next() {
        if let Ok(pdb::SymbolData::Public(data)) = symbol.parse() {
            let rva = data.offset.to_rva(&address_map);
            if let Some(rva) = rva {
                new_name_to_addr.insert(data.name.to_string(), rva.0 as u64);
            }
        }
    }

    let mut map = AddressMap::default();
    for (new_name, new_addr) in new_name_to_addr.iter() {
        if let Some(old_addr) = old_name_to_addr.get(new_name.as_ref()) {
            map.insert(old_addr.address, *new_addr);
        }
    }

    let new_base_address = new_name_to_addr
        .get("main")
        .cloned()
        .context("failed to find 'main' symbol in patch")?;

    let aslr_reference = old_name_to_addr
        .get("main")
        .map(|s| s.address)
        .context("failed to find '_main' symbol in original module")?;

    Ok(JumpTable {
        lib: patch.to_path_buf(),
        map,
        new_base_address,
        aslr_reference,
        ifunc_count: 0,
    })
}

/// Assemble a jump table for "nix" architectures. This uses the `object` crate to parse both
/// executable's symbol tables and then creates a mapping between the two. Unlike windows, the symbol
/// tables are stored within the binary itself, so we can use the `object` crate to parse them.
///
/// We use the `_aslr_reference` as a reference point in the base program to calculate the aslr slide
/// both at compile time and at runtime.
///
/// This does not work for WASM since the `object` crate does not support emitting the WASM format,
/// and because WASM requires more logic to handle the wasm-bindgen transformations.
pub fn create_native_jump_table(
    patch: &Path,
    triple: &Triple,
    cache: &HotpatchModuleCache,
) -> Result<JumpTable> {
    let old_name_to_addr = &cache.symbol_table;
    let obj2_bytes = std::fs::read(patch)?;
    let obj2 = File::parse(&obj2_bytes as &[u8])?;
    let mut map = AddressMap::default();
    let new_syms = obj2.symbol_map();

    let new_name_to_addr = new_syms
        .symbols()
        .par_iter()
        .map(|s| (s.name(), s.address()))
        .collect::<HashMap<_, _>>();

    for (new_name, new_addr) in new_name_to_addr.iter() {
        if let Some(old_addr) = old_name_to_addr.get(*new_name) {
            map.insert(old_addr.address, *new_addr);
        }
    }

    let sentinel = main_sentinel(triple);
    let new_base_address = new_name_to_addr
        .get(sentinel)
        .cloned()
        .context("failed to find 'main' symbol in base - are deubg symbols enabled?")?;
    let aslr_reference = old_name_to_addr
        .get(sentinel)
        .map(|s| s.address)
        .context("failed to find 'main' symbol in original module - are debug symbols enabled?")?;

    Ok(JumpTable {
        lib: patch.to_path_buf(),
        map,
        new_base_address,
        aslr_reference,
        ifunc_count: 0,
    })
}

/// Resolve the undefined symbols in the incrementals against the original binary, returning an object
/// file that can be linked along the incrementals.
///
/// This makes it possible to dlopen the resulting object file and use the original binary's symbols
/// bypassing the dynamic linker.
///
/// This is very similar to malware :) but it's not!
///
/// Note - this function is not defined to run on WASM binaries. The `object` crate does not
///
/// todo... we need to wire up the cache
pub fn create_undefined_symbol_stub(
    cache: &HotpatchModuleCache,
    incrementals: &[PathBuf],
    triple: &Triple,
    aslr_reference: u64,
) -> Result<Vec<u8>> {
    let sorted: Vec<_> = incrementals.iter().sorted().collect();

    // Find all the undefined symbols in the incrementals
    let mut undefined_symbols = HashSet::new();
    let mut defined_symbols = HashSet::new();

    for path in sorted {
        let bytes = std::fs::read(path).with_context(|| format!("failed to read {path:?}"))?;
        let file = File::parse(bytes.deref() as &[u8])?;
        for symbol in file.symbols() {
            if symbol.is_undefined() {
                undefined_symbols.insert(symbol.name()?.to_string());
            } else if symbol.is_global() {
                defined_symbols.insert(symbol.name()?.to_string());
            }
        }
    }
    let undefined_symbols: Vec<_> = undefined_symbols
        .difference(&defined_symbols)
        .cloned()
        .collect();

    tracing::trace!("Undefined symbols: {:#?}", undefined_symbols);

    // Create a new object file (architecture doesn't matter much for our purposes)
    let mut obj = object::write::Object::new(
        match triple.binary_format {
            target_lexicon::BinaryFormat::Elf => object::BinaryFormat::Elf,
            target_lexicon::BinaryFormat::Macho => object::BinaryFormat::MachO,
            target_lexicon::BinaryFormat::Coff => object::BinaryFormat::Coff,
            target_lexicon::BinaryFormat::Wasm => object::BinaryFormat::Wasm,
            target_lexicon::BinaryFormat::Xcoff => object::BinaryFormat::Xcoff,
            _ => return Err(PatchError::UnsupportedPlatform(triple.to_string())),
        },
        match triple.architecture {
            Architecture::Aarch64(_) => object::Architecture::Aarch64,
            Architecture::Wasm32 => object::Architecture::Wasm32,
            Architecture::X86_64 => object::Architecture::X86_64,
            _ => return Err(PatchError::UnsupportedPlatform(triple.to_string())),
        },
        match triple.endianness() {
            Ok(target_lexicon::Endianness::Little) => Endianness::Little,
            Ok(target_lexicon::Endianness::Big) => Endianness::Big,
            _ => Endianness::Little,
        },
    );

    // Write the headers so we load properly in ios/macos
    #[allow(clippy::identity_op)]
    match triple.operating_system {
        OperatingSystem::Darwin(_) => {
            obj.set_macho_build_version({
                let mut build_version = MachOBuildVersion::default();
                build_version.platform = macho::PLATFORM_MACOS;
                build_version.minos = (11 << 16) | (0 << 8) | 0; // 11.0.0
                build_version.sdk = (11 << 16) | (0 << 8) | 0; // SDK 11.0.0
                build_version
            });
        }
        OperatingSystem::IOS(_) => {
            obj.set_macho_build_version({
                let mut build_version = MachOBuildVersion::default();
                build_version.platform = match triple.environment {
                    target_lexicon::Environment::Sim => macho::PLATFORM_IOSSIMULATOR,
                    _ => macho::PLATFORM_IOS,
                };
                build_version.minos = (14 << 16) | (0 << 8) | 0; // 14.0.0
                build_version.sdk = (14 << 16) | (0 << 8) | 0; // SDK 14.0.0
                build_version
            });
        }

        _ => {}
    }

    // Get the offset from the main module and adjust the addresses by the slide;
    let aslr_ref_address = cache
        .symbol_table
        .get(main_sentinel(triple))
        .context("failed to find '_main' symbol in patch")?
        .address;

    if aslr_reference < aslr_ref_address {
        return Err(PatchError::InvalidModule(
            format!(
            "ASLR reference is less than the main module's address - is there a `main`?. {aslr_reference:x} < {aslr_ref_address:x}" )
        ));
    }

    let aslr_offset = aslr_reference - aslr_ref_address;

    // we need to assemble a PLT/GOT so direct calls to the patch symbols work
    // for each symbol we either write the address directly (as a symbol) or create a PLT/GOT entry
    let text_section = obj.section_id(StandardSection::Text);
    for name in undefined_symbols {
        let Some(sym) = cache
            .symbol_table
            .get(name.as_str().trim_start_matches("__imp_"))
        else {
            tracing::debug!("Symbol not found: {}", name);
            continue;
        };

        // Undefined symbols tend to be import symbols (darwin gives them an address of 0 until defined).
        // If we fail to skip these, then we end up with stuff like alloc at 0x0 which is quite bad!
        if sym.is_undefined {
            continue;
        }

        // ld64 likes to prefix symbols in intermediate object files with an underscore, but our symbol
        // table doesn't, so we need to strip it off.
        let name_offset = match triple.operating_system {
            OperatingSystem::MacOSX(_) | OperatingSystem::Darwin(_) | OperatingSystem::IOS(_) => 1,
            _ => 0,
        };

        let abs_addr = sym.address + aslr_offset;

        match sym.kind {
            // Handle synthesized window linker cross-dll statics.
            //
            // The `__imp_` prefix is a rather poorly documented feature of link.exe that makes it possible
            // to reference statics in DLLs via text sections. The linker will synthesize a function
            // that returns the address of the static, so calling that function will return the address.
            // We want to satisfy it by creating a data symbol with the contents of the *actual* symbol
            // in the original binary.
            //
            // We ca't use the `__imp_` from the original binary because it was not properly compiled
            // with this in mind. Instead we have to create the new symbol.
            //
            // This is currently only implemented for 64bit architectures (haven't tested 32bit yet).
            //
            // https://stackoverflow.com/questions/5159353/how-can-i-get-rid-of-the-imp-prefix-in-the-linker-in-vc
            _ if name.starts_with("__imp_") => {
                let data_section = obj.section_id(StandardSection::Data);

                // Add a pointer to the resolved address
                let offset = obj.append_section_data(
                    data_section,
                    &abs_addr.to_le_bytes(),
                    8, // Use proper alignment
                );

                // Add the symbol as a data symbol in our data section
                obj.add_symbol(Symbol {
                    name: name.as_bytes().to_vec(),
                    value: offset, // Offset within the data section
                    size: 8,       // Size of pointer
                    scope: SymbolScope::Linkage,
                    kind: SymbolKind::Data, // Always Data for IAT entries
                    weak: false,
                    section: SymbolSection::Section(data_section),
                    flags: SymbolFlags::None,
                });
            }

            // Text symbols are normal code symbols. We need to assemble stubs that resolve the undefined
            // symbols and jump to the original address in the original binary.
            //
            // Unfortunately this isn't simply cross-platform, so we need to handle Unix and Windows
            // calling conventions separately. It also depends on the architecture, making it even more
            // complicated.
            SymbolKind::Text => {
                let jump_asm = match triple.operating_system {
                    // The windows ABI and calling convention is different than the SystemV ABI.
                    OperatingSystem::Windows => match triple.architecture {
                        Architecture::X86_64 => {
                            // Windows x64 has specific requirements for alignment and position-independent code
                            let mut code = vec![
                                0x48, 0xB8, // movabs RAX, imm64 (move 64-bit immediate to RAX)
                            ];
                            // Append the absolute 64-bit address
                            code.extend_from_slice(&abs_addr.to_le_bytes());
                            // jmp RAX (jump to the address in RAX)
                            code.extend_from_slice(&[0xFF, 0xE0]);
                            code
                        }
                        Architecture::X86_32(_) => {
                            // On Windows 32-bit, we can use direct jump but need proper alignment
                            let mut code = vec![
                                0xB8, // mov EAX, imm32 (move immediate value to EAX)
                            ];
                            // Append the absolute 32-bit address
                            code.extend_from_slice(&(abs_addr as u32).to_le_bytes());
                            // jmp EAX (jump to the address in EAX)
                            code.extend_from_slice(&[0xFF, 0xE0]);
                            code
                        }
                        Architecture::Aarch64(_) => {
                            // Use MOV/MOVK sequence to load 64-bit address into X16
                            // This is more reliable than ADRP+LDR for direct hotpatching
                            let mut code = Vec::new();

                            // MOVZ X16, #imm16_0 (bits 0-15 of address)
                            let imm16_0 = (abs_addr & 0xFFFF) as u16;
                            let movz = 0xD2800010u32 | ((imm16_0 as u32) << 5);
                            code.extend_from_slice(&movz.to_le_bytes());

                            // MOVK X16, #imm16_1, LSL #16 (bits 16-31 of address)
                            let imm16_1 = ((abs_addr >> 16) & 0xFFFF) as u16;
                            let movk1 = 0xF2A00010u32 | ((imm16_1 as u32) << 5);
                            code.extend_from_slice(&movk1.to_le_bytes());

                            // MOVK X16, #imm16_2, LSL #32 (bits 32-47 of address)
                            let imm16_2 = ((abs_addr >> 32) & 0xFFFF) as u16;
                            let movk2 = 0xF2C00010u32 | ((imm16_2 as u32) << 5);
                            code.extend_from_slice(&movk2.to_le_bytes());

                            // MOVK X16, #imm16_3, LSL #48 (bits 48-63 of address)
                            let imm16_3 = ((abs_addr >> 48) & 0xFFFF) as u16;
                            let movk3 = 0xF2E00010u32 | ((imm16_3 as u32) << 5);
                            code.extend_from_slice(&movk3.to_le_bytes());

                            // BR X16 (Branch to address in X16)
                            code.extend_from_slice(&[0x00, 0x02, 0x1F, 0xD6]);

                            code
                        }
                        Architecture::Arm(_) => {
                            // For Windows 32-bit ARM, we need a different approach
                            let mut code = Vec::new();
                            // LDR r12, [pc, #8] ; Load the address into r12
                            code.extend_from_slice(&[0x08, 0xC0, 0x9F, 0xE5]);
                            // BX r12 ; Branch to the address in r12
                            code.extend_from_slice(&[0x1C, 0xFF, 0x2F, 0xE1]);
                            // 4-byte alignment padding
                            code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
                            // Store the 32-bit address - 4-byte aligned
                            code.extend_from_slice(&(abs_addr as u32).to_le_bytes());
                            code
                        }
                        _ => return Err(PatchError::UnsupportedPlatform(triple.to_string())),
                    },
                    _ => match triple.architecture {
                        Architecture::X86_64 => {
                            // Use JMP instruction to absolute address: FF 25 followed by 32-bit offset
                            // Then the 64-bit absolute address
                            let mut code = vec![0xFF, 0x25, 0x00, 0x00, 0x00, 0x00]; // jmp [rip+0]
                                                                                     // Append the 64-bit address
                            code.extend_from_slice(&abs_addr.to_le_bytes());
                            code
                        }
                        Architecture::X86_32(_) => {
                            // For 32-bit Intel, use JMP instruction with absolute address
                            let mut code = vec![0xE9]; // jmp rel32
                            let rel_addr = abs_addr as i32 - 5; // Relative address (offset from next instruction)
                            code.extend_from_slice(&rel_addr.to_le_bytes());
                            code
                        }
                        Architecture::Aarch64(_) => {
                            // For ARM64, we load the address into a register and branch
                            let mut code = Vec::new();
                            // LDR X16, [PC, #0]  ; Load from the next instruction
                            code.extend_from_slice(&[0x50, 0x00, 0x00, 0x58]);
                            // BR X16            ; Branch to the address in X16
                            code.extend_from_slice(&[0x00, 0x02, 0x1F, 0xD6]);
                            // Store the 64-bit address
                            code.extend_from_slice(&abs_addr.to_le_bytes());
                            code
                        }
                        Architecture::Arm(_) => {
                            // For 32-bit ARM, use LDR PC, [PC, #-4] to load the address and branch
                            let mut code = Vec::new();
                            // LDR PC, [PC, #-4] ; Load the address into PC (branching to it)
                            code.extend_from_slice(&[0x04, 0xF0, 0x1F, 0xE5]);
                            // Store the 32-bit address
                            code.extend_from_slice(&(abs_addr as u32).to_le_bytes());
                            code
                        }
                        _ => return Err(PatchError::UnsupportedPlatform(triple.to_string())),
                    },
                };
                let offset = obj.append_section_data(text_section, &jump_asm, 8);
                obj.add_symbol(Symbol {
                    name: name.as_bytes()[name_offset..].to_vec(),
                    value: offset,
                    size: jump_asm.len() as u64,
                    scope: SymbolScope::Linkage,
                    kind: SymbolKind::Text,
                    weak: false,
                    section: SymbolSection::Section(text_section),
                    flags: SymbolFlags::None, // ignore for these stubs
                });
            }

            // Rust code typically generates Tls accessors as functions (text), but they are referenced
            // indirectly as data symbols. We end up handling this by adding the TLS symbol as a data
            // symbol with the initializer as the address of the original tls initializer. That way
            // if new TLS are added at runtime, they get initialized properly, but otherwise, the
            // tls initialization check (cbz) properly skips re-initialization on patches.
            //
            // ```
            // __ZN17crossbeam_channel5waker17current_thread_id9THREAD_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h33618d877d86bb77E:
            //    stp     x20, x19, [sp, #-0x20]!
            //    stp     x29, x30, [sp, #0x10]
            //    add     x29, sp, #0x10
            //    adrp    x19, 21603 ; 0x1054bd000
            //    add     x19, x19, #0x998
            //    ldr     x20, [x19]
            //    mov     x0, x19
            //    blr     x20
            //    ldr     x8, [x0]
            //    cbz     x8, 0x10005acc0
            //    mov     x0, x19
            //    blr     x20
            //    ldp     x29, x30, [sp, #0x10]
            //    ldp     x20, x19, [sp], #0x20
            //    ret
            //    mov     x0, x19
            //    blr     x20
            //    bl      __ZN3std3sys12thread_local6native4lazy20Storage$LT$T$C$D$GT$10initialize17h818476638edff4e6E
            //    b       0x10005acac
            // ```
            SymbolKind::Tls => {
                let tls_section = obj.section_id(StandardSection::Tls);

                let pointer_width = match triple.pointer_width().unwrap() {
                    PointerWidth::U16 => 2,
                    PointerWidth::U32 => 4,
                    PointerWidth::U64 => 8,
                };

                // Resolve the TLS init data offset and size.
                //
                // On ELF: sym.address IS the TLS offset and sym.size is the data size.
                // On Mach-O: sym.address points to __thread_vars (TLV descriptor), NOT
                // __thread_data. Mach-O nlist has no size field (always 0). We look up
                // the corresponding $tlv$init symbol (LLVM convention) to get the real
                // offset and size within __thread_data.
                //
                // Note: each patch gets its own TLS copy (not shared with the main exe).
                // TLS variables reset to their initial value on patch.
                // Use the full name (with Mach-O `_` prefix) since tls_init_sizes
                // keys come from the same symbol table and include the prefix.
                let init_key = format!("{}$tlv$init", name);
                let (tls_offset, size) =
                    if let Some(&(offset, size)) = cache.tls_init_sizes.get(&init_key) {
                        // macOS: found the $tlv$init symbol with correct offset and size
                        (offset, size)
                    } else if sym.size > 0 {
                        // ELF: sym.address is the TLS offset, sym.size is the data size
                        (sym.address, sym.size)
                    } else if !cache.tls_init_sizes.is_empty() {
                        // macOS fallback: $tlv$init not found but map isn't empty (binary
                        // might be partially stripped). Use entire tdata as upper bound.
                        (0, cache.tls_init_data.len() as u64)
                    } else {
                        // Last resort (ELF with size=0): use pointer width
                        (sym.address, pointer_width)
                    };

                let align = size.min(pointer_width).next_power_of_two();

                let start = tls_offset as usize;
                let end = start + size as usize;
                let init = if end <= cache.tls_init_data.len() {
                    cache.tls_init_data[start..end].to_vec()
                } else {
                    // Beyond .tdata bounds (.tbss) or Mach-O fallback: zero-init
                    vec![0u8; size as usize]
                };

                // Use add_symbol_data() so the object crate's Mach-O writer auto-creates
                // __thread_vars TLV descriptors (via macho_add_thread_var). Without this,
                // the symbol stays in __thread_data and the runtime misinterprets raw init
                // bytes as a TLV descriptor — first 8 bytes become the thunk pointer.
                let sym_id = obj.add_symbol(Symbol {
                    name: name.as_bytes()[name_offset..].to_vec(),
                    value: 0,
                    size: 0,
                    scope: SymbolScope::Linkage,
                    kind: SymbolKind::Tls,
                    weak: false,
                    section: SymbolSection::Undefined,
                    flags: SymbolFlags::None,
                });
                obj.add_symbol_data(sym_id, tls_section, &init, align);
            }

            // We just assume all non-text symbols are data (globals, statics, etc)
            _ => {
                // darwin statics show up as "unknown" symbols even though they are data symbols.
                let kind = match sym.kind {
                    SymbolKind::Unknown => SymbolKind::Data,
                    k => k,
                };

                // plain linux *wants* these flags, but android doesn't.
                // unsure what's going on here, but this is special cased for now.
                // I think the more advanced linkers don't want these flags, but the default linux linker (ld) does.
                let flags = match triple.environment {
                    target_lexicon::Environment::Android => SymbolFlags::None,
                    _ => sym.flags,
                };

                obj.add_symbol(Symbol {
                    name: name.as_bytes()[name_offset..].to_vec(),
                    value: abs_addr,
                    size: 0,
                    scope: SymbolScope::Linkage,
                    kind,
                    weak: sym.is_weak,
                    section: SymbolSection::Absolute,
                    flags,
                });
            }
        }
    }

    Ok(obj.write()?)
}

/// Get the main sentinel symbol for the given target triple
///
/// We need to special case darwin since `main` is the entrypoint but `_main` is the actual symbol.
/// The entrypoint ends up outside the text section, seemingly, and breaks our aslr detection.
fn main_sentinel(triple: &Triple) -> &'static str {
    match triple.operating_system {
        // The symbol in the symtab is called "_main" but in the dysymtab it is called "main"
        OperatingSystem::MacOSX(_) | OperatingSystem::Darwin(_) | OperatingSystem::IOS(_) => {
            "_main"
        }

        _ => "main",
    }
}
