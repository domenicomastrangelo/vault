use crate::db::vault;

pub fn vault(args: &mut [&str]) {
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

    let mut vault = vault::Vault {
        name: args[1].to_string(),
        id: 0,
    };

    match args[0] {
        "create" => vault.create(&mut args[1..]),
        "delete" => vault.delete(&args[1..]),
        "update" => vault.update(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl vault::Vault {
    fn list(&self) {
        println!("Listing vaults");
        let res = self.db_list();

        match res {
            Ok(vaults) => {
                for vault in vaults {
                    println!("Vault: {}", vault);
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

    fn update(&self, args: &[&str]) {
        println!("Updating vault {}", args[0]);
        let res = self.db_update(args);

        match res {
            Ok(_) => println!("Vault updated"),
            Err(e) => println!("Error updating vault: {}", e),
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
