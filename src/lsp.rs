
use colored::*;
use regex::Regex;
use std::env;



pub fn run(queries: Vec<String>) {
    if let Ok(path) = env::var("PATH") {
        println!("{}", "Current Shell PATH Variables:".bold().underline());
        for (index, path) in path.split(':').enumerate() {
            if queries.is_empty() || queries.iter().any(|q| Regex::new(&format!("(?i){}", regex::escape(q))).unwrap().is_match(path)) {
                println!("{}: {}", index, path.green());
            }
        }
    } else {
        eprintln!("{}", "Error: Unable to retrieve PATH variable".red());
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let queries = args[1..].to_vec();

    run(queries);
}