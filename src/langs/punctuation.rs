use crate::settings::TestMod;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashSet;

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
    // in between words duh,
    InBetweener(InnerWord),
    Nil,
}

/// Creates a WeightedIndex of punctuation whis allows
/// to modify text in certain ways
pub struct PunctuationInsertFrequency {
    weighted_index: WeightedIndex<u16>,
    symbols: Vec<Punctuation>,
}

impl PunctuationInsertFrequency {
    fn from_protoplast(protoplast: Vec<(Punctuation, u16)>) -> Self {
        let mut weighted_index: Vec<u16> = Vec::with_capacity(protoplast.len());
        let mut symbols: Vec<Punctuation> = Vec::with_capacity(protoplast.len());
        for (p, w) in protoplast.into_iter() {
            weighted_index.push(w);
            symbols.push(p);
        }

        let weighted_index = WeightedIndex::new(weighted_index).unwrap();
        Self {
            weighted_index,
            symbols,
        }
    }

    pub fn from_test_mods(test_mods: &HashSet<TestMod>) -> Self {
        let mut protoplast: Vec<(Punctuation, u16)> = vec![(Punctuation::Nil, 750)];
        for test_mod in test_mods {
            match test_mod {
                TestMod::Punctuation => {
                    let mut we: Vec<(Punctuation, u16)> = vec![
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
                    ];
                    protoplast.append(&mut we);
                }
                TestMod::Numbers => {
                    protoplast.push((Punctuation::InBetweener(InnerWord::Number), 150));
                }
                // TODO symbols may be better served with their own punctuation table
                TestMod::Symbols => {
                    protoplast.push((Punctuation::InBetweener(InnerWord::Symbol), 71));
                }
                TestMod::Capitalization => {}
            }
        }
        Self::from_protoplast(protoplast)
    }
}

impl PunctuationInsertFrequency {
    pub fn choose(&self, rng: &mut ThreadRng) -> Punctuation {
        self.symbols[self.weighted_index.sample(rng)]
    }
}
