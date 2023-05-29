use rusqlite::params;

use super::db::connect;

use std::error::Error;
use std::io::Error as IoError;

pub struct Vault {
    pub name: String,
    pub id: u64,
}

impl Vault {
    pub fn db_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let conn = connect()?;
        let mut values = Vec::new();

        let mut stmt = conn.prepare("SELECT name FROM vaults")?;

        let rows = stmt.query_map(params![], |row| Ok(row.get(0)?))?;

        for row in rows {
            values.push(row?);
        }

        Ok(values)
    }

    pub fn db_create(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute("INSERT INTO vaults('name') values(?)", params![self.name]);

        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    let ee = IoError::new(std::io::ErrorKind::Other, "Vault already exists");
                    return Err(Box::new(ee));
                } else {
                    return Err(Box::new(e));
                }
            }
        }
    }

    pub fn db_update(&self, args: &[&str]) -> Result<usize, Box<dyn Error>> {
        if args.len() != 2 {
            let ee = IoError::new(
                std::io::ErrorKind::Other,
                "Usage: vault update <old_name> <new_name>",
            );
            return Err(Box::new(ee));
        }

        let conn = connect()?;

        let res = conn.execute(
            "UPDATE vaults SET name = ? WHERE name = ?",
            params![args[1], args[0]],
        );

        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    let ee = IoError::new(
                        std::io::ErrorKind::Other,
                        format!("A vault with the name {} already exists", args[1]),
                    );
                    return Err(Box::new(ee));
                } else {
                    return Err(Box::new(e));
                }
            }
        }
    }

    pub fn db_delete(&self) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute("DELETE FROM vaults WHERE name = ?", params![self.name])?;

        if res == 0 {
            let ee = IoError::new(std::io::ErrorKind::Other, "No vaults deleted");
            return Err(Box::new(ee));
        } else {
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::vault::Vault,
        test_utils::test_utils::{destroy_vault, setup_vault},
    };

    #[test]
    fn test_db_list() {
        let vault_name = "test_vault_db_list";
        let res = setup_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }

        let v = Vault {
            name: vault_name.to_string(),
            id: 0,
        };

        let res = v.db_list();

        match res {
            Ok(r) => {
                let found_string = r.iter().find(|&x| x == &vault_name.to_string());

                assert!(found_string.is_some());
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        let res = destroy_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_db_create() {
        let vault_name = "test_vault_db_create";
        let v = Vault {
            name: vault_name.to_string(),
            id: 0,
        };

        let res = v.db_create();

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }

        let res = destroy_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_db_update() {
        let vault_name = "test_vault_db_update";
        let vault_name_new = "test_vault_db_update_new";
        let res = setup_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }

        let v = Vault {
            name: vault_name.to_string(),
            id: 0,
        };

        let res = v.db_update(&[vault_name, vault_name_new]);

        match res {
            // r in this case is the row ID, not the number of rows affected
            Ok(r) => assert!(r > 0),
            Err(e) => {
                println!("{}", e);
            }
        }

        let res = v.db_list();

        match res {
            Ok(r) => {
                let found_string = r.iter().find(|&x| x == &vault_name_new.to_string());

                assert!(found_string.is_some());
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        let res = destroy_vault(vault_name_new.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_db_delete() {
        let vault_name = "test_vault_db_delete";
        let res = setup_vault(vault_name.to_string());

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }

        let v = Vault {
            name: vault_name.to_string(),
            id: 0,
        };

        let res = v.db_delete();

        match res {
            Ok(r) => assert_eq!(r, 1),
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
