use crate::common::record_trait::RecordTrait;

pub struct Certificate {
    pub name: String,
    pub data: String,
}

pub fn certificate(args: &[&str]) {
    if args.len() < 2 {
        println!(
            "Usage: certificate [create,delete,list,read,update] [rsa, ecdsa] <certificate name>"
        );
        return;
    }

    for arg in args {
        match args[0] {
            "create" => println!("Certificate create: {:?}", args),
            "delete" => println!("Certificate delete: {:?}", args),
            "list" => println!("Certificate list: {:?}", args),
            "update" => println!("Certificate update: {:?}", args),
            _ => println!("Unknown command: {}", arg),
        }
    }
}

impl RecordTrait for Certificate {
    fn create(&self, args: &[&str]) {
        println!("Certificate create: {:?}", args);
    }

    fn delete(&self, args: &[&str]) {
        println!("Certificate delete: {:?}", args);
    }

    fn list(&self) {
        println!("Listing certificates");
    }

    fn update(&self, args: &[&str]) {
        println!("Certificate update: {:?}", args);
    }
}
