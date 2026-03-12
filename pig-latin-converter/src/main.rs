/*
 * 5. Pig Latin Converter 
 *   Write a script that converts a given string of text into Pig Latin. 
 *   If a word starts with a consonant, move the first consonant to the 
 *   end of the word and add "ay" (e.g., "first" becomes "irst-fay"). 
 *   If a word starts with a vowel, just add "hay" to the end (e.g., "apple" becomes "apple-hay"). 
 *   This forces you to deal with Rust's strict UTF-8 string handling.
 */

use std::collections::HashSet;
use std::io::{self};

fn read_input() -> String {
    let mut input = String::new();

    println!("Enter the word:");

    io::stdin().read_line(&mut input).expect("Should be able to read");
    
    String::from(input.trim())
}

fn main() {
    let vowels: HashSet<char> = HashSet::from_iter(vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
    let input = read_input();
    let letters = Vec::from_iter(input.chars().into_iter());
    let initial_letter = &letters[0]; 
    
    if vowels.contains(initial_letter) {
        println!("{input}-hay")
    } else {
        let prefix: String = (&letters[1..]).iter().collect();
        
        println!("{prefix}-{initial_letter}ay");
    }
}
