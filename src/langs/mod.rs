mod helpers;
mod punctuation;

use crate::colorscheme::ToForeground;
use crate::settings::{TestVariant, TypingTestConfig};
use crate::typer::TestColors;
use helpers::{Capitalize, SpanIntake};
use punctuation::{InnerWord, Punctuation, PunctuationInsertFrequency};
use std::ffi::OsStr;
use std::process::Command;

use super::utils::randorst::Randorst;
use fastrand::Rng as FastRng;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use tui::text::Span;

const SYMBOLS: [char; 14] = [
    '@', '#', '$', '%', '^', '&', '*', '_', '=', '+', '-', '/', '|', '\\',
];

const LIMIT: usize = 65;

pub fn prepare_test<'a>(config: &TypingTestConfig, colors: &TestColors) -> Vec<Vec<Span<'a>>> {
    match config.variant {
        TestVariant::Standard => prepare_standart_test(config, colors),
        TestVariant::Script => prepare_script_test(config, colors),
    }
}

fn prepare_script_test<'a>(config: &TypingTestConfig, colors: &TestColors) -> Vec<Vec<Span<'a>>> {
    let script_output = call_script(config.get_scripts_file_path());
    let testable = to_testable_span(&script_output, colors);
    testable
}

fn prepare_standart_test<'a>(config: &TypingTestConfig, colors: &TestColors) -> Vec<Vec<Span<'a>>> {
    let prep = get_shuffled_words(config);

    let mut test: Vec<Vec<Span>> = vec![];
    let mut tmp: Vec<Vec<Span>> = vec![vec![]];
    let mut count = 0;

    match config.mods.is_empty() {
        true => {
            for word in &prep {
                count += word.len() + 1;
                if count > LIMIT {
                    test.append(&mut tmp);
                    count = word.len();
                    tmp.push(vec![]);
                }

                for c in word.chars() {
                    tmp[0].push_styled_char(c, colors.todo);
                }

                add_space_with_blank(&mut tmp[0], colors);
            }
        }

        false => return prepare_modded_test(config, &prep, colors),
    };

    let last = tmp.len() - 1;
    tmp[last].pop();
    tmp[last].pop();
    test.append(&mut tmp);

    test.into_iter().rev().collect()
}

fn get_shuffled_words(config: &TypingTestConfig) -> Vec<String> {
    // This is quick and bad
    // TODO impl more robust system
    let words_file = config.get_words_file_path();

    let file = File::open(words_file).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let mut container: Vec<String> = Vec::new();

    let mut prng = Randorst::gen(config.length, 0..config.word_pool);
    let mut last = prng.next().unwrap();
    let out = line_iter.nth(last).unwrap().unwrap();
    container.push(out);
    let mut cached_word: usize = container.len() - 1;

    for (i, val) in prng.enumerate() {
        if val == last {
            container.push(container[cached_word].to_string());
            continue;
        }
        container.push(line_iter.nth(val - last - 1).unwrap().unwrap());
        cached_word = i + 1;
        last = val;
    }

    FastRng::new().shuffle(&mut container);
    container
}

fn add_space_with_blank(container: &mut Vec<Span>, colors: &TestColors) {
    container.push(Span::styled("", colors.wrong.fg()));
    container.push(Span::styled(" ", colors.todo.fg()));
}

// calls script
fn call_script(script_path: impl AsRef<OsStr>) -> String {
    let output = Command::new(script_path).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}

fn to_testable_span<'a>(text: &str, colors: &TestColors) -> Vec<Vec<Span<'a>>> {
    let mut word: Vec<Span> = vec![];
    let mut lines: Vec<Vec<Span>> = vec![];
    let mut tmp: Vec<Vec<Span>> = vec![vec![]];
    let mut count = 0;
    let mut duplicate_whitespace_flag: bool = false;

    for c in text.chars() {
        count += 1;
        if c.is_whitespace() {
            if !duplicate_whitespace_flag {
                add_space_with_blank(&mut word, colors);
                tmp[0].append(&mut word);
                duplicate_whitespace_flag = true;
            }
        } else {
            duplicate_whitespace_flag = false;
            word.push_styled_char(c, colors.todo);
            if count > LIMIT {
                lines.append(&mut tmp);
                count = word.len();
                tmp.push(vec![]);
            }
        }
    }

    lines.append(&mut tmp);
    lines = lines.into_iter().rev().collect();
    lines[0].pop();
    lines[0].pop();
    lines
}

