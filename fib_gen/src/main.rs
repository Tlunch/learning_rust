use std::io;
use std::process;
use num_bigint::BigUint;
use num_traits::{One};

fn main() {
    let mut n = String::new();
    println!("Input n:");
    io::stdin()
        .read_line(&mut n)
        .expect("Unable to read input");
    let n:u64 = match n
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not a positive integer");
                process::exit(0);
            },
        };

    if n < 3 {
        println!("The {n}th fib is 1.");
        process::exit(0);
    }
    let mut first_fib: BigUint = One::one();
    let mut second_fib: BigUint = One::one();

    for _x in 3..=n {
        let hold = &first_fib + second_fib;
        second_fib = first_fib;
        first_fib = hold;
        
    } 
    println!("The {n}th fib is {first_fib}");
}
