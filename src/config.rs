use crate::langs::Punctuation;
use directories_next::ProjectDirs;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct TestType {
    punctuation: Option<Punctuation>
}

struct Config {
    words: PathBuf,
    source: String,
    length: u32,
    test_type: TestType,
}

impl Default for Config {
    fn default() -> Self {
        let base = ProjectDirs::from("pl", "ukmrs", "smokey").unwrap().data_dir().to_path_buf();
        Config {
            words: base.join("storage").join("words"),
            source: String::from("english"),
            length: 15,
            test_type: TestType::default(),
        }
    }
}

impl Config {
    fn get_source(&self) -> PathBuf {
        self.words.join(&self.source)
    }
} 
