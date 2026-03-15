/*
 * 11. Minigrep 
 *   Recreate the classic grep tool (as outlined in the Rust book). 
 *   Your program should accept a search string and a filename as command-line arguments. 
 *   Read the file, search for the string line-by-line, and print matching lines. 
 *   Implement an environment variable (IGNORE_CASE=1) to toggle case-insensitive searching.
 */

use clap::Parser;
use std::env;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long)]
    query: String,
}

fn main() {
    let args = Args::parse();
    let ignore_case = match env::var("IGNORE_CASE") {
        Ok(value) => value == "1",
        Err(_) => false
    };
    let query =  if ignore_case { args.query.to_ascii_lowercase() } else { args.query };

    match fs::read_to_string(&args.filename) {
        Ok(contents) => contents.lines().for_each(|line| {
            if ignore_case && line.to_ascii_lowercase().contains(&query) {
                println!("{line}");
            } else if line.contains(&query) {
                    println!("{line}");
                }
        }),
        Err(e) => println!("Failed reading file `{}`: {e}", &args.filename)
    }
}
