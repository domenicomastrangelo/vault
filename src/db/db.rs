use rusqlite::{Result, params};

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use std::error::Error;

pub fn connect() -> Result<PooledConnection<SqliteConnectionManager>, Box<dyn Error>> {
    let manager = SqliteConnectionManager::file("db.sqlite");

    let pool = Pool::new(manager)?;

    let conn = pool.get()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vaults (
            id INTEGER PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            UNIQUE(name)
        )",
        params![]
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (vault_id) REFERENCES vault(id),
            UNIQUE(vault_id, name)
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS certificates (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (vault_id) REFERENCES vault(id),
            UNIQUE(vault_id, name)
        )",
        params![],
    )?;

    Ok(conn)
}
