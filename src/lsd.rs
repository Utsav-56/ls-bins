use chrono::{DateTime, Local};
use clap::{Args, Parser};
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "lsd", about = "List directories with filters and sorting")]
struct Cli {
    #[arg(help = "Path to list directories from", default_value = ".")]
    path: String,

    #[arg(short = 'q', long = "query", num_args = 1.., help = "Search for directories matching queries (case-insensitive)")]
    query: Vec<String>,

    #[arg(short = 's', long = "sort", num_args = 1.. ,
    default_value = "n",
    help = "Sort directories by choice s or size , m or modified time, n or name, priority is given to first argument if multiple options passed.")]
    sort: Vec<String>,

    #[arg(
        short = 'o',
        long = "order",
        default_value = "asc",
        help = "Sort directories by order if s is not provided does nothing accepts desc,1,d or asc,0,a."
    )]
    order: String,

    #[arg(
        short = 'i',
        long = "index",
        default_value = "",
        help = "Returns the directory in that particular index  in the list"
    )]
    index: String,


}

fn main() {
    let cli = Cli::parse();

    let mut options = HashMap::new();

    let queries: Vec<String> = cli.query.iter().map(|s| s.to_lowercase()).collect();


   options.insert(
       "order".to_string(),
       if cli.order.to_lowercase() == "asc" || cli.order == "1" || cli.order.to_lowercase() == "a" {
           "1".to_string()
       } else {
           "".to_string()
       },
   );



    if !cli.sort.is_empty() {
    options.insert("sort".to_string(), cli.sort[0].to_lowercase());
    }

    options.insert("index".to_string(), cli.index.to_string());

    run(cli.path, queries, options);
}

fn run(target_path:String , queries:Vec<String>, options:HashMap<String, String> ) {

    let empty = "".to_string();
    let sort_criteria = options.get("sort").unwrap_or(&empty);
    let order = options.get("order").unwrap_or(&empty);


    let dir = Path::new(&target_path);
    if !dir.is_dir() {
        eprintln!("{}", "Error: Not a valid directory".red());
        return;
    }

    let mut results = vec![];

    for entry in fs::read_dir(dir).unwrap_or_else(|_| {
        eprintln!("{}", "Error: Unable to read directory".red());
        std::process::exit(1);
    }) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
                let item_count = fs::read_dir(&path).map(|iter| iter.count()).unwrap_or(0);
                let modified = entry
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or_else(|_| {
                        eprintln!("Warning: Could not retrieve modified time for {}", dir_name);
                        std::time::SystemTime::now()
                    });
                let modified_at = DateTime::<Local>::from(modified);

                // Match search queries
                let term_match = queries.is_empty()
                    || queries.iter().any(|query| {
                        let re = Regex::new(&format!(r"(?i){}", regex::escape(query))).unwrap();
                        re.is_match(&dir_name)
                    });

                if term_match {
                    results.push((dir_name, item_count, modified_at));
                }
            }
        }
    }

    // Sort by modification time (newest first)
    // Sort based on criteria
    match sort_criteria.as_str() {
        "m" | "modified" => results.sort_by(|a, b| b.2.cmp(&a.2)),
        "s" | "size" => results.sort_by(|a, b| b.1.cmp(&a.1)),
        "n" | "name" => results.sort_by(|a, b| a.0.cmp(&b.0)),
        _ => {}
    }

    if order == "1" {
        results.reverse();
    }


        let index = options.get("index").unwrap_or(&empty).parse::<i8>().unwrap_or(-1);



    let len = results.len();

    if index < len as i8 && index != -1 && index>-1 {

        let (dir_name, _ ,_) = &results[index as usize];

        //just print the directory name
        println!("{}", dir_name);

        return;
    }

    println!(
        "{:<5} | {:<35} | {:<6} | {}",
        "Index".bold().underline(),
        "Folder Name".bold().underline(),
        "Items".bold().underline(),
        "Modified At".bold().underline()
    );

    for (index, (dir_name, item_count, modified_at)) in results.iter().enumerate() {




        println!(
            "{:<5} | {:<35} | {:<6} items | {}",
            (index + 1).to_string().blue(),
            dir_name.bold().cyan(),
            item_count,
            modified_at.format("%Y-%m-%d %H:%M:%S")
        );
    }
}
