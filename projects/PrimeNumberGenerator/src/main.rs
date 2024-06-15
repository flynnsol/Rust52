use std::io;

fn main() {
    println!("Enter a number to get all prime numbers less than it:");
    let user_input = get_user_input();

    for n in 2..user_input {
        if check_if_prime_number(n) {
            println!("Prime Number: {}", n);
        }
    }
}

fn check_if_prime_number(n: i32) -> bool {
    let mut is_prime = true;
    for i in 2..n - 1 {
        let modulo = n % i;
        if n % i == 0 {
            is_prime = false;
        }
    }
    return is_prime;
}

fn get_user_input() -> i32 {
    let mut user_input = String::new();
    let mut user_input_num = 0;

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();

    match user_input.parse::<i32>() {
        Ok(n) => {
            if n > 1 {
                user_input_num = n;
            } else {
                println!("Invalid Input. Enter a number greater than 1");
                get_user_input();
            }
        },
        Err(_e) => {
            println!("Invalid Input");
            get_user_input();
        },
    }

    return user_input_num;
}
