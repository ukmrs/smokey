use crate::storage::get_storage_dir;
use crate::utils::StatefulList;
use crate::vec_of_strings;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tui::style::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetList {
    Length,
    Frequency,
    Test,
    Mods,
    Nil,
}

#[allow(dead_code)]
enum TestVariant {
    Standard,
    Script,
}

#[derive(PartialEq, Eq, Hash)]
pub enum TestMod {
    Punctuation,
}

#[allow(dead_code)]
pub struct TypingTestConfig {
    pub name: String,
    variant: TestVariant,
    pub length: usize,
    pub frequency: usize,
    pub mods: HashSet<TestMod>,
}

impl fmt::Display for TypingTestConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.variant {
            TestVariant::Standard => {
                let mut mods = String::new();
                if self.mods.contains(&TestMod::Punctuation) {
                    mods.push_str("+ punctuation")
                }
                write!(
                    f,
                    "{}: {} from {} {}",
                    self.name, self.length, self.frequency, mods
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
            frequency: 5000,
            mods: HashSet::default(),
        }
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
}

impl Default for Settings {
    fn default() -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);

        let frequency_list =
            StatefulList::with_items(vec_of_strings!["100", "1000", "5000", "10000", "50000"]);

        // TODO haphazardly implemented cleanup neeeded :broom:
        let words_list: Vec<String> = get_storage_dir()
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

        let mod_list = vec_of_strings!["Punctuation"];

        Self {
            hovered: SetList::Length,
            active: SetList::Nil,

            length_list,
            frequency_list,
            tests_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
            test_cfg: TypingTestConfig::default(),
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
            SetList::Test => self.test_cfg.name = self.tests_list.get_item().clone(),
            SetList::Frequency => {
                self.test_cfg.frequency = self
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
                if !self.test_cfg.mods.remove(&TestMod::Punctuation) {
                    self.test_cfg.mods.insert(TestMod::Punctuation);
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
                self.get_list(self.active);
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
                self.get_list(self.active);
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
