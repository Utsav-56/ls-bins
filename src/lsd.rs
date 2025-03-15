use chrono::{DateTime, Local};
use clap::Parser;

use colored::*;


use regex::Regex;
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "lsd", about = "List directories with filters and sorting")]
struct Cli {
    #[arg(default_value = ".")] path: String,
    #[arg(short = 'q', long = "query", num_args = 0..)] query: Vec<String>,
    #[arg(short = 's', long = "sort", num_args = 0..)] sort: Vec<String>,
    #[arg(short = 'o', long = "order", default_value = "asc")] order: String,
    #[arg(short = 'i', long = "index", default_value = "0")] index: usize,
}
use std::process::Command;

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
            run(new_path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
            return;
        }
    }

    // Proceed as usual if the path is not a valid number or output is blank
    run(cli.path, queries, cli.sort.first().unwrap_or(&"null".to_string()), desc_order, cli.index);
}

fn run(path: String, queries: Vec<String>, sort: &String, desc: bool, query_index: usize) {
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
            path.is_dir().then(|| {

                let name = path.file_name()?.to_string_lossy().into_owned();
                let count = fs::read_dir(&path).map(|iter| iter.count()).unwrap_or(0);
                let modified = entry.metadata().ok()?.modified().ok()?;
                Some((0, name, count, DateTime::<Local>::from(modified)))
            })
        })
        .flatten()
        .filter(|(_, name, _, _)| queries.is_empty() || queries.iter().any(|q| Regex::new(&format!("(?i){}", regex::escape(q))).unwrap().is_match(name)))
        .collect();

//sort by name first and then give them a proper index
 results.sort_by(|a, b| a.1.cmp(&b.1));

    if query_index > 0 && (query_index) > results.len() {
        println!("");
        return;
    }


    // Assign proper index values (starting from 1)
    for (idx, item) in results.iter_mut().enumerate() {
        item.0 = idx + 1;
        if query_index  == idx+1 {
            println!("{}", item.1);
            return;
        }
    }

    match sort.as_str() {
        "m" | "modified" => results.sort_by(|a, b| b.3.cmp(&a.3)),
        "s" | "size" => results.sort_by(|a, b| b.2.cmp(&a.2)),

        _ => {

        }
    }

    if desc { results.reverse(); }



    println!("{:<5} | {:<35} | {:<12} | {}", "Index".bold().underline(), "Folder Name".bold().underline(), "Items".bold().underline(), "Modified At".bold().underline());
    for (i, (indx, name, count, modified_at)) in results.iter().enumerate() {
        println!("{:<5} | {:<35} | {:<6} items | {}", (indx).to_string().blue(), name.bold().cyan(), count, modified_at.format("%Y-%m-%d %H:%M:%S"));
    }
}
