use crate::colorscheme;
use crate::settings::{TestMod, TypingTestConfig};
use colorscheme::ToForeground;
use std::collections::HashSet;
use tui::style::Color;

use super::utils::randorst::Randorst;
use fastrand::Rng as FastRng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tui::text::Span;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

const SYMBOLS: [char; 14] = [
    '@', '#', '$', '%', '^', '&', '*', '_', '=', '+', '-', '/', '|', '\\',
];

pub struct Capitalize {
    sync: [u8; 2],
}

/// InnerWord represent everything I can throw
/// in between words like numbers symbols dashes
/// and whatever you'd like fair lady / handsome stranger;
// Dash should become character(char) later if I add more
// stuff of that nature
#[derive(Debug, Clone, Copy)]
pub enum InnerWord {
    Dash,
    Number,
    Symbol,
}

#[derive(Debug, Clone, Copy)]
pub enum Punctuation {
    // comma, doesnt warrant Capital letter
    Normal(char),
    // full stop, exclamation etc
    End(char),
    // brackets of all kind, and dquotes
    Paired(char, char),
    // gonna use it like an em dash so in between words "word - word"
    InBetweener(InnerWord),
    Nil,
}

struct PFreq {
    weighted_index: WeightedIndex<u16>,
    symbols: Vec<Punctuation>,
}

impl Default for PFreq {
    // TODO this feels a little bit hacky

    fn default() -> Self {
        let we = vec![
            (Punctuation::End('.'), 65),
            (Punctuation::End('?'), 8),
            (Punctuation::End('!'), 6),
            (Punctuation::Normal(','), 61),
            (Punctuation::Normal(';'), 3),
            (Punctuation::Normal(':'), 3),
            (Punctuation::Paired('<', '>'), 2),
            (Punctuation::Paired('(', ')'), 5),
            (Punctuation::Paired('{', '}'), 2),
            (Punctuation::Paired('[', ']'), 2),
            (Punctuation::Paired('"', '"'), 13),
            (Punctuation::Paired('\'', '\''), 10),
            (Punctuation::InBetweener(InnerWord::Dash), 10),
            // (Punctuation::InBetweener(InnerWord::Number), 200),
            // (Punctuation::InBetweener(InnerWord::Symbol), 200),
            (Punctuation::Nil, 750),
        ];

        let mut weighted_index: Vec<u16> = Vec::with_capacity(we.len());
        let mut symbols: Vec<Punctuation> = Vec::with_capacity(we.len());
        for (p, w) in we.into_iter() {
            weighted_index.push(w);
            symbols.push(p);
        }

        let weighted_index = WeightedIndex::new(weighted_index).unwrap();
        Self {
            weighted_index,
            symbols,
        }
    }
}

impl PFreq {
    fn choose(&self, rng: &mut ThreadRng) -> Punctuation {
        self.symbols[self.weighted_index.sample(rng)]
    }
}

