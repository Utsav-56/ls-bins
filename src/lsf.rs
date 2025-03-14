use chrono::{DateTime, Local};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "lsf", about = "List files with filters and sorting")]
struct Cli {
    #[arg(default_value = ".")] path: String,
    #[arg(short = 'q', long = "query", num_args = 1..)] query: Vec<String>,
    #[arg(short = 's', long = "sort", default_value = "n", num_args = 1..)] sort: Vec<String>,
    #[arg(short = 'o', long = "order", default_value = "asc")] order: String,
    #[arg(short = 'i', long = "index", default_value = "")] index: String,
}

fn main() {
    let cli = Cli::parse();
    let queries: Vec<_> = cli.query.iter().map(|s| s.to_lowercase()).collect();
    let order = matches!(cli.order.to_lowercase().as_str(), "asc" | "1" | "a");
    run(cli.path, queries, cli.sort.first().unwrap_or(&"n".to_string()), order, cli.index);
}

fn run(path: String, queries: Vec<String>, sort: &String, asc: bool, index: String) {
    let dir = Path::new(&path);
    if !dir.is_dir() {
        eprintln!("{}", "Error: Not a valid directory".red());
        return;
    }

    let mut results: Vec<_> = fs::read_dir(dir).unwrap_or_else(|_| {
        eprintln!("{}", "Error: Unable to read directory".red());
        std::process::exit(1);
    })
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| {
        let path = entry.path();
        path.is_file().then(|| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            let size = entry.metadata().ok()?.len();
            let modified = entry.metadata().ok()?.modified().ok()?;
            Some((name, size, DateTime::<Local>::from(modified)))
        })
    })
    .flatten()
    .filter(|(name, _, _)| queries.is_empty() || queries.iter().any(|q| Regex::new(&format!("(?i){}", regex::escape(q))).unwrap().is_match(name)))
    .collect();

    match sort.as_str() {
        "m" | "modified" => results.sort_by(|a, b| b.2.cmp(&a.2)),
        "s" | "size" => results.sort_by(|a, b| b.1.cmp(&a.1)),
        "n" | "name" => results.sort_by(|a, b| a.0.cmp(&b.0)),
        _ => {}
    }
    if asc { results.reverse(); }

    if let Ok(idx) = index.parse::<usize>() {
        if let Some((name, _, _)) = results.get(idx) {
            println!("{}", name);
            return;
        }
    }

    println!("{:<5} | {:<35} | {:<10} | {}", "Index".bold().underline(), "File Name".bold().underline(), "Size (KB)".bold().underline(), "Modified At".bold().underline());
    for (i, (name, size, modified_at)) in results.iter().enumerate() {
        println!("{:<5} | {:<35} | {:<10.2} KB | {}", (i + 1).to_string().blue(), name.bold().green(), *size as f64 / 1024.0, modified_at.format("%Y-%m-%d %H:%M:%S"));
    }
}