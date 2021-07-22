pub mod randorst;
pub mod termprep;
use bytecount;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tui::widgets::ListState;

#[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn get_item(&mut self) -> &mut T {
        &mut self.items[self
            .state
            .selected()
            .expect("should be impossible to call this function without selected item")]
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub fn count_lines_from_path(filepath: impl AsRef<Path>) -> anyhow::Result<usize> {
    let file = File::open(filepath)?;
    count_lines(file)
}

/// Expects a file and returns number of lines
///
/// ```
/// use smokey::utils::count_lines;
/// let file: &[u8] = b"one line\nanother line\nand even more lines\n";
/// assert_eq!(count_lines(file).unwrap(), 3);
///
/// ```
///
/// based on cw -l functionality from
/// https://github.com/Freaky/cw
/// a fast wc clone in Rust
/// great stuff I use it as well
pub fn count_lines<R: io::Read>(file: R) -> anyhow::Result<usize> {
    let mut reader = io::BufReader::new(file);
    let mut count: usize = 0;

    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            break;
        }
        count += bytecount::count(&buffer, b'\n');
        let buflen = buffer.len();
        reader.consume(buflen);
    }
    Ok(count)
}
