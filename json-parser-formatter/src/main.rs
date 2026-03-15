/*
 * 12. JSON Parser/Formatter 
 *   Using the serde and serde_json crates, write a program that reads a messy, unformatted 
 *   JSON file from your disk. Define Rust structs that accurately represent the JSON schema, 
 *   deserialize the file into your structs, and then serialize it back to a new file, 
 *   but pretty-printed with proper indentation.
 */
use clap::Parser;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    is_active: bool,
    email: String,
}

fn parse_to(contents: &str, to: &str) -> () {
    match serde_json::from_str::<User>(contents) {
        Ok(user) => {
            match serde_json::to_string_pretty(&user) {
                Ok(pretty) => fs::write(to, pretty).unwrap(),
                Err(e) => println!("Failed serializing {e}")
            };
        },
        Err(e) => println!("error parsing file {e}: {}", contents)
    };
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.from) {
        Ok(contents) => parse_to(&contents, &args.to),
        Err(e) => println!("Failed reading file {}: {e}", &args.from)
    };
}
