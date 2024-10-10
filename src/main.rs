use grepX::{run, Config};
use std::env;
use std::process;
fn main() {
    let arg: Vec<String> = env::args().collect();

    let config = Config::build(&arg).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);
    println!("Result:\n--------------------\n");

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    println!("\n--------END---------\n");
}
