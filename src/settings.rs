use crate::storage;
use crate::utils::{count_lines_from_path, StatefulList};
use crate::vec_of_strings;
use phf;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::PathBuf;
use tui::style::Color;

pub const SCRIPT_SIGN: &'static str = "#!";

pub fn is_script(text: &str) -> bool {
    if text.len() < 2 {
        return false;
    }
    &text[..2] == SCRIPT_SIGN
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetList {
    Length,
    Frequency,
    Test,
    Mods,
    Nil,
}

#[allow(dead_code)]
pub enum TestVariant {
    Standard,
    Script,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestMod {
    Punctuation,
    Numbers,
    Symbols,
}

pub static TEST_MODS: phf::Map<&'static str, TestMod> = phf::phf_map! {
    "Punctuation" => TestMod::Punctuation,
    "Numbers" => TestMod::Numbers,
    "Symbols" => TestMod::Symbols,
};

impl fmt::Display for TestMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Punctuation => write!(f, "{}", "!?"),
            Self::Numbers => write!(f, "{}", "17"),
            Self::Symbols => write!(f, "{}", "#$"),
        }
    }
}

pub struct TestSummary {
    pub correct_chars: usize,
    pub mistakes: usize,
    pub wpm: f64,
    pub acc: f64,
}

impl Default for TestSummary {
    fn default() -> Self {
        Self {
            correct_chars: 0,
            mistakes: 0,
            wpm: 0.,
            acc: 0.,
        }
    }
}

pub struct TypingTestConfig {
    pub name: String,
    pub variant: TestVariant,
    pub length: usize,
    pub word_pool: usize,
    pub mods: HashSet<TestMod>,
    pub test_summary: TestSummary,
}

impl fmt::Display for TypingTestConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.variant {
            TestVariant::Standard => {
                let mut mods = String::new();
                if !self.mods.is_empty() {
                    mods.push_str("+")
                }
                for test_mod in &self.mods {
                    mods.push_str(&format!(" {}", test_mod));
                }
                write!(
                    f,
                    "{}: {} from {} {}",
                    self.name, self.length, self.word_pool, mods
                )
            }
            _ => write!(f, "{}", self.name),
        }
    }
}

impl Default for TypingTestConfig {
    fn default() -> Self {
        Self {
            name: String::from("english"),
            variant: TestVariant::Standard,
            length: 25,
            word_pool: 5000,
            mods: HashSet::default(),
            test_summary: TestSummary::default(),
        }
    }
}

impl TypingTestConfig {
    pub fn get_words_file_path(&self) -> PathBuf {
        storage::get_word_list_path(&self.name)
    }

    pub fn get_scripts_file_path(&self) -> PathBuf {
        storage::get_storage_dir()
            .join("scripts")
            .join(&self.name[2..])
    }
}

pub struct Settings {
    pub hovered: SetList,
    pub active: SetList,

    pub test_cfg: TypingTestConfig,

    pub length_list: StatefulList<String>,
    pub frequency_list: StatefulList<String>,
    pub tests_list: StatefulList<String>,
    pub mods_list: StatefulList<String>,
    pub word_amount_cache: HashMap<String, usize>,
}

