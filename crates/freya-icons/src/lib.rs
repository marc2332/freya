#[cfg(feature = "lucide")]
pub mod lucide {
    include!(concat!(env!("OUT_DIR"), "/lucide.rs"));
}

#[macro_export]
macro_rules! generate_svg {
    ($name:ident, $path:expr) => {
        #[allow(unused)]
        pub fn $name() -> bytes::Bytes {
            bytes::Bytes::from_static(include_bytes!($path))
        }
    };
}