fn get_shuffled_words(config: &TypingTestConfig) -> Vec<String> {
    // This is quick and bad
    // TODO impl more robust system
    let words_file = config.get_words_file_path();

    let file = File::open(words_file).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let mut container: Vec<String> = Vec::new();

    let mut prng = Randorst::gen(config.length, 0..config.frequency);
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

fn add_space_with_blank(container: &mut Vec<Span>, wrong: Color, todo: Color) {
    container.push(Span::styled("", wrong.fg()));
    container.push(Span::styled(" ", todo.fg()));
}

pub fn prep_test<'a>(
    config: &TypingTestConfig,
    limit: usize,
    wrong: Color,
    todo: Color,
) -> Vec<Vec<Span<'a>>> {
    let prep = get_shuffled_words(config);

    let mut test: Vec<Vec<Span>> = vec![];
    let mut tmp: Vec<Vec<Span>> = vec![vec![]];
    let mut count = 0;

    let p = PFreq::default();

    // TODO cleanup this in Config branch
    match config.mods.contains(&TestMod::Punctuation) {
        false => {
            for word in &prep {
                count += word.len() + 1;
                if count > limit {
                    test.append(&mut tmp);
                    count = word.len();
                    tmp.push(vec![]);
                }

                for c in word.chars() {
                    tmp[0].push(Span::styled(c.to_string(), todo.fg()));
                }

                add_space_with_blank(&mut tmp[0], wrong, todo);
            }
        }

        true => {
            let mut rng = thread_rng();
            let mut capitalize = Capitalize::default();
            capitalize.signal(); // start off with a capital letter
            capitalize.capitalize();

            let mut begin: Option<char>;
            let mut end: Option<char>;
            let mut inner_word: Option<InnerWord>;

            for word in &prep {
                count += word.len() + 1;
                if count > limit {
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
                    }

                    Punctuation::Normal(c) => {
                        begin = None;
                        end = Some(c);
                    }

                    Punctuation::Paired(a, z) => {
                        begin = Some(a);
                        end = Some(z);
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

                let mut iter_chars = word.chars();

                if let Some(c) = begin {
                    tmp[0].push(Span::styled(c.to_string(), todo.fg()));
                }

                if capitalize.capitalize() {
                    let upper = iter_chars
                        .next()
                        .expect("word should never be empty")
                        .to_uppercase();
                    for upper_char in upper {
                        tmp[0].push(Span::styled(upper_char.to_string(), todo.fg()));
                    }
                }

                for c in iter_chars {
                    // repetition btw
                    tmp[0].push(Span::styled(c.to_string(), todo.fg()));
                }

                if let Some(c) = end {
                    tmp[0].push(Span::styled(c.to_string(), todo.fg()));
                }

                add_space_with_blank(&mut tmp[0], wrong, todo);

                if let Some(ib) = inner_word {
                    // TODO: do I care for occasional dashes at the end?
                    // propably not but they are kinda ugly not gonna lie
                    match ib {
                        InnerWord::Dash => {
                            tmp[0].push(Span::styled("-".to_string(), todo.fg()));
                        }

                        InnerWord::Number => {
                            for c in rng.gen_range(0..=999).to_string().chars() {
                                tmp[0].push(Span::styled(c.to_string(), todo.fg()));
                            }
                        }

                        InnerWord::Symbol => {
                            let times = rng.gen_range(1..=3);
                            for _ in 0..times {
                                let symbol = SYMBOLS
                                    .choose(&mut rng)
                                    .expect("SYMBOlS shouldn't be empty");

                                tmp[0].push(Span::styled(symbol.to_string(), todo.fg()));
                            }
                        }
                    }

                    add_space_with_blank(&mut tmp[0], wrong, todo);
                }
            }
        }
    };

    let last = tmp.len() - 1;
    tmp[last].pop();
    tmp[last].pop();
    test.append(&mut tmp);

    test.into_iter().rev().collect()
}

impl Default for Capitalize {
    fn default() -> Self {
        Self { sync: [0, 0] }
    }
}

impl Capitalize {
    /// signals that the next word should be capitalized
    fn signal(&mut self) {
        if self.sync[0] == 0 {
            self.sync[0] = 2;
            return;
        }
        self.sync[1] = 2;
    }

    /// checks whether word should be capitalized
    /// should be queried only once per word
    fn capitalize(&mut self) -> bool {
        if self.sync[0] == 1 {
            if self.sync[1] == 0 {
                self.sync[0] = 0;
            } else {
                self.sync = [1, 0];
            }
            return true;
        }
        self.sync[0] = self.sync[0].saturating_sub(1);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::TypingTestConfig;
    use tui::style::Color;

    #[test]
    fn test_prep() {
        let mut cfg = TypingTestConfig::default();
        cfg.length = 200;
        let mut words = 1;
        let limit = 65;
        let mut char_count = 0;

        let result = prep_test(&cfg, limit, Color::Red, Color::Blue);
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
            assert!(char_count <= limit + 1);
            char_count = 0;
        }

        assert_eq!(words, cfg.length);
    }
}
