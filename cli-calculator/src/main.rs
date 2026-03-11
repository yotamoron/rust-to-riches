/*
 * 4. CLI Calculator 
 *  Create a program that takes a simple mathematical expression from 
 * standard input (e.g., "5 + 3" or "10 / 2"). 
 * Define an enum representing the four basic operations 
 * (Add, Subtract, Multiply, Divide). 
 * Parse the input, map the operator to your enum, and use a 
 * match statement to execute the operation and print the result.
 */
use std::io;

fn operator(symbol: &str) -> impl Fn(i32, i32) -> i32 {
    match symbol {
        "+" => |l: i32, r: i32| -> i32 { l + r },
        "*" => |l: i32, r: i32| -> i32 { l * r },
        "-" => |l: i32, r: i32| -> i32 { l - r },
        "/" => |l: i32, r: i32| -> i32 { l / r },
        _ => panic!("Wrong operand")
    }
}

fn operand(split: &Vec<&str>, idx: usize, msg: &str) -> i32 {
    let op: i32 = split
            .get(idx)
            .unwrap()
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("{msg}"));
        op
}

fn main() {
    let mut input = String::new();
    println!("Insert your expression:");

    io::stdin().read_line(&mut input).expect("Should be able to read from cli");
    let split: Vec<&str> = input.split_ascii_whitespace().collect();
    assert_eq!(split.len(), 3, "Wrong expression");
    let left = operand(&split, 0, "Wrong left side");
    let operator = operator(split[1]);
    let right = operand(&split, 2, "Wrong right side");

    println!("Result: {}", operator(left, right));
    
}
