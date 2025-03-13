
use colored::*;

pub fn run() {
    if let Ok(path) = std::env::var("PATH") {
        println!("{}", "Current Shell PATH Variables:".bold().underline());
        for (index, path) in path.split(':').enumerate() {
            println!("{}: {}", index, path.green());
        }
    } else {
        eprintln!("{}", "Error: Unable to retrieve PATH variable".red());
    }
}

fn main() {
    run();
}