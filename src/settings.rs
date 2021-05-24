use crate::utils::StatefulList;
use std::path::Path;
use crate::vec_of_strings;
use tui::style::Color;
use std::collections::HashMap;


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


    pub fn color_hover_or_active(&self, hover_color: Color, active_color: Color) {
        let mut hm: HashMap<SetList, Color> = HashMap::with_capacity(4);
        hm.insert(SetList::Length, Color::Gray);
        hm.insert(SetList::Words, Color::Gray);
        hm.insert(SetList::Frequency, Color::Gray);
        hm.insert(SetList::Mods, Color::Gray);

        if self.hovered != SetList::Nil {
            hm.insert(self.hovered, hover_color);
            return
        } 

        hm.insert(self.active, active_color);
    }

    pub fn up(&mut self) {
        match self.hovered {
            SetList::Length => self.hovered = SetList::Words,
            SetList::Words => self.hovered = SetList::Length,
            SetList::Frequency => self.hovered = SetList::Mods,
            SetList::Mods => self.hovered = SetList::Frequency,
            SetList::Nil => {
                self.get_list(self.active);
            },
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
