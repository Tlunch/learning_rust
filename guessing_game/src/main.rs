use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    
    println!("Guess the number!");

    loop{
        println!("Input a guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a postitive integer.");
                    continue;
                },
            };

        println!("You guessed: {guess}");
        
        match guess.cmp(&number){
            Ordering::Less => println!("Guess too low!"),
            Ordering::Greater => println!("Guess too large!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            },
        };
    }
}

