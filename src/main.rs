use std::env;
use std::fs;
use std::process;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct HttpSection {
    endpoints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PingSection {
    endpoints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Requests {
    http: HttpSection,
    ping: PingSection
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1)
    }

    let file_path = &args[1];
    let file_content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {e}");
            process::exit(1);
        }
    };
    
    let requests: Requests = match toml::from_str(&file_content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse TOML-file: {e}");
            process::exit(1);
        }
    };
}
