use crate::db::certificate::Certificate;

use crate::common::utils::check_args;

pub fn certificate(args: &[&str]) {
    if args.len() < 2 {
        println!(
            "Usage: certificate [create,delete,list,get,update] <vault_name> <rsa, ecdsa> <certificate name>"
        );
        return;
    }

    let mut certificate = Certificate {
        vault_name: "".to_string(),
        name: "".to_string(),
        cert_type: "".to_string(),
        data: "".to_string(),
        enabled: true,
    };

    match args[0] {
        "create" => certificate.create(&args[1..]),
        "delete" => certificate.delete(&args[1..]),
        "list" => certificate.list(&args[1..]),
        "get" => certificate.get(&args[1..]),
        "disable" => certificate.disable(&args[1..]),
        "enable" => certificate.enable(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl Certificate {
    fn list(&mut self, args: &[&str]) {
        check_args(1, args);

        self.vault_name = args[0].to_string();

        println!("Listing certificates");

        let values = self.db_list();

        match values {
            Ok(value) => {
                value.iter().for_each(|v| println!("{}", v));
            }
            Err(e) => println!("{}", e),
        }
    }

    fn create<'a>(&'a mut self, args: &[&str]) {
        check_args(3, args);

        self.vault_name = args[0].to_string();
        self.cert_type = args[1].to_string();
        self.name = args[2].to_string();

        println!("Creating certificate: {}", self.name);

        println!("Insert the certificate data here:");

        let mut value = String::new();
        std::io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        self.data = value.to_owned();

        let res = self.db_create();

        match res {
            Ok(_) => println!("Certificate created"),
            Err(e) => println!("Error creating certificate: {}", e),
        }
    }

    fn delete(&mut self, args: &[&str]) {
        check_args(1, args);

        self.vault_name = args[0].to_string();
        self.name = args[1].to_string();

        println!("Deleting certificate: {:?}", self.name);

        let res = self.db_delete();

        match res {
            Ok(_) => println!("Certificate deleted"),
            Err(e) => println!("Error deleting certificate: {}", e),
        }
    }

    fn get(&mut self, args: &[&str]) {
        check_args(2, args);

        self.vault_name = args[0].to_string();
        self.name = args[1].to_string();

        println!("Getting certificate: {:?}", self.name);

        let res = self.db_get();

        match res {
            Ok(value) => {
                println!("Enabled: {}", value.0);
                println!("{}", value.1);
            }
            Err(e) => println!("Error getting certificate: {}", e),
        }
    }

    fn disable(&mut self, args: &[&str]) {
        check_args(2, args);

        self.vault_name = args[0].to_string();
        self.name = args[1].to_string();

        println!("Disabling certificate: {:?}", self.name);

        let res = self.db_disable();

        match res {
            Ok(_) => println!("Certificate disabled"),
            Err(e) => println!("Error disabling certificate: {}", e),
        }
    }

    fn enable(&mut self, args: &[&str]) {
        check_args(2, args);

        self.vault_name = args[0].to_string();
        self.name = args[1].to_string();

        println!("Enabling certificate: {:?}", self.name);

        let res = self.db_enable();

        match res {
            Ok(_) => println!("Certificate enabled"),
            Err(e) => println!("Error enabling certificate: {}", e),
        }
    }
}
