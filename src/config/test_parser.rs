use crate::settings::{is_script, TestMod, TestVariant, TypingTestConfig, TEST_MODS};
use std::collections::HashSet;

#[derive(serde_derive::Deserialize, Debug)]
pub struct UserTest {
    name: Option<String>,
    len: Option<usize>,
    pool: Option<usize>,
    mods: Option<Vec<String>>,
}

impl UserTest {
    /// consumes UserTest and returns TypingTestConfig
    fn to_typing_test_config(self) -> TypingTestConfig {
        let name = self.name.unwrap_or_else(|| "english".to_string());
        let variant = resolve_test_variant(&name);
        let mut ttc = TypingTestConfig {
            name,
            variant,
            ..TypingTestConfig::default()
        };

        if let Some(length) = self.len {
            ttc.length = length
        }

        if let Some(word_pool) = self.pool {
            ttc.word_pool = word_pool
        }

        if let Some(mods) = self.mods {
            ttc.mods = parse_mods(&mods)
        }

        ttc
    }
}

fn parse_mods(raw_mods: &Vec<String>) -> HashSet<TestMod> {
    let mut parsed_mods = HashSet::new();
    for raw_mod in raw_mods {
        if let Some(&parsed_mod) = TEST_MODS.get(raw_mod) {
            parsed_mods.insert(parsed_mod);
        }
    }
    parsed_mods
}

fn resolve_test_variant(test_name: &str) -> TestVariant {
    if is_script(test_name) {
        TestVariant::Script
    } else {
        TestVariant::Standard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complete_theme() {
        // complete
        let complete_config = r##"
        [theme]
        done = "#fc08f4"
        active = "lightyellow"
        wrong = "maGenta"
        hover = "BLUE"
        todo = "#ff0000"
    "##;
    }
}
