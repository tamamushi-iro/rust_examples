use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Guess the Number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("Secret Number is: {secret_num}");

    loop {    
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Go Higher"),
            Ordering::Greater => println!("Go Lower"),
            Ordering::Equal => {
                println!("You Guessed It!");
                break;
            }
        }
    }
}