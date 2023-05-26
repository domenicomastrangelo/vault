use crate::common::record_trait::RecordTrait;

pub struct Vault {
    pub name: String,
    pub id: u64,
}

pub fn vault(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: vault [create,delete,list,read,update] <vault name>");
        return;
    }

    let vault = Vault {
        name: args[1].to_string(),
        id: 0,
    };

    match args[0] {
        "create" => vault.create(&args[1..]),
        "delete" => vault.delete(&args[1..]),
        "list" => vault.list(&args[1..]),
        "read" => vault.read(&args[1..]),
        "update" => vault.update(&args[1..]),
        _ => println!("Unknown command: {}", args[0]),
    }
}

impl RecordTrait for Vault {
    fn list(&self, args: &[&str]) {
        print!("Vault list {:?}", args)
    }

    fn create(&self, args: &[&str]) {
        print!("Vault create {:?}", args)
    }

    fn read(&self, args: &[&str]) {
        print!("Vault read {:?}", args)
    }

    fn update(&self, args: &[&str]) {
        print!("Vault update {:?}", args)
    }

    fn delete(&self, args: &[&str]) {
        print!("Vault delete {:?}", args)
    }
}

