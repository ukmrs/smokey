use directories_next::ProjectDirs;
use std::path::PathBuf;

// this may be lazy_static later or something
pub fn get_storage_dir() -> PathBuf {
    ProjectDirs::from("pl", "ukmrs", "smokey")
        .expect("no valid home directory could be found")
        .data_dir()
        .to_path_buf()
}

pub fn get_word_list_path(word_list_name: &str) -> PathBuf {
    get_storage_dir().join("words").join(word_list_name)
}
