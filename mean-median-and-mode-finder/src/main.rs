use std::collections::HashMap;

/*
 * 6. Mean, Median, & Mode Finder
 *   Given a hardcoded list of integers (a Vec<i32>), write functions to calculate the 
 *   mean (average, requiring a cast to f64), the median (middle value, requiring you to 
 *   sort the vector first), and the mode (most frequent value, requiring a HashMap to track occurrences).
 */
fn main() {
    let mut numbers: Vec<i32> = vec![
        -72, 45, -12, 88, -3, 56, -91, 24, 10, -67, 
        33, -50, 19, -8, 95, -22, 61, -39, 4, -84, 
        77, -15, 42, -99, 13, 2, -55, 68, -30, 88
    ];
    let sum: i32 = numbers.iter().sum();
    let mean: f64 = (sum as f64) / (numbers.len() as f64);

    numbers.sort();
    let median = numbers[numbers.len() / 2];
    
    let mut freq: HashMap<i32, i32> = HashMap::new();
    numbers.iter().for_each(|n| -> () {
        let n_freq = freq.get(n);
        match n_freq {
            Some(freq_for_n) => freq.insert(*n, *freq_for_n + 1),
            None => freq.insert(*n, 1)
            
        };
    });
    let max_key = freq.iter().max_by_key(|&(_, v)| v).map(|(k, _v)| k);

    println!("Mean = {mean} Median = {median} Most frequent = {}", max_key.unwrap());
}
