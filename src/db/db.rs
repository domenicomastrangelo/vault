use rusqlite::{params, Connection, Result};

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("db.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vault (
            id INTEGER PRIMARY KEY,
            name VARCHAR(255) NOT NULL
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS secret (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (vault_id) REFERENCES vault(id)
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS certificate (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (vault_id) REFERENCES vault(id)
        )",
        params![],
    )?;

    Ok(conn)
}
