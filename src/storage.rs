use directories_next::ProjectDirs;
use std::path::PathBuf;

fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("pl", "ukmrs", "smokey").expect("no valid directories could be found")
}

pub fn get_storage_dir() -> PathBuf {
    get_project_dirs().data_dir().to_path_buf()
}

pub fn get_word_list_path(word_list_name: &str) -> PathBuf {
    get_storage_dir().join("words").join(word_list_name)
}

// this may be lazy_static later or something
pub fn get_config_dir() -> PathBuf {
    get_project_dirs()
        .config_dir()
        .to_path_buf()
        .join("smokey.toml")
}
