/*
 * 3. Nth Fibonacci Generator 
 *    Write a function that takes an integer 
 *    n and returns the nth number in the Fibonacci sequence. 
 *    Try implementing it twice: once using recursion and once using a 
 *    simple loop (iteration). This will test your understanding of basic 
 *    integer types and control flow without hitting performance bottlenecks.
*/

fn nth_recursive(n_minus_2: u64, n_minus_1: u64, remaining: u64) -> u64 {
    let next = n_minus_2 + n_minus_1;
    match remaining {
        2 => next,
        1 => n_minus_1,
        0 => n_minus_2,
        _ => nth_recursive(n_minus_1, next, remaining - 1)
    }
}

fn nth(n: u64) -> u64 {
    let mut n_minus_2 = 0;
    let mut n_minus_1 = 1;
    let mut next: u64 = n_minus_2 + n_minus_1;

    match n {
        0 => n_minus_2,
        1 => n_minus_1,
        2 => next,
        _ => {
            let mut remaining = n;
            while remaining > 1 {
                next = n_minus_2 + n_minus_1;
                n_minus_2 = n_minus_1;
                n_minus_1 = next;
                remaining -= 1;
            }
            next
        }
    }
}

fn main() {
    let target: u64 = 93;
    let nth_rec = nth_recursive(0, 1, target);
    let nth = nth(target);

    println!("{nth_rec} {nth}");
}
