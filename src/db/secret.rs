use std::error::Error;

use rusqlite::params;

use crate::db::db::connect;

use std::io::Error as IoError;

pub struct Secret {
    pub name: String,
    pub value: String,
    pub vault: String,
}

impl Secret {
    pub fn db_list(&self) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
        let conn = connect()?;
        let mut values = Vec::new();

        let mut stmt = conn.prepare(
            "SELECT id, name FROM secrets WHERE vault_id = (SELECT id from vaults where name = ? LIMIT 1)",
        )?;

        let rows = stmt.query_map(params![self.vault], |row| Ok((row.get(0)?, row.get(1)?)))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }

    pub fn db_create(&self) -> Result<usize, Box<dyn Error>> {
        let mut value = String::new();

        println!("Enter secret value: ");

        std::io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let conn = connect()?;

        let res = conn.execute(
            "INSERT INTO secrets('name', 'value', 'vault_id') values(?, ?, (SELECT id FROM vaults WHERE name = ? LIMIT 1))",
            params![self.name, value, self.vault],
        );

        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    let ee = IoError::new(std::io::ErrorKind::Other, "Secret already exists");
                    return Err(Box::new(ee));
                } else {
                    let ee = IoError::new(
                        std::io::ErrorKind::Other,
                        "Failed to create secret, check vault name",
                    );
                    return Err(Box::new(ee));
                }
            }
        }
    }

    pub fn db_delete(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute("DELETE FROM secrets where name = ?", params![self.name]);

        match res {
            Ok(size) if size > 0 => Ok(size),
            Ok(_) => Err(Box::new(IoError::new(
                std::io::ErrorKind::Other,
                "Secret was not deleted, double check the name",
            ))),
            Err(_) => Err(Box::new(IoError::new(
                std::io::ErrorKind::Other,
                "There's been an error deleting the secret",
            ))),
        }
    }

    pub fn db_get(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let conn = connect()?;

        let mut stmt = conn.prepare(
            "SELECT value FROM secrets WHERE name = ? AND vault_id = (SELECT id FROM vaults WHERE name = ? LIMIT 1)",
        )?;

        let mut values = Vec::new();

        let rows = stmt.query_map(params![self.name, self.vault], |row| Ok(row.get(0)?))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }
}