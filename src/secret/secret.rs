use crate::db::secret;

pub fn secret(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: secret [create,delete,list,read,update,disable,enable] <vault name> <secret name>");
        return;
    }

    let mut secret = secret::Secret {
        name: "".to_owned(),
        value: "".to_owned(),
        vault: args[1].to_string(),
        enabled: true,
    };

    if args.len() >= 3 {
        secret.name = args[2].to_string();
    }

    match args[0] {
        "create" => secret.create(&args[1..]),
        "delete" => secret.delete(&args[1..]),
        "list" => secret.list(),
        "get" => secret.get(),
        "disable" => secret.disable(),
        "enable" => secret.enable(),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl secret::Secret {
    fn create(&mut self, args: &[&str]) {
        println!("Creating secret: {:?}", args[1]);

        let mut value = String::new();

        println!("Enter secret value: ");

        std::io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        self.value = value.trim().to_string();

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
                    println!("Secret: {}", secret);
                }
            }
            Err(e) => println!("Error listing secrets: {}", e),
        }
    }

    fn get(&self) {
        println!("Reading secret {}", self.name);

        let res = self.db_get();

        match res {
            Ok(data) => {
                println!("Enabled: {}", data.1);
                println!("Secret: {}", data.0)
            }
            Err(e) => println!("Error reading secret {}", e),
        }
    }

    fn disable(&self) {
        println!("Disabling secret {}", self.name);

        let res = self.db_disable();

        match res {
            Ok(_) => println!("Secret disabled"),
            Err(e) => println!("Error disabling secret {}", e),
        }
    }

    fn enable(&self) {
        println!("Enabling secret {}", self.name);

        let res = self.db_enable();

        match res {
            Ok(_) => println!("Secret enabled"),
            Err(e) => println!("Error enabling secret {}", e),
        }
    }
}
