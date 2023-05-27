use crate::db::secret;

pub fn secret(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: secret [create,delete,list,read,update] <vault name> <secret name>");
        return;
    }

    let secret = secret::Secret {
        name: args[1].to_string(),
        value: args[2].to_string(),
        vault: args[0].to_string(),
    };

    match args[0] {
        "create" => secret.create(&args[1..]),
        "delete" => secret.delete(&args[1..]),
        "list" => secret.list(),
        "update" => secret.update(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl secret::Secret {
    fn create(&self, args: &[&str]) {
        println!("Creating secret: {:?}", args[1]);

        let res = self.db_create(&args[0], args[1]);

        match res {
            Ok(_) => println!("Secret created"),
            Err(e) => println!("Error creating secret: {}", e),
        }
    }

    fn delete(&self, args: &[&str]) {
        println!("Secret delete: {:?}", args);

        let res = self.db_delete(args);

        match res {
            Ok(_) => println!("Secret deleted"),
            Err(e) => println!("Error deleting secret: {}", e),
        }
    }

    fn list(&self) {
        println!("Listing secrets");

        let res = self.db_list();

        match res {
            Ok(secrets) => {
                for secret in secrets {
                    println!("Secret: {}", secret.1);
                }
            }
            Err(e) => println!("Error listing secrets: {}", e),
        }
    }

    fn update(&self, args: &[&str]) {
        println!("Secret update: {:?}", args);

        let res = self.db_update(args);

        match res {
            Ok(_) => println!("Secret updated"),
            Err(e) => println!("Error updating secret: {}", e),
        }
    }
}
