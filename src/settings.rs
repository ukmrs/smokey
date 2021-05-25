use crate::utils::StatefulList;
use crate::vec_of_strings;
use std::collections::HashMap;
use std::path::Path;
use tui::style::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetList {
    Length,
    Frequency,
    Words,
    Mods,
    Nil,
}

pub struct Settings {
    pub hovered: SetList,
    pub active: SetList,

    pub length_list: StatefulList<String>,
    pub frequency_list: StatefulList<String>,
    pub words_list: StatefulList<String>,
    pub mods_list: StatefulList<String>,
}

impl Settings {
    pub fn new(path: &Path) -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);

        let frequency_list =
            StatefulList::with_items(vec_of_strings!["100", "1k", "5k", "10k", "max"]);

        let words_list: Vec<String> = path
            .iter()
            .map(|i| i.to_string_lossy().to_string())
            .collect();

        let mod_list = vec_of_strings!["Punctuation"];

        Self {
            hovered: SetList::Length,
            active: SetList::Nil,

            length_list,
            frequency_list,
            words_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
        }
    }

    // length freq
    // words mods

    pub fn color_hover_or_active(
        &self,
        hover_color: Color,
        active_color: Color,
    ) -> HashMap<SetList, Option<Color>> {
        let mut hm: HashMap<SetList, Option<Color>> = HashMap::with_capacity(4);
        hm.insert(SetList::Length, None);
        hm.insert(SetList::Words, None);
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

        }

    }

    pub fn up(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Words,
            SetList::Words => self.hovered = SetList::Length,
            SetList::Frequency => self.hovered = SetList::Mods,
            SetList::Mods => self.hovered = SetList::Frequency,
            SetList::Nil => {
                self.get_list(self.active);
            }
        }
    }

    pub fn down(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Words,
            SetList::Words => self.hovered = SetList::Length,
            SetList::Frequency => self.hovered = SetList::Mods,
            SetList::Mods => self.hovered = SetList::Frequency,
            SetList::Nil => {
                self.get_list(self.active);
            }
        }
    }

    pub fn left(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Frequency,
            SetList::Words => self.hovered = SetList::Mods,
            SetList::Frequency => self.hovered = SetList::Length,
            SetList::Mods => self.hovered = SetList::Words,
            SetList::Nil => {
                self.get_list(self.active);
            }
        }
    }

    pub fn right(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Frequency,
            SetList::Words => self.hovered = SetList::Mods,
            SetList::Frequency => self.hovered = SetList::Length,
            SetList::Mods => self.hovered = SetList::Words,
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
            SetList::Words => Some(&mut self.words_list),
            SetList::Nil => None,
        }
    }
}
