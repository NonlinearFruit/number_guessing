extern crate rand;

use rand::Rng;
use std::io;
use std::fs;
use std::cmp::Ordering;

fn main() {
    print_title();

    let mut guess_count = 8;
    let upper_bound = read_number("Please the upper bound.");
    let mut score = upper_bound;
    let secret = get_secret(upper_bound);

    println!("You have {} guess(es)", guess_count);
    println!("The range is 1..{}", upper_bound);

    if !guessing_was_successful(guess_count, secret) {
        score = 0;
    }

    process_score(score);
}

fn get_secret(upper_bound: i32) -> i32 {
    rand::thread_rng().gen_range(1,upper_bound)
}

fn guessing_was_successful(mut guess_count: i32, secret: i32) -> bool {
    if guess_count == 0 {
        println!("You lose! The answer was {}", secret);
        return false;
    }
    guess_count -= 1;
    let guess = read_number("Make a guess!");
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small. {} guess(es) left", guess_count),
        Ordering::Greater => println!("Too big. {} guess(es) left", guess_count),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    return guessing_was_successful(guess_count, secret);
}

fn process_score(score: i32) -> () {
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
    let data = match fs::read_to_string("high_score.txt") {
        Ok(num) => num,
        Err(_) => "0".to_owned()
    };
    let score: i32 = data.trim().parse()
        .expect("High score was not a number");
    score
}

fn write_high_score(score: i32) -> () {
    fs::write("high_score.txt", score.to_string())
        .expect("Unable to write file");
}

fn print_title() -> () {
    let title = r#"
     ____                     _                ____                      
    / ___|_   _  ___  ___ ___(_)_ __   __ _   / ___| __ _ _ __ ___   ___ 
   | |  _| | | |/ _ \/ __/ __| | '_ \ / _` | | |  _ / _` | '_ ` _ \ / _ \
   | |_| | |_| |  __/\__ \__ \ | | | | (_| | | |_| | (_| | | | | | |  __/
    \____|\__,_|\___||___/___/_|_| |_|\__, |  \____|\__,_|_| |_| |_|\___|
                                      |___/                              

"#;
    println!("{}", title);
}
