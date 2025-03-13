use chrono::{DateTime, Local};
use colored::*;
use std::fs;
use std::path::Path;

pub fn run(path: &Option<String>) {
    let dir = path.as_deref().unwrap_or(".");
    let path = Path::new(dir);

    if let Ok(entries) = fs::read_dir(path) {
        println!(
            "{:<5} | {:<25} | {:<10} | {}",
            "Index".bold().underline(),
            "File Name".bold().underline(),
            "Size (KB)".bold().underline(),
            "Modified At".bold().underline()
        );
        for (index, entry) in entries.filter_map(Result::ok).enumerate() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let modified: DateTime<Local> = metadata.modified().unwrap().into();
                    let file_name = entry.file_name().to_string_lossy().bold().green();
                    let size_kb = metadata.len() as f64 / 1024.0;

                    println!(
                        "{:<5} | {:<25} | {:<10.2} | {}",
                        index,
                        file_name,
                        size_kb,
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
