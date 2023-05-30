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

    pub fn db_get(&self) -> Result<String, Box<dyn Error>> {
        let conn = connect()?;

        let mut stmt = conn.prepare(
            "SELECT value FROM secrets WHERE name = ? AND vault_id = (SELECT id FROM vaults WHERE name = ? LIMIT 1) LIMIT 1",
        )?;

        // let row = stmt.query_map(params![self.name, self.vault], |row| Ok(row.get(0)?))?;

        let value = stmt.query_row(params![self.name, self.vault], |row| row.get(0));

        Ok(value?)
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
        setup_vault(vault_name.to_string());

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        res.unwrap_or_else(|e| panic!("Failed to create secret: {}", e));

        let res = secret.db_list();

        let found_string = res.unwrap_or_else(|e| panic!("Failed to list secrets: {}", e));

        destroy_vault(vault_name.to_string());

        assert!(found_string.contains(&secret_name.to_string()));
    }

    #[test]
    fn test_db_create() {
        let vault_name = "test_secret_db_create";
        let secret_name = "test_secret_db_create";
        setup_vault(vault_name.to_string());

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        let secret_created = res.unwrap_or_else(|e| panic!("Failed to create secret: {}", e));

        destroy_vault(vault_name.to_string());

        assert_eq!(secret_created, 1);
    }

    #[test]
    fn test_db_delete() {
        let vault_name = "test_secret_db_delete";
        let secret_name = "test_secret_db_delete";

        setup_vault(vault_name.to_string());

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        res.unwrap_or_else(|e| panic!("Failed to create secret: {}", e));

        let res = secret.db_delete();

        let secret_deleted = res.unwrap_or_else(|e| panic!("Failed to delete secret: {}", e));

        destroy_vault(vault_name.to_string());

        assert_eq!(secret_deleted, 1);
    }

    #[test]
    fn test_db_get() {
        let vault_name = "test_secret_db_get";
        let secret_name = "test_secret_db_get";

        setup_vault(vault_name.to_string());

        let secret = Secret {
            name: secret_name.to_string(),
            value: "test".to_string(),
            vault: vault_name.to_string(),
        };

        let res = secret.db_create();

        res.unwrap_or_else(|e| panic!("Failed to create secret: {}", e));

        let res = secret.db_get();

        let secret_from_db = res.unwrap_or_else(|e| panic!("Failed to get secret: {}", e));

        destroy_vault(vault_name.to_string());

        assert_eq!(secret_from_db, "test".to_string())
    }
}
