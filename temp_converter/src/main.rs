use std::io;
use std::process;

const CONVERSION_FACTOR: f64 = 5.0 / 9.0;
const CONVERSION_ADDEND: f64 = 32.0;

fn f_to_c(temp: f64) -> f64 {
    (temp - CONVERSION_ADDEND)*CONVERSION_FACTOR
}
        
fn c_to_f(temp: f64) -> f64 {
    (temp/CONVERSION_FACTOR) + CONVERSION_ADDEND
}

fn main() {
    let mut temp = String::new();
    let mut temp_type = String::new();

    println!("Enter a temperature:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");
    let temp: f64 = match temp
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number.");
                process::exit(0);
                },
        };

    println!("Enter a temperature type (F or C):");
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line.");
    let temp_type: char = match temp_type
        .trim()
        .to_uppercase()
        .parse() {
            Ok(temp_type) => temp_type,
            Err(_) => {
                println!("Not a character.");
                process::exit(0);
            },
        };
    
    if temp_type == 'F' {
        println!("The tempature {}°F is {}°C", temp, f_to_c(temp))
    } else if temp_type == 'C' {
        println!("The tempature {}°C is {}°F", temp, c_to_f(temp))
    } else {
        println!("The string {temp_type} is not a valid tempeture type.")
    }
}

