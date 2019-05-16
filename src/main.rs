extern crate rand;

use rand::Rng;
use std::io;
use std::fs;
use std::cmp::Ordering;

fn main() {
    let mut guess_count = 8;
    let upper_bound = read_number("Please the upper bound.");
    let mut score = upper_bound;
    let secret = rand::thread_rng().gen_range(1,upper_bound);

    println!("Guess the number!");
    println!("You have {} guess(es)", guess_count);
    println!("The range is 1..{}", upper_bound);

    loop {
        if guess_count == 0 {
            println!("You lose! The answer was {}", secret);
            score = 0;
            break;
        }

        guess_count -= 1;
        let guess = read_number("Make a guess!");

        println!("You guessed {} with {} guess(es) remaining", guess, guess_count);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let high_score = get_high_score();
    match score.cmp(&high_score) {
        Ordering::Equal => println!("You tied the high score at {} points!", score),
        Ordering::Less => println!("Your score: {}\nHigh score: {}", score, high_score),
        Ordering::Greater => {
            println!("New high score of {}!", score);
            write_high_score(score);
        }
    }
}

fn read_number(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        return number;
    }
}

fn get_high_score() -> i32 {
    let data = fs::read_to_string("/home/bbolen/projects/rust/guessing_cargo/high_score.txt")
        .expect("Unable to read file");
    let score: i32 = data.trim().parse()
        .expect("High score was not a number");
    score
}

fn write_high_score(score: i32) -> () {
    fs::write("/home/bbolen/projects/rust/guessing_cargo/high_score.txt", score.to_string())
        .expect("Unable to write file");
}
