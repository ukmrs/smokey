use crate::colorscheme;
use colorscheme::Theme;

use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use tui::text::Span;

#[allow(dead_code)]
pub fn mock<'a>(th: &'a Theme) -> Vec<Span<'a>> {
    let msg = fs::read_to_string("./typedbg/typetest").expect("coudn't load test");
    let mut test = Vec::with_capacity(msg.len() * 2);

    for c in msg.chars() {
        test.push(Span::styled("", th.wrong));
        test.push(Span::styled(c.to_string(), th.todo));
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
