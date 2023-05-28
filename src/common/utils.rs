pub fn check_args(num: usize, args: &[&str]) {
    if args.len() < num {
        println!("Wrong number of arguments");
        std::process::exit(1);
    }
}
