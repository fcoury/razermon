use crate::database::Conn;

pub fn get(key: &str) -> anyhow::Result<Option<String>> {
    let db = Conn::new()?;
    let mut stmt = db
        .conn
        .prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query(&[key])?;
    if let Some(row) = rows.next()? {
        return Ok(Some(row.get(0)?));
    }
    Ok(None)
}

pub fn set(key: &str, value: &str) -> anyhow::Result<()> {
    let db = Conn::new()?;
    db.conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
            (&key, &value),
        )?;
    Ok(())
}
