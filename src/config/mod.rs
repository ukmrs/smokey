mod test_parser;
mod theme_parser;

use crate::{colorscheme::Theme, settings::TypingTestConfig, storage};
use serde_derive::Deserialize;
use std::fs;

use test_parser::UserTest;
use theme_parser::UserTheme;

/// Default Config overwritten partially or completely
/// by user preferences expressed in smokey.toml
#[derive(Default)]
pub struct FinalConfig {
    pub theme: Theme,
    pub typing_test_config: TypingTestConfig,
}

#[derive(Deserialize, Debug)]
struct UserConfig {
    theme: Option<UserTheme>,
    test: Option<UserTest>,
}

impl UserConfig {
    // Consumes user_config and returns FinalConfig
    // that will be used during app runtime
    fn to_final_config(self) -> FinalConfig {
        let final_theme = match self.theme {
            Some(user_theme) => user_theme.to_theme(),
            None => Theme::default(),
        };

        let final_ttc = match self.test {
            Some(user_test) => user_test.to_typing_test_config(),
            None => TypingTestConfig::default(),
        };

        FinalConfig {
            theme: final_theme,
            typing_test_config: final_ttc,
        }
    }
}

fn parse_user_config() -> anyhow::Result<UserConfig> {
    let toml_string = fs::read_to_string(storage::get_config_file())?;
    let user_config: UserConfig = toml::from_str(&toml_string)?;
    Ok(user_config)
}

// Parses smokey.toml and returns FinalConfig struct
pub fn get_final_config() -> FinalConfig {
    match parse_user_config() {
        Ok(user_config) => user_config.to_final_config(),
        _ => FinalConfig::default(),
    }
}
