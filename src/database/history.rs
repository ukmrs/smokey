use crate::settings;
use rusqlite::{self, Connection};
use std::io::{self, BufWriter, Write};

const CLI_HISTORY_STATEMENT: &str = r#"SELECT 

wpm,
acc,
test.test_name,
mods,
correct_chars,
mistakes,
datetime(date, 'unixepoch', 'localtime'),
length,
word_pool

FROM run
INNER JOIN test ON test.test_id = run.test_id
ORDER BY run_id desc
LIMIT ?;
"#;

fn decode(bitflag: u8) -> String {
    let mut result = String::from(" ");

    for i in 0..3 {
        if bitflag >> i & 1 == 1 {
            match i {
                0 => result.push_str(settings::PUNCTUATION_SHORTHAND),
                1 => result.push_str(settings::NUMBERS_SHORTHAND),
                2 => result.push_str(settings::SYMBOLS_SHORTHAND),
                _ => unreachable!(),
            }
        };
    }

    result
}

pub struct History {
    som: Vec<EntryCell>,
    justing: JustingInfo,
}

impl History {
    pub fn print(&self) {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut buff = BufWriter::new(lock);

        let clen = format!("{}", self.justing.correct).len();

        writeln!(
            buff,
            "{:6}|{:6}|{:clen$}|{:3}",
            "wpm",
            "acc",
            "c",
            "mis",
            clen = clen
        )
        .expect("couldn't write to stdout");
        for a in &self.som {
            writeln!(
                buff,
                "{:6}|{:6}|{:<w$}|{:<3}|{:width$}|{}",
                format!("{:.2}", a.wpm),
                format!("{:.2}", a.acc),
                a.correct,
                a.mis,
                a.name,
                a.date,
                width = self.justing.name_len,
                w = clen,
            )
            .expect("oof: couldn't write to stdout")
        }
        buff.flush().expect("oof: couldn't flush to stdout");
    }
}

#[derive(Debug)]
struct EntryCell {
    wpm: f64,
    acc: f64,
    correct: usize,
    mis: usize,
    name: String,
    date: String,
}

#[derive(Debug, Default)]
struct JustingInfo {
    name_len: usize,
    correct: usize,
}

impl JustingInfo {
    fn update(&mut self, entry_cell: &EntryCell) {
        if entry_cell.name.len() > self.name_len {
            self.name_len = entry_cell.name.len()
        }
        self.correct = std::cmp::max(self.correct, entry_cell.correct)
    }
}

pub fn get_history(conn: &Connection, limit: usize) -> Result<History, rusqlite::Error> {
    let mut stmt = conn.prepare(CLI_HISTORY_STATEMENT)?;
    let mut justing = JustingInfo::default();

    let rows = stmt.query_map([limit], |row| {
        let name: String;
        let word_pool: usize = row.get(8)?;

        if word_pool == 0 {
            name = row.get(2)?;
        } else {
            let raw_name: String = row.get(2)?;
            let length: usize = row.get(7)?;
            name = format!(
                "{} {}/{}{}",
                raw_name,
                length,
                word_pool,
                decode(row.get(3)?)
            );
        }

        let s = EntryCell {
            wpm: row.get(0)?,
            acc: row.get(1)?,
            correct: row.get(4)?,
            mis: row.get(5)?,
            name,
            date: row.get(6)?,
        };

        justing.update(&s);
        Ok(s)
    })?;

    // It would oof earlier than this unwrap anyway right? xD
    let container: Vec<EntryCell> = rows.map(|x| x.unwrap()).collect();

    Ok(History {
        som: container,
        justing,
    })
}
