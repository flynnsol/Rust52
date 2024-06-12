use std::io;
use rand::prelude::*;

fn main() {
    let mut running = true;
    let mut guess_count = -1;
    let mut num_to_guess = 0;
    let mut win = false;

    while running {
        if guess_count == -1 {
            num_to_guess = get_num_to_guess();
            guess_count = 3;
            println!("Welcome to Guess a Number!");
            println!("Guess a number 1-10.");
        } else if guess_count == 0 {
            win = false;
        }
        if win == false && guess_count == 0 {
            println!("You're out of guesses! The number was {}!", num_to_guess);
            println!("Would you like to play again?");
            println!("1) Yes");
            println!("2) No");
            let answer = get_user_input();
            if answer == 1 {
                win = false;
                guess_count = -1;
                num_to_guess = get_num_to_guess();
            } else {
                running = false;
            }
        } else if win == true && guess_count > 0 {
            println!("You Won! The answer was {}!", num_to_guess);
            println!("Would you like to play again?");
            println!("1) Yes");
            println!("2) No");
            let answer = get_user_input();
            if answer == 1 {
                win = false;
                guess_count = -1;
                num_to_guess = get_num_to_guess();
            } else {
                running = false;
            }
        } else {
            println!("You have {} guesses left! Guess again!", guess_count);
            let guess = get_user_input();
            if guess == num_to_guess {
                win = true;
            } else {
                guess_count -= 1;
            }
        }
    }
}

fn get_num_to_guess() -> i32 {
    let mut rng = rand::thread_rng();
    let mut nums: f64 = rng.gen();
    nums = nums * 10.0;
    let num_to_guess: i32 = nums as i32 + 1;
    num_to_guess
}

fn get_user_input() -> i32 {
    let input = io::stdin();
    let mut user_input = String::new();

    input.read_line(&mut user_input).expect("Failed to read input");
    let mut user_input_int: i32 = 0;
    user_input = user_input.trim().to_string();

    match user_input.parse::<i32>() {
        Ok(n) => {
            if n > 0 && n < 11 {
                user_input_int = n;
            } else {
                println!("That's outside of the range of numbers. Guess again.");
                get_user_input();
            }
        },
        Err(_e) => {
            println!("Invalid Input");
            get_user_input();
        },
    }
    return user_input_int;
}
