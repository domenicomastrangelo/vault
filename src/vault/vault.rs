use std::error::Error;

use crate::db::vault;

pub fn vault(args: &mut [&str]) {
    let mut vault = parse_args(args);

    match vault {
        Ok(ref mut vault) => match args[0] {
            "create" => vault.create(&args[1..]),
            "delete" => vault.delete(&args[1..]),
            "list" => vault.list(),
            _ => println!("Unknown command: {}", args[0]),
        },
        Err(e) => println!("{}", e),
    }
}

fn parse_args(args: &[&str]) -> Result<vault::Vault, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Usage: vault [create,delete,list] <vault_name>".into());
    }

    let vault = vault::Vault {
        name: args[1].to_string(),
    };

    Ok(vault)
}

impl vault::Vault {
    fn list(&self) {
        println!("Listing vaults");
        let res = self.db_list();

        match res {
            Ok(vaults) => {
                for vault in vaults {
                    println!("{}", vault);
                }
            }
            Err(e) => println!("Error listing vaults: {}", e),
        }
    }

    fn create(&mut self, args: &[&str]) {
        println!("Creating vault {}", args[0]);

        self.name = args[0].to_string();

        let res = self.db_create();

        match res {
            Ok(_) => println!("Vault created"),
            Err(e) => println!("Error creating vault: {}", e),
        }
    }

    fn delete(&mut self, args: &[&str]) {
        self.name = args[0].to_string();

        println!("Deleting vault: {}", self.name);

        println!("This will delete all secrets and certiticates in the vault");

        println!("Are you sure? (y/N)");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() != "y" {
            println!("Aborting");
            return;
        }

        match self.db_delete() {
            Ok(_) => println!("Vaults deleted"),
            Err(e) => println!("Error deleting vaults: {}", e),
        }
    }
}
