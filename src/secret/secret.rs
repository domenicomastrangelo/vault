use crate::db::secret;

pub fn secret(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: secret [create,delete,list,read,update] <vault name> <secret name>");
        return;
    }

    let mut secret = secret::Secret {
        name: "".to_owned(),
        value: "".to_owned(),
        vault: args[1].to_string(),
    };

    if args.len() >= 3 {
        secret.name = args[2].to_string();
    }

    match args[0] {
        "create" => secret.create(&args[1..]),
        "delete" => secret.delete(&args[1..]),
        "list" => secret.list(),
        "get" => secret.get(),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl secret::Secret {
    fn create(&self, args: &[&str]) {
        println!("Creating secret: {:?}", args[1]);

        let res = self.db_create();

        match res {
            Ok(_) => println!("Secret created"),
            Err(e) => println!("Error creating secret: {}", e),
        }
    }

    fn delete(&self, args: &[&str]) {
        println!("Secret delete: {:?}", args);

        let res = self.db_delete();

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

    fn get(&self) {
        println!("Reading secret {}", self.name);

        let res = self.db_get();

        match res {
            Ok(data) => data.iter().for_each(|d| println!("{}", d)),
            Err(e) => println!("Error reading secret {}", e),
        }
    }
}
