pub mod init;
use crate::settings::{TestMod, TypingTestConfig, BITFLAG_MODS};
use crate::storage;
use anyhow::Result;
use rusqlite::{params, Connection};
use std::collections::HashSet;

/// A wrapper around rusqlite::Connection
/// with convenient methods to save run results
/// to the underlying database
pub struct RunHistoryDatbase {
    pub conn: Connection,
}

impl Default for RunHistoryDatbase {
    fn default() -> Self {
        Self {
            conn: Connection::open(&*storage::DATABASE).expect("couldn't open db"),
        }
    }
}

impl RunHistoryDatbase {
    pub fn save(&mut self, ttc: &TypingTestConfig) {
        let test_id = get_test_id_or_create(&self.conn, &ttc.name).unwrap();
        let mods = encode_test_mod_bitflag(&ttc.mods);
        let sum = &ttc.test_summary;

        self.conn.execute(
            "INSERT INTO run (date, test_id, length, mods, word_pool, correct_chars, mistakes, wpm, acc)
            VALUES ((SELECT strftime('%s', 'now', 'localtime')), ?, ?, ?, ?, ?, ?, ?, ?);",
            params![test_id, ttc.length, mods, ttc.word_pool,
            sum.correct_chars, sum.mistakes, sum.wpm, sum.acc],
            )
            .expect("inserting into run");
    }
}

pub fn get_test_id(conn: &Connection, test_id: &str) -> Result<usize, rusqlite::Error> {
    conn.query_row(
        "select test_id from test where test_name = ?",
        [test_id],
        |row| row.get(0),
    )
}

pub fn get_test_id_or_create(conn: &Connection, test_id: &str) -> Result<usize> {
    match get_test_id(conn, test_id) {
        Ok(id) => Ok(id),
        Err(_) => {
            conn.execute(
                "INSERT INTO test (test_name) VALUES (?) ;",
                params![test_id],
            )?;
            Ok(conn.last_insert_rowid() as usize)
        }
    }
}

fn encode_test_mod_bitflag(test_mods: &HashSet<TestMod>) -> u8 {
    let mut bitflag: u8 = 0;
    for test_mod in test_mods {
        bitflag += BITFLAG_MODS.get_by_right(test_mod).expect("wrong mod?");
    }
    bitflag
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::TestMod;
    use std::collections::HashSet;

    #[test]
    fn test_encode_mod_bitflag() {
        let mut tm: HashSet<TestMod> = HashSet::new();
        let zero = encode_test_mod_bitflag(&tm);
        assert_eq!(zero, 0_u8);

        tm.insert(TestMod::Punctuation);
        let one = encode_test_mod_bitflag(&tm);
        assert_eq!(one, 1_u8);

        tm.insert(TestMod::Symbols);
        let five = encode_test_mod_bitflag(&tm);
        assert_eq!(five, 5_u8);
    }
}
