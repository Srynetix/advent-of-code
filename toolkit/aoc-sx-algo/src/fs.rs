use std::path::PathBuf;

pub fn get_debug_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("debug")
}
