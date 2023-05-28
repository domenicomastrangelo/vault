use crate::db::certificate::{self, Certificate};

pub fn certificate(args: &[&str]) {
    println!("{:#?}", args);
    if args.len() < 2 {
        println!(
            "Usage: certificate [create,delete,list,read,update] <vault_name> <rsa, ecdsa> <certificate name>"
        );
        return;
    } else if args.len() < 4 {
        if args[0] == "list" {
            let certificate = Certificate {
                vault_name: args[1].to_string(),
                name: "".to_string(),
                cert_type: "".to_string(),
                data: "".to_owned(),
            };

            certificate.list();

            return;
        } else {
            println!(
                "Usage: certificate [create,delete,list,read,update] <vault_name> <rsa, ecdsa> <certificate name>"
        );
            return;
        }
    }

    let mut certificate = Certificate {
        vault_name: args[1].to_string(),
        name: args[2].to_string(),
        cert_type: args[3].to_string(),
        data: "".to_owned(),
    };

    match args[0] {
        "create" => certificate.create(),
        "delete" => println!("Certificate delete: {:?}", args),
        "list" => println!("Certificate list: {:?}", args),
        "update" => println!("Certificate update: {:?}", args),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl certificate::Certificate {
    fn list(&self) {
        println!("Listing certificates");

        let values = self.db_list();

        match values {
            Ok(value) => {
                value.iter().for_each(|v| println!("{}", v));
            },
            Err(e) => println!("{}", e),
        }
    }

    fn create<'a>(&'a mut self) {
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

    fn update(&self, args: &[&str]) {
        println!("Certificate update: {:?}", args);
    }

    fn delete(&self, args: &[&str]) {
        println!("Certificate delete: {:?}", args);
    }
}