fn prepare_modded_test<'a>(
    config: &TypingTestConfig,
    words: &Vec<String>,
    colors: &TestColors,
) -> Vec<Vec<Span<'a>>> {
    let p = PunctuationInsertFrequency::from_test_mods(&config.mods);

    let mut test: Vec<Vec<Span>> = vec![];
    let mut tmp: Vec<Vec<Span>> = vec![vec![]];
    let mut count = 0;

    let mut rng = thread_rng();

    // setup capitalazion for Punctuation::End
    let mut capitalize = Capitalize::default();
    capitalize.signal(); // start off with a capital letter
    capitalize.capitalize();

    // variables signaling variety of options of inserting
    // stuff into the text
    let mut begin: Option<char>;
    let mut end: Option<char>;
    let mut inner_word: Option<InnerWord>;

    for word in words {
        count += word.len() + 1;
        if count > LIMIT {
            test.append(&mut tmp);
            count = word.len();
            tmp.push(vec![]);
        }

        let punct = p.choose(&mut rng);

        inner_word = None;
        match punct {
            Punctuation::Nil => {
                begin = None;
                end = None;
            }

            Punctuation::End(c) => {
                capitalize.signal();
                begin = None;
                end = Some(c);
                count += 1;
            }

            Punctuation::Normal(c) => {
                begin = None;
                end = Some(c);
                count += 1;
            }

            Punctuation::Paired(a, z) => {
                begin = Some(a);
                end = Some(z);
                count += 2;
            }

            // TODO implement this bullshit
            // i am kinda fed up with what this became
            // need to think it through
            Punctuation::InBetweener(in_betweener) => {
                begin = None;
                end = None;
                inner_word = Some(in_betweener);
            }
        }

        if let Some(c) = begin {
            tmp[0].push_styled_char(c, colors.todo);
        }

        // the part where actual word is inserted
        // First letter
        let mut iter_chars = word.chars();
        if capitalize.capitalize() {
            let upper = iter_chars
                .next()
                .expect("word should never be empty")
                .to_uppercase();
            for upper_char in upper {
                tmp[0].push_styled_char(upper_char, colors.todo);
            }
        }

        // rest of the word
        for c in iter_chars {
            // repetition btw
            tmp[0].push_styled_char(c, colors.todo);
        }

        if let Some(c) = end {
            tmp[0].push_styled_char(c, colors.todo);
        }

        add_space_with_blank(&mut tmp[0], colors);

        if let Some(ib) = inner_word {
            // TODO: do I care for occasional dashes at the end?
            // propably not but they are kinda ugly not gonna lie
            match ib {
                InnerWord::Dash => {
                    tmp[0].push_styled_char('-', colors.todo);
                    count += 1;
                }

                InnerWord::Number => {
                    let number = rng.gen_range(0..=999).to_string();
                    count += number.len();
                    for c in number.chars() {
                        tmp[0].push_styled_char(c, colors.todo);
                    }
                }

                InnerWord::Symbol => {
                    let times = rng.gen_range(1..=3);
                    count += times;
                    for _ in 0..times {
                        let symbol = SYMBOLS
                            .choose(&mut rng)
                            .expect("SYMBOlS shouldn't be empty");

                        tmp[0].push_styled_char(*symbol, colors.todo);
                    }
                }
            }
            add_space_with_blank(&mut tmp[0], colors);
        }
    }

    let last = tmp.len() - 1;
    tmp[last].pop();
    tmp[last].pop();
    test.append(&mut tmp);

    test.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::TypingTestConfig;
    use crate::typer::TestColors;

    #[test]
    fn test_prep() {
        let mut cfg = TypingTestConfig::default();
        cfg.length = 200;
        let mut words = 1;
        let mut char_count = 0;

        let result = prepare_test(&cfg, &TestColors::default());
        for line in &result {
            for span in line {
                if span.content == " " {
                    words += 1;
                }
                if !span.content.is_empty() {
                    char_count += 1;
                }
            }
            // there can be space at the end and I dont care for it
            assert!(char_count <= LIMIT + 1);
            char_count = 0;
        }

        assert_eq!(words, cfg.length);
    }
}
