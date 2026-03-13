/*
 * 9. Simple Todo List
 *   Create a persistent task manager. 
 *   Define a Task struct with a description and a boolean completed state. 
 *   Store these in a Vec<Task>. 
 *   Run a continuous loop where the user can choose to add a task, view all tasks, mark a task as 
 *   complete by its index, or quit the application.
 */
use std::io;

struct Task {
    id: i32,
    description: String,
    completed: bool
}

fn read_input() -> String {
    println!("\nEnter command:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Should read line");
    input.trim().to_string()
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut counter= 1;

    loop {
        let input = read_input();
        let input = input.split(" ").collect::<Vec<&str>>();

        match input[0] {
            "view" => tasks.iter().for_each(|t| println!("{}.\n\tDescriptio: {}\n\tCompleted:{}", t.id, t.description, t.completed)),
            "mark" => {
                match input[1].parse() {
                    Err(e) => println!("Cannot parse task id: {e}"),
                    Ok(id) => {
                        if let Some(index) = tasks.iter().position(|t| t.id == id) {
                            tasks.get_mut(index).unwrap().completed = true;
                        }
                    }
                };
            },
            "add" => {
                tasks.push(Task { id: counter, description: input[1..].join(" "), completed: false });
                counter += 1;
            },
            _ => println!("Unknown command")
        }
    }}
