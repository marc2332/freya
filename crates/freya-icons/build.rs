use std::{
    fs,
    path::Path,
};

fn main() {
    generate("lucide", "./external/lucide/icons");
}

fn generate(name: &str, path: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(name).with_extension("rs");

    let mut code = String::new();

    for entry in fs::read_dir(path).expect("Failed to read icons directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let path_for_include = path
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\\", "\\\\");

            fn sanitize_name(name: &str) -> String {
                let mut name = name
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                    .collect();
                if name == "box" {
                    name = "icon_box".to_string();
                } else if name == "type" {
                    name = "icon_type".to_string();
                } else if name == "move" {
                    name = "icon_move".to_string();
                }
                name
            }

            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let func_name = sanitize_name(file_stem);

            code.push_str(&format!(
                "crate::generate_svg!({name}, \"{path}\");\n",
                name = func_name,
                path = path_for_include
            ));
        }
    }

    fs::write(dest_path, code).expect("Failed to write generated file");
}
