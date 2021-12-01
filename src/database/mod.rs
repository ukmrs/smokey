pub mod history;
pub mod init;
use crate::settings::{TestMod, TestVariant, TypingTestConfig, BITFLAG_MODS};
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
        let sum = &ttc.test_summary;

        let (len, mods, pool): (usize, u8, usize);

        match ttc.variant {
            TestVariant::Script => {
                len = 0;
                mods = 0;
                pool = 0;
            }
            TestVariant::Standard => {
                len = ttc.length;
                mods = encode_test_mod_bitflag(&ttc.mods);
                pool = ttc.word_pool;
            }
        }

        self.conn.execute(
            "INSERT INTO run (date, test_id, length, mods, word_pool, correct_chars, mistakes, wpm, acc)
            VALUES ((SELECT strftime('%s', 'now')), ?, ?, ?, ?, ?, ?, ?, ?);",
            params![test_id, len, mods, pool,
            sum.correct_chars, sum.mistakes, sum.wpm, sum.acc],
            )
            .expect("inserting into run");
    }

    pub fn print_history(&self, limit: usize) {
        history::get_history(&self.conn, limit)
            .expect("could")
            .print();
    }
}

pub fn get_max_wpm_script(conn: &Connection, script_name: &str) -> Option<f64> {
    conn.query_row(
        "SELECT max(wpm) FROM run WHERE
        test_id = (select test_id FROM test WHERE test_name = ?)",
        params![&script_name,],
        |row| row.get(0),
    )
    .ok()
}

pub fn get_max_wpm(conn: &Connection, ttc: &TypingTestConfig) -> Option<f64> {
    conn.query_row(
        "SELECT max(wpm) FROM run WHERE
        test_id = (select test_id FROM test WHERE test_name = ?)
        AND length = ?
        AND word_pool = ?
        AND mods = ?",
        params![
            &ttc.name,
            ttc.length,
            ttc.word_pool,
            encode_test_mod_bitflag(&ttc.mods),
        ],
        |row| row.get(0),
    )
    .ok()
}

pub fn get_test_id(conn: &Connection, test_name: &str) -> Result<usize, rusqlite::Error> {
    conn.query_row(
        "select test_id from test where test_name = ?",
        [test_name],
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

pub fn encode_test_mod_bitflag(test_mods: &HashSet<TestMod>) -> u8 {
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
    use crate::settings::TypingTestConfig;
    use rusqlite::Connection;
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

    #[test]
    fn test_get_max_wpm() {
        let mut conn = RunHistoryDatbase {
            conn: Connection::open_in_memory().unwrap(),
        };
        init::init_db(&mut conn.conn).unwrap();
        let mut ttc = TypingTestConfig::default();

        let no_records_wpm = get_max_wpm(&conn.conn, &ttc);
        assert!(no_records_wpm.is_none());

        let wpms: [f64; 5] = [69., 152., 51., 72., 150.];
        for wpm in wpms {
            ttc.test_summary.wpm = wpm;
            conn.save(&ttc);
        }

        let max_wpm = get_max_wpm(&conn.conn, &ttc).unwrap();
        let should_be_max_wpm = 152.;

        assert!(max_wpm - f64::EPSILON <= should_be_max_wpm);
        assert!(max_wpm + f64::EPSILON >= should_be_max_wpm);
    }
}
