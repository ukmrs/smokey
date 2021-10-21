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
    pub fn into_typing_test_config(self) -> TypingTestConfig {
        let name = self.name.unwrap_or_else(|| "english".to_string());
        let variant = resolve_test_variant(&name);
        let mut ttc = TypingTestConfig {
            name,
            variant,
            ..TypingTestConfig::default()
        };

        if let Some(length) = self.len {
            if length > 0 {
                ttc.length = length
            }
        }

        if let Some(word_pool) = self.pool {
            if word_pool > 0 {
                ttc.word_pool = word_pool
            }
        }

        if let Some(mods) = self.mods {
            ttc.mods = parse_mods(&mods)
        }

        ttc
    }
}

fn parse_mods(raw_mods: &[String]) -> HashSet<TestMod> {
    let mut parsed_mods = HashSet::new();
    for raw_mod in raw_mods {
        if let Some(&parsed_mod) = TEST_MODS.get_by_left(&raw_mod as &str) {
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
    use crate::settings::TestMod;
    use crate::vec_of_strings;
    use std::collections::HashSet;

    #[test]
    fn test_parse_mods() {
        let faulty = vec_of_strings!["nonexitant", "punctuation"];
        let full = vec_of_strings!["numbers", "punctuation", "symbols"];
        let empty: Vec<String> = vec_of_strings![];
        let mut hs = HashSet::new();
        assert_eq!(parse_mods(&empty), hs);
        hs.insert(TestMod::Punctuation);
        assert_eq!(parse_mods(&faulty), hs);
        hs.insert(TestMod::Numbers);
        hs.insert(TestMod::Symbols);
        assert_eq!(parse_mods(&full), hs);
    }
}
