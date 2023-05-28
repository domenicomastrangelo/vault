use std::error::Error;

use super::db::connect;

use rusqlite::params;

use std::io::Error as IoError;

pub struct Certificate {
    pub vault_name: String,
    pub name: String,
    pub cert_type: String,
    pub data: String,
}

impl Certificate {
    pub fn db_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let conn = connect()?;

        let mut values: Vec<String> = Vec::new();

        let mut stmt = conn.prepare("SELECT name FROM certificates")?;

        let rows = stmt.query_map(params![], |row| Ok(row.get(0)?))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }

    pub fn db_create(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute(
            "INSERT INTO certificates('vault_id', 'name', 'data', 'cert_type') VALUES((SELECT id FROM vaults WHERE name = ? LIMIT 1), ?, ?, ?)",
            params![self.vault_name, self.name, self.data, self.cert_type],
        );

        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    let ee = IoError::new(std::io::ErrorKind::Other, "Certificate already exists");
                    return Err(Box::new(ee));
                } else {
                    let ee = IoError::new(
                        std::io::ErrorKind::Other,
                        "There's been an error creating the certificate",
                    );
                    return Err(Box::new(ee));
                }
            }
        }
    }

    pub fn db_update(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute(
            "UPDATE certificates SET data = ? WHERE name = ?",
            params![self.data, self.name],
        );

        match res {
            Ok(size) => Ok(size),
            Err(_) => {
                let ee = IoError::new(
                    std::io::ErrorKind::Other,
                    "There's been an error updating the certificate",
                );
                return Err(Box::new(ee));
            }
        }
    }

    pub fn db_delete(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute(
            "DELETE FROM certificates WHERE name = ?",
            params![self.name],
        );

        match res {
            Ok(size) => Ok(size),
            Err(_) => {
                let ee = IoError::new(
                    std::io::ErrorKind::Other,
                    "There's been an error deleting the certificate",
                );
                return Err(Box::new(ee));
            }
        }
    }

    pub fn db_get(&self) -> Result<String, Box<dyn Error>> {
        let conn = connect()?;

        let data = conn.query_row(
            "SELECT data FROM certificates WHERE name = ?",
            params![self.name],
            |row| Ok(row.get(0)?),
        )?;

        Ok(data)
    }
}
