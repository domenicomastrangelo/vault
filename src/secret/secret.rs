use crate::common::record_trait::RecordTrait;

pub struct Secret {
    pub name: String,
    pub value: String,
}

pub fn secret(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: secret [create,delete,list,read,update] <vault name> <secret name> <secret value>");
        return;
    }

    let secret = Secret {
        name: args[1].to_string(),
        value: args[1].to_string(),
    };

    match args[0] {
        "create" => secret.create(&args[1..]),
        "delete" => secret.delete(&args[1..]),
        "list" => secret.list(),
        "update" => secret.update(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl RecordTrait for Secret {
    fn create(&self, args: &[&str]) {
        println!("Secret create: {:?}", args);
    }

    fn delete(&self, args: &[&str]) {
        println!("Secret delete: {:?}", args);
    }

    fn list(&self) {
        println!("Listing secrets");
    }

    fn update(&self, args: &[&str]) {
        println!("Secret update: {:?}", args);
    }
}
