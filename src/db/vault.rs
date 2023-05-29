use rusqlite::params;

use super::db::connect;

use std::error::Error;
use std::io::Error as IoError;

pub struct Vault {
    pub name: String,
    pub id: u64,
}

impl Vault {
    pub fn db_list(&self) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
        let conn = connect()?;
        let mut values = Vec::new();

        let mut stmt = conn.prepare("SELECT id, name FROM vaults")?;

        let rows = stmt.query_map(params![], |row| Ok((row.get(0)?, row.get(1)?)))?;

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
