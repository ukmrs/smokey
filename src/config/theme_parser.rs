use crate::colorscheme::Theme;
use serde_derive::Deserialize;
use std::num::ParseIntError;
use tui::style::Color;

#[derive(Deserialize, Debug)]
pub struct UserTheme {
    done: Option<String>,
    wrong: Option<String>,
    todo: Option<String>,
    hover: Option<String>,
    active: Option<String>,
}

impl UserTheme {
    pub fn to_theme(&self) -> Theme {
        let mut base = Theme::default();

        change_color_to_user_defined(&mut base.done, &self.done);
        change_color_to_user_defined(&mut base.wrong, &self.wrong);
        change_color_to_user_defined(&mut base.todo, &self.todo);
        change_color_to_user_defined(&mut base.active, &self.active);
        change_color_to_user_defined(&mut base.hover, &self.hover);

        base
    }
}

fn change_color_to_user_defined(final_color: &mut Color, user_defined_color: &Option<String>) {
    if let Some(clr_str) = user_defined_color {
        if let Some(clr) = parse_user_defined_colors(clr_str) {
            *final_color = clr;
        }
    }
}

pub fn parse_user_defined_colors(user_color: &str) -> Option<Color> {
    let prepared_user_color = user_color.trim();

    if let Some(c) = prepared_user_color.chars().nth(0) {
        if c == '#' && user_color.len() >= 7 {
            hex_to_color(prepared_user_color).ok()
        } else {
            str_to_color(&prepared_user_color.to_lowercase())
        }
    } else {
        None
    }
}

fn hex_to_color(hex_code: &str) -> Result<Color, ParseIntError> {
    let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
    let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
    let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;
    Ok(Color::Rgb(r, g, b))
}

fn str_to_color(color: &str) -> Option<Color> {
    match color {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" => Some(Color::Gray),
        "darkgray" => Some(Color::DarkGray),
        "lightred" => Some(Color::LightRed),
        "lightgreen" => Some(Color::LightGreen),
        "lightyellow" => Some(Color::LightYellow),
        "lightblue" => Some(Color::LightBlue),
        "lightmagenta" => Some(Color::LightMagenta),
        "lightcyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::UserConfig;
    use super::*;
    use crate::colorscheme::Theme;

    #[test]
    fn test_parse_user_defined_color() {
        // empty string
        assert_eq!(parse_user_defined_colors(""), None);

        // correct hex code
        assert_eq!(
            parse_user_defined_colors("#Fc08f4").unwrap(),
            Color::Rgb(252, 8, 244)
        );

        // correct standard color
        assert_eq!(
            parse_user_defined_colors("LightMagenta").unwrap(),
            Color::LightMagenta
        );

        // invalid hex code
        assert_eq!(parse_user_defined_colors("#0acg4e"), None);

        // too short hex code
        assert_eq!(parse_user_defined_colors("#fc08f"), None);

        // works despite whitespace
        assert_eq!(
            parse_user_defined_colors(" #fc08f4").unwrap(),
            Color::Rgb(252, 8, 244)
        );
        assert_eq!(
            parse_user_defined_colors("#fc08f4 ").unwrap(),
            Color::Rgb(252, 8, 244)
        );

        // works despite capitalization
        assert_eq!(
            parse_user_defined_colors("liGhTGREEN").unwrap(),
            Color::LightGreen
        )
    }

    fn theme_from_config(config: &str) -> Theme {
        let parsed_config: UserConfig = toml::from_str(config).unwrap();
        parsed_config.theme.unwrap().to_theme()
    }

    #[test]
    fn test_parse_partial_theme() {
        // partial
        let partial_config = r##"
        [theme]
        done = "#fc08f4"
        active = "lightyellow"
        # invalid stuff should be ignored
        hover = "rouge"
        todo = ""
    "##;
        let th = Theme {
            done: Color::Rgb(252, 8, 244),
            active: Color::LightYellow,
            ..Theme::default()
        };
        assert_eq!(th, theme_from_config(partial_config));
    }

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

        let th = Theme {
            done: Color::Rgb(252, 8, 244),
            active: Color::LightYellow,
            wrong: Color::Magenta,
            hover: Color::Blue,
            todo: Color::Rgb(255, 0, 0),
        };

        assert_eq!(th, theme_from_config(complete_config));
    }
}
