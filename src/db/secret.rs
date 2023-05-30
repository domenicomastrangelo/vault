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
    pub fn db_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let conn = connect()?;
        let mut values = Vec::new();

        let mut stmt = conn.prepare(
            "SELECT name FROM secrets WHERE vault_id = (SELECT id from vaults where name = ? LIMIT 1)",
        )?;

        let rows = stmt.query_map(params![self.vault], |row| Ok(row.get(0)?))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }

    pub fn db_create(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute(
            "INSERT INTO secrets('name', 'value', 'vault_id') values(?, ?, (SELECT id FROM vaults WHERE name = ? LIMIT 1))",
            params![self.name, self.value, self.vault],
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

#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::{destroy_vault, setup_vault};

    use super::Secret;

    #[test]
    fn test_db_list() {
        let vault_name = "test_secret_db_list";
        let secret_name = "test_secret_db_list";
        let res = setup_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to setup vault: {}", e),
        }

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to create secret: {}", e),
        }

        let res = secret.db_list();

        match res {
            Ok(r) => {
                let found_string = r.iter().find(|&x| x == &secret_name.to_string());
                assert!(found_string.is_some());
            }
            Err(e) => panic!("Failed to list secrets: {}", e),
        }

        let res = destroy_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to destroy vault: {}", e),
        }
    }

    #[test]
    fn test_db_create() {
        let vault_name = "test_secret_db_create";
        let secret_name = "test_secret_db_create";
        let res = setup_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to setup vault: {}", e),
        }

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to create secret {}", e),
        }

        let res = destroy_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => panic!("Failed to destroy vault {}", e),
        }
    }
}
