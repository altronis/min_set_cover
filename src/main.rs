use std::env;
use std::process;
use min_set_cover;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  min_set_cover <filename>");
        process::exit(1);
    }

    let filename: &str = &args[1];

    if let Err(e) = min_set_cover::run(filename) {
        println!("{}", e);
        process::exit(1);
    }
}
