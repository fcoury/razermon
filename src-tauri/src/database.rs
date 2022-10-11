use std::fs;

use rusqlite::Connection;

#[derive(Debug)]
pub struct Conn {
    pub conn: Connection,
}

impl Conn {
    fn create_database() -> anyhow::Result<Connection> {
        let db_path = dirs::config_dir().unwrap().join("razermon");
        fs::create_dir_all(&db_path)?;
        let db_file = db_path.join("razermon.db");
        let conn = Connection::open(db_file)?;
        conn.execute("CREATE TABLE IF NOT EXISTS battery (id INTEGER PRIMARY KEY, created_at TEXT DEFAULT CURRENT_TIMESTAMP, device_id INTEGER, percentage INTEGER, charging INTEGER)", ())?;
        conn.execute("CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY, created_at TEXT DEFAULT CURRENT_TIMESTAMP, key TEXT UNIQUE, value TEXT)", ())?;
        Ok(conn)
    }

    pub fn new() -> anyhow::Result<Self> {
        let conn = Self::create_database()?;
        Ok(Self { conn })
    }
}
