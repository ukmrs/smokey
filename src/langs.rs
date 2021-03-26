use crate::colorscheme;
use colorscheme::Theme;

use rand::seq::SliceRandom;
use std::fs;
use tui::text::Span;

#[allow(dead_code)]
use rand::distributions::WeightedIndex;
use rand::prelude::*;

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

impl PFreq {
    fn choose(&self, rng: &mut ThreadRng) -> Punctuation {
        self.symbols[self.weighted_index.sample(rng)]
    }
}

pub fn mock<'a>(th: &'a Theme) -> Vec<Span<'a>> {
    let msg = fs::read_to_string("./typedbg/typetest").expect("can load test");
    let msg = msg.trim_end();
    let mut test = Vec::with_capacity(msg.len() * 2);

    for c in msg.chars() {
        if c == ' ' {
            test.push(Span::styled("", th.wrong));
            test.push(Span::styled(" ", th.todo));
        } else {
            test.push(Span::styled(c.to_string(), th.todo));
        }
    }
    test
}

pub fn prepare_test<'a>(source: &str, length: u32, th: &'a Theme) -> Vec<Span<'a>> {
    let rd = fs::read_to_string(source).unwrap();
    let prep = rd.trim_end().split('\n').collect::<Vec<&str>>();

    let mut test: Vec<Span> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..length - 1 {
        for c in prep.choose(&mut rng).unwrap().chars() {
            test.push(Span::styled(c.to_string(), th.todo));
        }
        test.push(Span::styled("", th.wrong));
        test.push(Span::styled(" ", th.todo));
    }

    for c in prep.choose(&mut rng).unwrap().chars() {
        test.push(Span::styled(c.to_string(), th.todo));
    }

    test
}
