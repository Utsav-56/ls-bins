use colored::*;
use std::env;

pub fn run() {
    println!("{}", "Environment Variables:".bold().underline());

    for (key, value) in env::vars() {

        if key == "PATH" || key =="LS_COLORS" {
            continue;
        }

        println!("{:<30} | {}", key.bold().yellow(), value.green());
    }
}

fn main() {
    run();
}