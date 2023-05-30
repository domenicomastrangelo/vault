use rusqlite::{params, Result};

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use std::error::Error;

pub fn connect() -> Result<PooledConnection<SqliteConnectionManager>, Box<dyn Error>> {
    let manager = SqliteConnectionManager::file("db.sqlite");

    let pool = Pool::new(manager)?;

    let conn = pool.get()?;

    conn.execute("PRAGMA foreign_keys=1", params![])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vaults (
            id INTEGER PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            UNIQUE(name)
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT 1,
            FOREIGN KEY (vault_id) REFERENCES vaults(id) ON DELETE CASCADE,
            UNIQUE(vault_id, name)
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS certificates (
            id INTEGER PRIMARY KEY,
            vault_id INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            data TEXT NOT NULL,
            cert_type VARCHAR(255) NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT 1,
            FOREIGN KEY (vault_id) REFERENCES vaults(id) ON DELETE CASCADE,
            UNIQUE(vault_id, name)
        )",
        params![],
    )?;

    Ok(conn)
}
