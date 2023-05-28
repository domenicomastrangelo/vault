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

    pub fn db_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut values: Vec<String> = Vec::new();
        values.push("test".to_string());

        Ok(values)
    }

    pub fn db_update(&self) -> Result<u64, Box<dyn Error>> {
        Ok(0)
    }

    pub fn db_delete(&self) -> Result<u64, Box<dyn Error>> {
        Ok(0)
    }
}
