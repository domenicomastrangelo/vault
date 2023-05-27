use crate::common::record_trait::{RecordTrait, RecordDatabaseTrait};
use crate::db::vault;

pub fn vault(args: &[&str]) {
    if args.len() < 2 {
        if args[0] == "list" {
            let vault = vault::Vault {
                name: "".to_string(),
                id: 0,
            };

            vault.list();

            return;
        } else {
            println!("Usage: vault [create,delete,list,read,update] <vault name>");
            return;
        }
    }

    let vault = vault::Vault {
        name: args[1].to_string(),
        id: 0,
    };

    match args[0] {
        "create" => vault.create(&args[1..]),
        "delete" => vault.delete(&args[1..]),
        "read" => vault.read(&args[1..]),
        "update" => vault.update(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl RecordTrait for vault::Vault {
    fn list(&self) {
        println!("Listing vaults");
        let res = self.db_list();

        match res {
            Ok(vaults) => {
                for vault in vaults {
                    println!("Vault: {}", vault.1);
                }
            },
            Err(e) => println!("Error listing vaults: {}", e),
        }
    }

    fn create(&self, args: &[&str]) {
        println!("Creating vault {:?}", args[0]);
        let res = self.db_create(args[0]);

        match res {
            Ok(_) => println!("Vault created"),
            Err(e) => println!("Error creating vault: {}", e),
        }
    }

    fn read(&self, args: &[&str]) {
        println!("Vault read {:?}", args);
    }

    fn update(&self, args: &[&str]) {
        println!("Vault update {:?}", args);
    }

    fn delete(&self, args: &[&str]) {
        println!("Vault delete {:?}", args);
    }
}

