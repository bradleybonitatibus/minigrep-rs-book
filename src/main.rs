use std::env;
use std::process;

mod lib;
use lib::Config;

fn main() {
    // get program arguments
    let args: Vec<String> = env::args().collect();

    // constructor to create config object and unwraps result
    let conf: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", conf.query);
    println!("In file: {}", conf.filename);

    if let Err(e) = lib::run(conf) {
        println!("Application error: {}", e);
    }
}


