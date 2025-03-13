use colored::*;
use regex::Regex;
use std::env;

pub fn run(queries: &[String]) {
    let env_vars: Vec<(String, String)> = env::vars().collect();

    println!("{}", "Environment Variables:".bold().underline());

    for (key, value) in env_vars {
        // Check if any query matches key (case-insensitive)
        let matches = queries.iter().any(|query| {
            let re = Regex::new(&format!(r"(?i){}", regex::escape(query))).unwrap();
            re.is_match(&key)
        });

        // Print only matching variables if queries exist, otherwise print all
        if queries.is_empty() || matches {
            println!("{:<30} | {}", key.bold().yellow(), value.green());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let queries = &args[1..];

    run(queries);
}