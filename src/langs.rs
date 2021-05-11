use crate::application::Config;
use crate::colorscheme;
use colorscheme::ToForeground;
use tui::style::Color;

use super::utils::randorst::Randorst;
use fastrand::Rng as FastRng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tui::text::Span;

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Punctuation {
    // comma, doesnt warrant Capital letter
    Normal(char),
    // full stop, exclamation etc
    End(char),
    // brackets of all kind, and dquotes
    Paired(char, char),
    // gonna use it like an em dash so in between words "word - word"
    DashLike(char),
    Null,
}

#[allow(dead_code)]
struct PFreq {
    weighted_index: WeightedIndex<u16>,
    symbols: Vec<Punctuation>,
}

impl Default for PFreq {
    fn default() -> Self {
        let we = vec![
            (Punctuation::End('.'), 65),
            (Punctuation::End('?'), 6),
            (Punctuation::End('!'), 3),
            (Punctuation::Normal(','), 61),
            (Punctuation::Normal(';'), 3),
            (Punctuation::Normal(':'), 3),
            (Punctuation::Paired('<', '>'), 2),
            (Punctuation::Paired('(', ')'), 3),
            (Punctuation::Paired('{', '}'), 2),
            (Punctuation::Paired('[', ']'), 2),
            (Punctuation::Paired('"', '"'), 13),
            (Punctuation::Paired('\'', '\''), 10),
            (Punctuation::DashLike('-'), 10),
            (Punctuation::Null, 800),
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

#[allow(dead_code)]
impl PFreq {
    fn choose(&self, rng: &mut ThreadRng) -> Punctuation {
        self.symbols[self.weighted_index.sample(rng)]
    }
}

fn get_shuffled_words(config: &Config) -> Vec<String> {
    let file = File::open(&config.get_source()).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let mut container: Vec<String> = Vec::new();

    let mut prng = Randorst::gen(config.length, 0..config.freq_cut_off);
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

pub fn prepare_test<'a>(config: &Config, wrong: Color, todo: Color) -> Vec<Span<'a>> {
    let prep = get_shuffled_words(config);

    let mut test: Vec<Span> = vec![];

    for word in &prep {
        for c in word.chars() {
            test.push(Span::styled(c.to_string(), todo.fg()));
        }
        test.push(Span::styled("", wrong.fg()));
        test.push(Span::styled(" ", todo.fg()));
    }

    test.pop();
    test.pop();

    test
}

#[allow(dead_code)]
pub fn prep_test<'a>(config: &Config, wrong: Color, todo: Color) -> Vec<Vec<Span<'a>>> {
    let prep = get_shuffled_words(config);

    let mut test: Vec<Vec<Span>> = vec![];
    let mut tmp: Vec<Vec<Span>> = vec![vec![]];

    let limit = 20;
    let mut count = 0;

    for word in &prep {
        count += word.len();
        if count > limit {
            test.append(&mut tmp);
            count = word.len();
            tmp.push(vec![]);
        }

        for c in word.chars() {
            tmp[0].push(Span::styled(c.to_string(), todo.fg()));
        }
        tmp[0].push(Span::styled("", wrong.fg()));
        tmp[0].push(Span::styled(" ", todo.fg()));
    }

    let last = tmp.len() - 1;
    tmp[last].pop();
    tmp[last].pop();

    test.append(&mut tmp);
    test

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::Config;
    use tui::style::Color;

    #[test]
    fn test_prep() {
        let mut cfg = Config::default();
        cfg.length = 100;
        let result = prep_test(&cfg, Color::Red, Color::Blue);
        // println!("result {:?}", result);
        for line in &result {
            for span in line {
                print!("{}", span.content);
            }
            println!("");
        }
    }
}
