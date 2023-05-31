use std::error::Error;

use crate::db::certificate::Certificate;

pub fn certificate(args: &[&str]) {
    let mut certificate = parse_args(args);

    match certificate {
        Ok(ref mut certificate) => match args[0] {
            "create" => certificate.create(),
            "delete" => certificate.delete(),
            "list" => certificate.list(),
            "get" => certificate.get(),
            "disable" => certificate.disable(),
            "enable" => certificate.enable(),
            _ => println!("Unknown command: {}", args[0]),
        },
        Err(e) => println!("{}", e),
    }
}

fn parse_args(args: &[&str]) -> Result<Certificate, Box<dyn Error>> {
    if args.len() < 2 {
        return Err(
            "Usage: certificate [create,delete,list,get,update] <vault_name> <certificate_name>"
                .into(),
        );
    }

    let mut certificate = Certificate {
        vault_name: args[1].to_string(),
        name: "".to_string(),
        cert_type: "".to_string(),
        data: "".to_string(),
        enabled: true,
    };

    if args.len() >= 3 {
        certificate.name = args[2].to_string();
    }

    if args.len() >= 4 {
        certificate.cert_type = args[3].to_string();
    }

    Ok(certificate)
}

impl Certificate {
    fn list(&self) {
        println!("Listing certificates");

        let values = self.db_list();

        match values {
            Ok(value) => {
                value.iter().for_each(|v| println!("{}", v));
            }
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

    fn delete(&mut self) {
        println!("Deleting certificate: {:?}", self.name);

        let res = self.db_delete();

        match res {
            Ok(_) => println!("Certificate deleted"),
            Err(e) => println!("Error deleting certificate: {}", e),
        }
    }

    fn get(&mut self) {
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

    fn disable(&mut self) {
        println!("Disabling certificate: {:?}", self.name);

        let res = self.db_disable();

        match res {
            Ok(_) => println!("Certificate disabled"),
            Err(e) => println!("Error disabling certificate: {}", e),
        }
    }

    fn enable(&mut self) {
        println!("Enabling certificate: {:?}", self.name);

        let res = self.db_enable();

        match res {
            Ok(_) => println!("Certificate enabled"),
            Err(e) => println!("Error enabling certificate: {}", e),
        }
    }
}
