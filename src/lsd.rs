use chrono::{DateTime, Local};
use colored::*;
use std::fs;
use std::path::Path;

pub fn run(path: &Option<String>) {
    let dir = path.as_deref().unwrap_or(".");
    let path = Path::new(dir);

    if let Ok(entries) = fs::read_dir(path) {
        println!(
            "{:<5} | {:<25} | {:<5} | {}",
            "Index".bold().underline(),
            "Folder Name".bold().underline(),
            "Items".bold().underline(),
            "Modified At".bold().underline()
        );
        for (index, entry) in entries.filter_map(Result::ok).enumerate() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let modified: DateTime<Local> = metadata.modified().unwrap().into();
                    let folder_name = entry.file_name().to_string_lossy().bold().cyan();
                    let count = fs::read_dir(entry.path()).unwrap().count();

                    println!(
                        "{:<5} | {:<25} | {:<5} | {}",
                        index,
                        folder_name,
                        count,
                        modified.format("%Y-%m-%d %H:%M:%S")
                    );
                }
            }
        }
    } else {
        eprintln!("{}", "Error: Unable to read directory".red());
    }
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };



    run(&path);
}
