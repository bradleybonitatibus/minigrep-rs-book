use std::fs;
use std::error::Error;

// Config structure
pub struct Config {
    pub query: String,
    pub filename: String,
}

// Implement constructor and check if the arguments vector has 3 arguments
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // clone args to convert to String
        let query = args[1].clone();
        let filename = args[2].clone();

        // return Config struct
        Ok(Config { query, filename })
    }
}

// Public function to run the Parser
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // returns the error result using
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    // Return Ok result
    Ok(())
}