impl Default for Settings {
    fn default() -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);

        // TODO haphazardly implemented cleanup neeeded :broom:
        // lets not kid myself everything is
        // but this especially<question mark>
        let mut words_list: Vec<String> = storage::get_storage_dir()
            .join("words")
            .read_dir()
            .unwrap()
            .map(|i| {
                i.unwrap()
                    .path()
                    .iter()
                    .last()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        let scripts_iterator = storage::get_storage_dir()
            .join("scripts")
            .read_dir()
            .unwrap()
            .map(|i| {
                i.unwrap()
                    .path()
                    .iter()
                    .last()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .map(|s| format!("{}{}", SCRIPT_SIGN, s));

        words_list.extend(scripts_iterator);

        let mod_list: Vec<String> = TEST_MODS.keys().map(|&x| x.to_string()).collect();

        let test_cfg = TypingTestConfig::default();
        let mut word_amount_cache = HashMap::new();

        let word_count = count_lines_from_path(&test_cfg.get_words_file_path());
        word_amount_cache.insert(test_cfg.name.clone(), word_count);
        let frequency_list = create_frequency_list(word_count);

        Self {
            hovered: SetList::Length,
            active: SetList::Nil,

            length_list,
            frequency_list,
            word_amount_cache,
            test_cfg,
            tests_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
        }
    }
}

impl Settings {
    // length freq
    // words mods

    // test length
    // freq mods

    pub fn color_hover_or_active(
        &self,
        hover_color: Color,
        active_color: Color,
    ) -> HashMap<SetList, Option<Color>> {
        let mut hm: HashMap<SetList, Option<Color>> = HashMap::with_capacity(4);
        hm.insert(SetList::Length, None);
        hm.insert(SetList::Test, None);
        hm.insert(SetList::Frequency, None);
        hm.insert(SetList::Mods, None);

        if self.hovered != SetList::Nil {
            hm.insert(self.hovered, Some(hover_color));
            return hm;
        }

        hm.insert(self.active, Some(active_color));
        hm
    }

    pub fn escape(&mut self) -> bool {
        if self.hovered != SetList::Nil {
            true
        } else {
            self.hovered = self.active;
            self.active = SetList::Nil;
            false
        }
    }

    fn get_word_count(&mut self, key: &str) -> usize {
        if let Some(word_count) = self.word_amount_cache.get(key) {
            *word_count
        } else {
            let word_count = count_lines_from_path(storage::get_word_list_path(key));
            self.word_amount_cache.insert(key.to_string(), word_count);
            word_count
        }
    }

    pub fn enter(&mut self) {
        if self.hovered != SetList::Nil {
            self.active = self.hovered;
            self.hovered = SetList::Nil;
            let active_list = self.get_list(self.active).unwrap();
            if active_list.state.selected().is_none() {
                active_list.state.select(Some(0))
            }
            return;
        }

        match self.active {
            SetList::Length => {
                self.test_cfg.length = self.length_list.get_item().parse::<usize>().unwrap()
            }

            SetList::Test => {
                let chosen_test_name = self.tests_list.get_item().clone();

                if is_script(&chosen_test_name) {
                    self.test_cfg.variant = TestVariant::Script;
                } else {
                    self.test_cfg.variant = TestVariant::Standard;
                    let word_count = self.get_word_count(&chosen_test_name);

                    self.frequency_list = create_frequency_list(word_count);
                    if self.test_cfg.word_pool > word_count {
                        self.test_cfg.word_pool = word_count;
                    }
                }

                self.test_cfg.name = chosen_test_name;
            }

            SetList::Frequency => {
                self.test_cfg.word_pool = self
                    .frequency_list
                    .get_item()
                    .parse::<usize>()
                    .unwrap_or(69);
            }
            // TODO this isnt robust implementation
            // It doesnt allow for adding more mods in the future
            // its one of the haphazard changes to make smokey semi-functional before
            // I prob won't be able to work on this for some time
            SetList::Mods => {
                let test_mod = TEST_MODS
                    .get(&self.mods_list.get_item())
                    .expect("UI doesn't match TEST_MODS");

                if !self.test_cfg.mods.remove(test_mod) {
                    self.test_cfg.mods.insert(*test_mod);
                }
            }
            SetList::Nil => unreachable!(),
        }
    }

    pub fn up(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Test,
            SetList::Test => self.hovered = SetList::Length,
            SetList::Frequency => self.hovered = SetList::Mods,
            SetList::Mods => self.hovered = SetList::Frequency,
            SetList::Nil => {
                self.get_list(self.active).unwrap().previous();
            }
        }
    }

    pub fn down(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Test,
            SetList::Test => self.hovered = SetList::Length,
            SetList::Frequency => self.hovered = SetList::Mods,
            SetList::Mods => self.hovered = SetList::Frequency,
            SetList::Nil => {
                self.get_list(self.active).unwrap().next();
            }
        }
    }

    pub fn left(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Frequency,
            SetList::Test => self.hovered = SetList::Mods,
            SetList::Frequency => self.hovered = SetList::Length,
            SetList::Mods => self.hovered = SetList::Test,
            SetList::Nil => {
                self.hovered = self.active;
                self.active = SetList::Nil;
                self.left();
            }
        }
    }

    pub fn right(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Frequency,
            SetList::Test => self.hovered = SetList::Mods,
            SetList::Frequency => self.hovered = SetList::Length,
            SetList::Mods => self.hovered = SetList::Test,
            SetList::Nil => {
                self.hovered = self.active;
                self.active = SetList::Nil;
                self.right();
            }
        }
    }

    pub fn get_list(&mut self, sl: SetList) -> Option<&mut StatefulList<String>> {
        match sl {
            SetList::Length => Some(&mut self.length_list),
            SetList::Frequency => Some(&mut self.frequency_list),
            SetList::Mods => Some(&mut self.mods_list),
            SetList::Test => Some(&mut self.tests_list),
            SetList::Nil => None,
        }
    }
}

fn create_frequency_list(word_count: usize) -> StatefulList<String> {
    let mut initial: Vec<String> = [100, 1000, 5000, 10000, 20000, 50000]
        .iter()
        .filter(|&x| *x < word_count)
        .map(|x| x.to_string())
        .collect();
    initial.push(word_count.to_string());

    StatefulList::with_items(initial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_frequency_list() {
        let large = create_frequency_list(69000);
        let medium = create_frequency_list(15889);
        let small = create_frequency_list(1000);
        let tiny = create_frequency_list(20);

        assert_eq!(
            large.items,
            vec!["100", "1000", "5000", "10000", "20000", "50000", "69000"]
        );
        assert_eq!(medium.items, vec!["100", "1000", "5000", "10000", "15889"]);
        assert_eq!(small.items, vec!["100", "1000"]);
        assert_eq!(tiny.items, vec!["20"]);
    }
}
