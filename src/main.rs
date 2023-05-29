mod certificate;
use certificate::certificate::certificate;

mod vault;
use vault::vault::vault;

mod secret;
use secret::secret::secret;

mod db;
use db::db as database;

mod common;
mod test_utils;

fn main() {
    env_logger::init();

    let conn = database::connect();

    if conn.is_err() {
        println!("Error connecting to database: {}", conn.err().unwrap());
        return;
    }

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let mut args_str = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match args[1].as_str() {
        "vault" => vault(&mut args_str[2..]),
        "secret" => secret(&args_str[2..]),
        "certificate" => certificate(&args_str[2..]),
        _ => println!("Unknown command: {}", args[1]),
    }
}
