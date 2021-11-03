use crate::database::{self, RunHistoryDatbase};
use crate::storage;
use crate::utils::{count_lines_from_path, StatefulList};
use crate::vec_of_strings;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use tui::style::Color;

pub const SCRIPT_SIGN: &str = "#!";

use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEST_MODS: BiMap<&'static str, TestMod> = [
        ("punctuation", TestMod::Punctuation),
        ("numbers", TestMod::Numbers),
        ("symbols", TestMod::Symbols),
    ]
    .iter()
    .copied()
    .collect();
}

lazy_static! {
    pub static ref BITFLAG_MODS: BiMap<u8, TestMod> = [
        (0b00000001, TestMod::Punctuation),
        (0b00000010, TestMod::Numbers),
        (0b00000100, TestMod::Symbols),
    ]
    .iter()
    .copied()
    .collect();
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestMod {
    Punctuation,
    Numbers,
    Symbols,
}

impl TestMod {
    pub fn from_bitflag(bitflag: u8) -> Self {
        match bitflag {
            0b00000001 => TestMod::Punctuation,
            0b00000010 => TestMod::Numbers,
            0b00000100 => TestMod::Symbols,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TestMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Punctuation => write!(f, "!?"),
            Self::Numbers => write!(f, "17"),
            Self::Symbols => write!(f, "#$"),
        }
    }
}

pub fn decode_test_mod_bitflags(bitflag: u8) -> HashSet<TestMod> {
    let mut test_mods: HashSet<TestMod> = HashSet::new();

    for i in 0..8 {
        if bitflag >> i & 1 == 1 {
            test_mods.insert(TestMod::from_bitflag(2_u8.pow(i)));
        };
    }

    test_mods
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

#[derive(Default)]
pub struct PostBox {
    pub cached_historic_wpm: f64,
}

/// Basically a dupe of some of the info of ttc
/// but allows me to be more flexible in the future
/// when it comes to caching test info
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TestIdentity {
    pub length: usize,
    pub word_pool: usize,
    pub mods: u8,
}

/// This stuct contains information about
/// test type and also the eventual result of a test
/// these are both dispalyed at the post screen
/// so it makes sense to have them in one place
/// but it is kinda messy???
pub struct TypingTestConfig {
    // test type
    pub name: String,
    pub variant: TestVariant,
    pub length: usize,
    pub word_pool: usize,
    pub mods: HashSet<TestMod>,

    // summary
    pub test_summary: TestSummary,
}

impl fmt::Display for TypingTestConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.variant {
            TestVariant::Standard => {
                let mut mods = String::new();
                if !self.mods.is_empty() {
                    mods.push('+')
                }
                for test_mod in &self.mods {
                    mods.push_str(&format!(" {}", test_mod));
                }
                write!(
                    f,
                    "{}: {}/{} {}",
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
    /// checks if the file name corresponds to a valid path
    /// and whether or not the word_pool field
    /// is in bounds in respect to acutal number of words
    /// avaible in the file
    ///
    /// returns the maximum possible value of word_pool to be cached
    fn validate(&mut self) -> usize {
        let path = self.get_file_path();
        if !path.is_file() {
            self.name = "english".to_string()
        }

        let lines = count_lines_from_path(path).expect("fallback to the english word file");

        if self.word_pool > lines {
            self.word_pool = lines;
        }
        lines
    }

    // TODO rename this XD
    pub fn gib_identity(&self) -> TestIdentity {
        TestIdentity {
            length: self.length,
            word_pool: self.word_pool,
            mods: database::encode_test_mod_bitflag(&self.mods),
        }
    }

    fn get_file_path(&self) -> PathBuf {
        match self.variant {
            TestVariant::Standard => self.get_words_file_path(),
            TestVariant::Script => self.get_scripts_file_path(),
        }
    }

    pub fn get_words_file_path(&self) -> PathBuf {
        storage::get_word_list_path(&self.name)
    }

    pub fn get_scripts_file_path(&self) -> PathBuf {
        storage::get_storage_dir().join("scripts").join(&self.name)
    }
}

pub struct SettingsColors {
    pub hover: Color,
    pub active: Color,
}

impl Default for SettingsColors {
    fn default() -> Self {
        Self {
            hover: Color::Magenta,
            active: Color::Green,
        }
    }
}
// Option feels more clean than f64::NAN
type InfoCache = HashMap<String, (usize, HashMap<TestIdentity, Option<f64>>)>;
type ScriptCache = HashMap<String, Option<f64>>;

pub struct Settings {
    pub hovered: SetList,
    pub active: SetList,
    pub colors: SettingsColors,

    pub test_cfg: TypingTestConfig,

    pub length_list: StatefulList<String>,
    pub frequency_list: StatefulList<String>,
    pub tests_list: StatefulList<String>,
    pub mods_list: StatefulList<String>,
    // HM<test.name (file_word_amount, HM<TestIdentity, historic_max_wpm>)>
    // NaN = historic_max_wpm wasnt cached
    pub info_cache: InfoCache,
    pub script_cache: ScriptCache,

    pub database: RunHistoryDatbase,
    pub postbox: PostBox,
}

impl Default for Settings {
    fn default() -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);
        let words_list = storage::parse_storage_contents();
        let mod_list: Vec<String> = TEST_MODS.left_values().map(|&x| x.to_string()).collect();
        let test_cfg = TypingTestConfig::default();
        let mut info_cache: InfoCache = HashMap::new();
        let word_count = count_lines_from_path(&test_cfg.get_words_file_path()).unwrap();

        // TODO
        // This code is not only ass but also a dupe
        let conn = Connection::open(&*storage::DATABASE).unwrap();
        let max_wpm = database::get_max_wpm(&conn, &test_cfg);

        let mut hs: HashMap<TestIdentity, Option<f64>> = HashMap::new();
        hs.insert(test_cfg.gib_identity(), max_wpm);

        info_cache.insert(test_cfg.name.clone(), (word_count, hs));

        let frequency_list = create_frequency_list(word_count);
        Self {
            hovered: SetList::Length,
            active: SetList::Nil,

            length_list,
            frequency_list,
            info_cache,
            test_cfg,
            tests_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
            colors: SettingsColors::default(),
            script_cache: ScriptCache::default(),
            database: RunHistoryDatbase::default(),
            postbox: PostBox::default(),
        }
    }
}

impl Settings {
    /// TODO a lot of repetitive code taken from default function
    /// restructure ?? idk
    /// I can't do ..Self::default() as that would count lines twice
    pub fn with_config(colors: SettingsColors, ttc: TypingTestConfig) -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);
        let words_list = storage::parse_storage_contents();
        let mod_list: Vec<String> = TEST_MODS.left_values().map(|&x| x.to_string()).collect();

        let mut test_cfg = ttc;
        let word_count = test_cfg.validate();

        let mut info_cache: InfoCache = HashMap::new();

        let conn = Connection::open(&*storage::DATABASE).unwrap();
        let max_wpm = database::get_max_wpm(&conn, &test_cfg);

        let mut hs: HashMap<TestIdentity, Option<f64>> = HashMap::new();
        hs.insert(test_cfg.gib_identity(), max_wpm);

        info_cache.insert(test_cfg.name.clone(), (word_count, hs));

        let frequency_list = create_frequency_list(word_count);
        Self {
            hovered: SetList::Length,
            active: SetList::Nil,

            length_list,
            frequency_list,
            info_cache,
            test_cfg,
            tests_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
            script_cache: ScriptCache::default(),
            database: RunHistoryDatbase::default(),
            postbox: PostBox::default(),
            colors,
        }
    }

    pub fn color_hover_or_active(&self) -> HashMap<SetList, Option<Color>> {
        let mut hm: HashMap<SetList, Option<Color>> = HashMap::with_capacity(4);
        hm.insert(SetList::Length, None);
        hm.insert(SetList::Test, None);
        hm.insert(SetList::Frequency, None);
        hm.insert(SetList::Mods, None);

        if self.hovered != SetList::Nil {
            hm.insert(self.hovered, Some(self.colors.hover));
            return hm;
        }

        hm.insert(self.active, Some(self.colors.active));
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

    pub fn update_historic_max_wpm(&mut self, max_wpm: f64) {
        match self.test_cfg.variant {
            TestVariant::Standard => {
                *self
                    .info_cache
                    .get_mut(&self.test_cfg.name)
                    .unwrap()
                    .1
                    .get_mut(&self.test_cfg.gib_identity())
                    .unwrap() = Some(max_wpm);
            }
            TestVariant::Script => {
                *self.script_cache.get_mut(&self.test_cfg.name).unwrap() = Some(max_wpm);
            }
        }
    }

    // TODO these unwraps may be questionable
    pub fn get_current_historic_max_wpm(&self) -> Option<f64> {
        let first = &self.info_cache.get(&self.test_cfg.name).unwrap().1;
        *first.get(&self.test_cfg.gib_identity()).unwrap()
    }

    pub fn get_current_historic_max_wpm_script(&self) -> Option<f64> {
        *self.script_cache.get(&self.test_cfg.name).unwrap()
    }

    // ------------- TESTEND / DATABASE METHODS ---------------------

    /// This function performs actions needed after test termination
    /// that includes saving results to db and caching new max_wpm if need be
    pub fn save_test_results(&mut self, summary: TestSummary) {
        self.test_cfg.test_summary = summary;
        let final_wpm = self.test_cfg.test_summary.wpm;

        // If record is beat the historic_max_wpm but the
        // previous one is cached so it can be displayed in
        // the post screen
        match self.test_cfg.variant {
            TestVariant::Standard => {
                let historic_max_wpm: f64 = self.get_current_historic_max_wpm().unwrap_or(0.);

                self.postbox.cached_historic_wpm = historic_max_wpm;

                if final_wpm > historic_max_wpm {
                    self.update_historic_max_wpm(final_wpm);
                }
                self.database.save_test(&self.test_cfg);
            }

            TestVariant::Script => {
                // Check for cached max_wpm
                let historic_max_wpm = self
                    .script_cache
                    .get(&self.test_cfg.name)
                    .unwrap()
                    .unwrap_or(0.);

                self.postbox.cached_historic_wpm = historic_max_wpm;
                if final_wpm > historic_max_wpm {
                    *self.script_cache.get_mut(&self.test_cfg.name).unwrap() = Some(final_wpm);
                }
                self.database.save_script(&self.test_cfg);
            }
        }
    }

    pub fn save_run_to_database(&mut self) {
        self.database.save(&self.test_cfg);
    }

    fn cache_historic_max_wpm(&mut self) {
        let tid = self.test_cfg.gib_identity();

        let inner_cache = &mut self
            .info_cache
            .get_mut(&self.test_cfg.name)
            .expect("this should never fail beacuase name is set beforehand")
            .1;

        if inner_cache.get(&tid).is_none() {
            let max_wpm = database::get_max_wpm(&self.database.conn, &self.test_cfg);
            inner_cache.insert(tid, max_wpm);
        }
        debug!("{:?}", &self.info_cache);
    }

    /// TODO
    /// this function is really bad
    fn get_word_count(&mut self) -> usize {
        if let Some(info_cache) = self.info_cache.get(&self.test_cfg.name) {
            info_cache.0
        } else {
            let word_count =
                count_lines_from_path(storage::get_word_list_path(&self.test_cfg.name)).unwrap();
            self.info_cache
                .insert(self.test_cfg.name.clone(), (word_count, HashMap::new()));
            word_count
        }
    }

    // ------------------ KEYBOUND METHODS ------------------

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
            // low priority consideration:
            // take into account these changes
            // and have em ready when user swaps to TestVariant::Standard
            // again. As it stands the changes are ignored
            // which isn't bad tbh
            SetList::Length => {
                if let TestVariant::Script = self.test_cfg.variant {
                    return;
                }
                self.test_cfg.length = self.length_list.get_item().parse::<usize>().unwrap();
                self.cache_historic_max_wpm();
            }

            SetList::Test => {
                let chosen_test_name = self.tests_list.get_item();

                if is_script(chosen_test_name) {
                    self.test_cfg.variant = TestVariant::Script;

                    self.test_cfg.name = chosen_test_name[2..].to_string();
                    debug!("{:?}", &self.test_cfg.name);
                    let hwpm =
                        database::get_max_wpm_script(&self.database.conn, &self.test_cfg.name);
                    debug!("{:?}", hwpm);
                    self.script_cache.insert(self.test_cfg.name.clone(), hwpm);
                } else {
                    self.test_cfg.variant = TestVariant::Standard;
                    self.test_cfg.name = chosen_test_name.to_string();

                    let word_count = self.get_word_count();

                    self.frequency_list = create_frequency_list(word_count);
                    if self.test_cfg.word_pool > word_count {
                        self.test_cfg.word_pool = word_count;
                    }

                    self.cache_historic_max_wpm();
                }
            }

            SetList::Frequency => {
                if let TestVariant::Script = self.test_cfg.variant {
                    return;
                }
                self.test_cfg.word_pool = self
                    .frequency_list
                    .get_item()
                    .parse::<usize>()
                    .unwrap_or(69);
                self.cache_historic_max_wpm();
            }

            SetList::Mods => {
                if let TestVariant::Script = self.test_cfg.variant {
                    return;
                }
                let test_mod = TEST_MODS
                    .get_by_left(self.mods_list.get_item() as &str)
                    .expect("UI doesn't match TEST_MODS");

                if !self.test_cfg.mods.remove(test_mod) {
                    self.test_cfg.mods.insert(*test_mod);
                }
                self.cache_historic_max_wpm();
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
    use std::collections::HashSet;

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

    #[test]
    fn test_decode_bitflags() {
        let ans = decode_test_mod_bitflags(0b00000101);
        let mut hs = HashSet::new();
        hs.insert(TestMod::Punctuation);
        hs.insert(TestMod::Symbols);
        assert_eq!(ans, hs);

        let zero_ans = decode_test_mod_bitflags(0);
        assert!(zero_ans.is_empty());
    }
}
