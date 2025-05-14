use chrono::{DateTime, Local};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::{fs, path::Path};
use std::process::Command;

#[derive(Parser)]
#[command(name = "lsd", about = "List directories with filters and sorting")]
struct Cli {
    #[arg(default_value = ".")] path: String,
    #[arg(short = 'q', long = "query", num_args = 0..)] query: Vec<String>,
    #[arg(short = 's', long = "sort", num_args = 0..)] sort: Vec<String>,
    #[arg(short = 'o', long = "order", default_value = "asc")] order: String,
    #[arg(short = 'i', long = "index", default_value = "0")] index: usize,
    #[arg(short = 'm', long = "minimal")] minimal: bool,
}

fn main() {
    let cli = Cli::parse();
    let queries: Vec<_> = cli.query.iter().map(|s| s.to_lowercase()).collect();
    let desc_order = matches!(cli.order.to_lowercase().as_str(), "desc" | "0" | "d");

    // Check if the path is a valid number
    if let Ok(index) = cli.path.parse::<usize>() {
        // Call the program with the -i flag
        let output = Command::new("lsd")
            .arg("-i")
            .arg(index.to_string())
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // If the output is not blank, set the path to ./<output>
        if !output_str.is_empty() {
            let new_path = format!("./{}", output_str);

            if cli.minimal {
                run_minimal(new_path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
                return;
            }

            run(new_path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
            return;
        }
    }

    if cli.minimal {
        run_minimal(cli.path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
        return;
    }

    run(cli.path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
}

fn run(path: String, queries: Vec<String>, sort: &String, desc: bool, query_index: usize) {
    let dir = Path::new(&path);
    if !dir.is_dir() {
        eprintln!("{}", "Error: Not a valid directory".red());
        return;
    }

    let mut largest_len = 0;
    let mut current_len = 0;

    let mut largest_count = 0;
    let mut current_count = 0;

    let mut results: Vec<_> = fs::read_dir(dir).unwrap_or_else(|_| {
        eprintln!("{}", "Error: Unable to read directory".red());
        std::process::exit(1);
    })
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| {
        let path = entry.path();
        path.is_dir().then(|| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            let count = fs::read_dir(&path).map(|iter| iter.count()).unwrap_or(0);
            let modified = entry.metadata().ok()?.modified().ok()?;
            let size = entry.metadata().ok()?.len();

            current_count = count.to_string().len();
            if current_count > largest_count {
                largest_count = current_count;
            }

            current_len = name.len();
            if current_len > largest_len {
                largest_len = current_len;
            }

            Some((0, name, count, DateTime::<Local>::from(modified)))
        })
    }).flatten().collect();

    // Assign proper index values (starting from 1)
    for (idx, item) in results.iter_mut().enumerate() {
        item.0 = idx + 1;
        if query_index == idx + 1 {
            println!("{}", item.1);
            return;
        }
    }

     results = results. into_iter()
    .filter(|(_, name, _, _)| queries.is_empty() || queries.iter().any(|q| Regex::new(&format!("(?i){}", regex::escape(q))).unwrap().is_match(name)))
    .collect();

    // Sort by name first and then give them a proper index
    results.sort_by(|a, b| a.1.cmp(&b.1));

    if query_index > 0 && (query_index) > results.len() {
        println!("");
        return;
    }



    match sort.as_str() {
        "m" | "modified" => results.sort_by(|a, b| b.3.cmp(&a.3)),
        "s" | "size" => results.sort_by(|a, b| b.2.cmp(&a.2)),
        _ => {}
    }

    if desc { results.reverse(); }

    let headers = format!("{:<5} | {:<width$} | {:<items_width$} | {}", "Index".bold().underline(), "Folder Name".bold().underline(), "Items".bold().underline(),"Modified At".bold().underline(), width = largest_len, items_width = largest_count+6);

    println!("{}", headers);

    for (i, (indx, name, count, modified_at)) in results.iter().enumerate() {
       let line =  format!("{:<5} | {:<width$} | {:<items_width$} items | {}", (indx).to_string().blue(), name.bold().cyan(), count, modified_at.format("%Y-%m-%d %H:%M:%S"), width = largest_len, items_width = largest_count);
         println!("{}", line);
    }
}

fn run_minimal(path: String, queries: Vec<String>, sort: &String, desc: bool, query_index: usize) {
    let dir = Path::new(&path);
    if !dir.is_dir() {
        eprintln!("{}", "Error: Not a valid directory".red());
        return;
    }

    let mut largest_len = 0;
    let mut current_len = 0;

    let mut results: Vec<_> = fs::read_dir(dir).unwrap_or_else(|_| {
        eprintln!("{}", "Error: Unable to read directory".red());
        std::process::exit(1);
    })
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| {
        let path = entry.path();
        path.is_dir().then(|| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            current_len = name.len();
            if current_len > largest_len {
                largest_len = current_len;
            }
            Some((0, name))
        })
    })
    .flatten()
    .filter(|(_, name)| queries.is_empty() || queries.iter().any(|q| Regex::new(&format!("(?i){}", regex::escape(q))).unwrap().is_match(name)))
    .collect();

    // Sort by name first and then give them a proper index
    results.sort_by(|a, b| a.1.cmp(&b.1));

    if query_index > 0 && (query_index) > results.len() {
        println!("");
        return;
    }

    // Assign proper index values (starting from 1)
    for (idx, item) in results.iter_mut().enumerate() {
        item.0 = idx + 1;
        if query_index == idx + 1 {
            println!("{}", item.1);
            return;
        }
    }

    if desc { results.reverse(); }

    let header = format!("{:<5} | {:<width$} |", "S.n".bold().underline(), "Folder Name".bold().underline(), width = largest_len);
    println!("{}", header);

    // println!("{:<5} | {:<35} |", "Index".bold().underline(), "Folder Name".bold().underline());

    for (i, (indx, name)) in results.iter().enumerate() {
        let line = format!("{:<5} | {:<width$} |", (indx).to_string().blue(), name.bold().green(), width = largest_len);
        println!("{}", line);
        // println!("{:<5} | {:<35} |", (indx).to_string().blue(), name.bold().cyan());
    }
}