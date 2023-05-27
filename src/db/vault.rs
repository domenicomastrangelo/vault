use rusqlite::params;

use crate::common::record_trait::RecordDatabaseTrait;

use super::db::connect;

use std::error::Error;
use std::io::Error as IoError;

pub struct Vault {
    pub name: String,
    pub id: u64,
}

impl RecordDatabaseTrait for Vault {
    fn db_list(&self, arg: &str) -> Result<usize, Box<dyn Error>> {
        println!("Vault list {:?}", arg);

        Ok(0)
    }

    fn db_create(&self, arg: &str) -> Result<usize, Box<dyn Error>> {
        let conn = connect()?;

        let res = conn.execute("INSERT INTO vaults('name') values(?)", params![arg]);

        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    let ee = IoError::new(std::io::ErrorKind::Other, "Vault already exists");
                    return Err(Box::new(ee))
                } else {
                    return Err(Box::new(e))
                }
            }
        }
    }

    fn db_read(&self, arg: &str) -> Result<usize, Box<dyn Error>> {
        println!("Vault read {:?}", arg);

        Ok(0)
    }

    fn db_update(&self, arg: &str) -> Result<usize, Box<dyn Error>> {
        println!("Vault update {:?}", arg);

        Ok(0)
    }

    fn db_delete(&self, arg: &str) -> Result<usize, Box<dyn Error>> {
        println!("Vault delete {:?}", arg);

        Ok(0)
    }
}
