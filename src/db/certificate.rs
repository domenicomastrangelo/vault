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

#[cfg(test)]
mod tests {
    use crate::db::certificate::Certificate;
    use crate::test_utils::test_utils::{destroy_vault, setup_vault};

    #[test]
    fn test_db_create() {
        let vault_name = "test_certificate_db_create";
        let certificate_name = "test_db_create";
        setup_vault(vault_name.to_string());

        let cert = Certificate {
            vault_name: vault_name.to_string(),
            name: certificate_name.to_string(),
            cert_type: "rsa".to_string(),
            data: "test".to_string(),
        };

        let res = cert.db_create();

        let certificate_created =
            res.unwrap_or_else(|e| panic!("Failed to create certificate: {}", e));

        destroy_vault(vault_name.to_string());

        assert_eq!(certificate_created, 1);
    }

    #[test]
    fn test_db_delete() {
        let vault_name = "test_certificate_db_delete";
        let certificate_name = "test_db_delete";
        setup_vault(vault_name.to_string());

        let cert = Certificate {
            vault_name: vault_name.to_string(),
            name: certificate_name.to_string(),
            cert_type: "rsa".to_string(),
            data: "test".to_string(),
        };

        let res = cert.db_create();

        res.unwrap_or_else(|e| panic!("Failed to create certificate: {}", e));

        let res = cert.db_delete();

        let certificate_deleted =
            res.unwrap_or_else(|e| panic!("Failed to delete certificate: {}", e));

        destroy_vault(vault_name.to_string());

        assert_eq!(certificate_deleted, 1);
    }

    #[test]
    fn test_db_list() {
        let vault_name = "test_certificate_db_list";
        let certificate_name = "test_db_list";
        setup_vault(vault_name.to_string());

        let cert = Certificate {
            vault_name: vault_name.to_string(),
            name: certificate_name.to_string(),
            cert_type: "rsa".to_string(),
            data: "test".to_string(),
        };

        let res = cert.db_create();

        res.unwrap_or_else(|e| panic!("Failed to create certificate: {}", e));

        let res = cert.db_list();

        let list = res.unwrap_or_else(|e| panic!("Failed to list certificates: {}", e));

        let found_string = list.iter().find(|&x| x == &certificate_name.to_string());

        destroy_vault(vault_name.to_string());

        assert!(found_string.is_some());
    }
}
