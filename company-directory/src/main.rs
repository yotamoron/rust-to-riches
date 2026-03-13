/*
 * 8. Company Directory 
 *   Build a text interface to manage employees and departments. 
 *   The user should be able to type commands like "Add Sally to Engineering" 
 *   or "Add Amir to Sales". Store this data using a HashMap<String, Vec<String>>. 
 *   Allow the user to request a list of all people in a specific department, or an
 *   alphabetically sorted list of all people in the company.
 */
use std::io;
use std::process;
use std::collections::HashMap;

fn read_input() -> String {
    println!("Insert Command");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Should be able to read line");
    input.trim().to_string()
}

fn add(input: &[&str], directory: &mut HashMap<String, Vec<String>>) {
    if input.len() != 3 {
        println!("Wrong number of arguments for add");
    } else {
        let people = directory.entry(input[2].to_string()).or_insert_with(Vec::new);
        if let None = people.iter().position(|s| s == input[0]) {
            people.push(input[0].to_string());
        }
    }
}

fn remove(input: &[&str], directory: &mut HashMap<String, Vec<String>>) {
    if let Some(people) = directory.get_mut(&input[2].to_string()) {
        if let Some(index) = people.iter().position(|s| s == input[0]) {
            people.remove(index);
        }
    }
}

fn show(input: &[&str], directory: &mut HashMap<String, Vec<String>>) {
    match input.len() {
        1 => {
            let mut people = directory.values().flatten().collect::<Vec<_>>();
            people.sort();
            println!("{}", people.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(", "));
        },
        3 => {
            if let Some(people) = directory.get_mut(&input[2].to_string()) {
                people.sort();
                println!("People in {} are: {}", input[2], people.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(", "));
            }
        },
        _ => println!("Unknown command")
    }
}

/*
 * Commands:
 *   Add X to Y
 *   Remove X from Y
 *   Show all from Y
 *   Show all
 *   Exit
 */
// In a real-world scenario, I would invest more in properly parsing the commands.
// However, this is just for practice.
fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let input = read_input();
        let input = input.split(" ").collect::<Vec<&str>>();

        match input[0] {
            "Exit" => {
                println!("Exiting");
                process::exit(0);
            },
            "Add" => add(&input[1..], &mut directory),
            "Remove" => remove(&input[1..], &mut directory),
            "Show" => show(&input[1..], &mut directory),
            _ => println!("Unknown command {}", input[0])
        }
    }
}
