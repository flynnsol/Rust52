use std::io;

fn main() {
    println!("Enter your number:");
    let mut number: u128 = get_user_input();
    let mut answer: u128 = number;

    while number > 1 {
        number -= 1;
        answer = answer * number;
    }

    println!("Answer: {}", answer);
}

fn get_user_input() -> u128 {
    let mut user_input = String::new();
    let mut user_input_num: u128 = 0;

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();

    match user_input.parse::<u128>() {
        Ok(n) => {
            if n > 0 && n < 35 {
                user_input_num = n;
            } else {
                println!("Number must be larger than 0 and Less than 34!");
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
