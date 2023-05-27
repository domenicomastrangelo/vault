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

        let mut stmt = conn.prepare("SELECT id, name FROM secrets WHERE vault_id = (SELECT id from vaults where name = ?)")?;

        println!("Vault: {}", self.vault);
        let rows = stmt.query_map(params![self.vault], |row| Ok((row.get(0)?, row.get(1)?)))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }

    pub fn db_create(&self, vault_name: &str, secret_name: &str) -> Result<usize, Box<dyn Error>> {
        let mut value = String::new();

        println!("Enter secret value: ");

        std::io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let conn = connect()?;

        let res = conn.execute(
            "INSERT INTO secrets('name', 'value', 'vault_id') values(?, ?, (SELECT id FROM vaults WHERE name = ? LIMIT 1))",
            params![secret_name, value, vault_name],
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

    pub fn db_update(&self, args: &[&str]) -> Result<usize, Box<dyn Error>> {
        println!("Secret update: {:?}", args);

        Ok(0)
    }

    pub fn db_delete(&self, args: &[&str]) -> Result<usize, Box<dyn Error>> {
        println!("Secret delete: {:?}", args);

        Ok(0)
    }
}
