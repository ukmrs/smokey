#![allow(dead_code)]

use rusqlite::Connection;
use rusqlite::Result as SqlResult;

pub fn init_db(conn: &mut Connection) -> SqlResult<()> {
    let tx = conn.transaction()?;

    test_table_init(&tx)?;
    run_table_init(&tx)?;
    mod_table_init(&tx)?;

    tx.commit()?;

    Ok(())
}

fn test_table_init(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test (
    test_id INTEGER PRIMARY KEY,
    test_name TEXT UNIQUE
    );",
        [],
    )?;
    Ok(())
}

fn run_table_init(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS run (
    run_id INTEGER PRIMARY KEY,
    date INTEGER NOT NULL,
    test_id INTEGER NOT NULL,
    length INTEGER NOT NULL,
    word_pool INTEGER NOT NULL,
    correct_chars INTEGER NOT NULL,
    mistakes INTEGER NOT NULL,
    wpm REAL NOT NULL,
    acc REAL NOT NULL,
    mods INTEGER NOT NULL,
    FOREIGN KEY (test_id) REFERENCES test (test_id) ON DELETE CASCADE
    );",
        [],
    )?;
    Ok(())
}

fn script_table_init(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS script (
    script_id INTEGER PRIMARY KEY,
    script_name TEXT UNIQUE
    );",
        [],
    )?;
    Ok(())
}

fn run_script_table_init(conn: Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS run (
    run_id INTEGER PRIMARY KEY,
    date INTEGER NOT NULL,
    script_id INTEGER NOT NULL,
    correct_chars INTEGER NOT NULL,
    mistakes INTEGER NOT NULL,
    wpm REAL NOT NULL,
    acc REAL NOT NULL,
    FOREIGN KEY (script_id) REFERENCES script (script_id)
    );",
        [],
    )?;
    Ok(())
}

fn mod_table_init(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mod (
        mod_id INTEGER PRIMARY KEY,
        mod_name TEXT UNIQUE
        );",
        [],
    )?;
    Ok(())
}

fn enable_foreign_keys(conn: &Connection) {
    conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use rusqlite::{params, Connection, Result as SqlResult};

    fn connect() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        enable_foreign_keys(&conn);
        conn
    }

    #[test]
    fn test_enable_foreign_keys() {
        let conn = Connection::open_in_memory().unwrap();
        enable_foreign_keys(&conn);

        let foreign_keys: i64 = conn
            .query_row("PRAGMA foreign_keys;", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_keys, 1);
    }

    #[test]
    fn test_init_db() {
        let mut conn = connect();
        init_db(&mut conn).expect("db init went oof");

        conn.execute(
            "INSERT INTO test (test_name) VALUES (?) ;",
            params!["english"],
        )
        .expect("inserting into test");

        conn.execute(
            "INSERT INTO run (date, test_id, length, mods, word_pool, correct_chars, mistakes, wpm, acc)
            VALUES ((SELECT strftime('%s', 'now', 'localtime')), (SELECT test_id FROM test WHERE test_name = ? ), ?, ?, ?, ?, ?, ?, ?);",
            params!["english", 100, 0, 5000, 120, 0, 80., 97.5],
        )
        .expect("inserting into run");

        let _date: i64 = conn
            .query_row("SELECT date from run LIMIT 1", [], |row| row.get(0))
            .unwrap();
    }
}
