use std::io;

fn main() {
    println!("Enter your first number:");
    let mut num_one = get_user_input();
    println!("Enter your second number:");
    let mut num_two = get_user_input();

    while num_two < 150000 {
        let temp_num_one = num_one;
        num_one =  num_two;
        num_two = temp_num_one + num_two;
        println!("Your Number: {}", num_two);
    }
}

fn get_user_input() -> i32 {
    let input = io::stdin();
    let mut user_input = String::new();
    let mut user_input_num = 0;

    input.read_line(&mut user_input).expect("Failed to read input");
    user_input = user_input.trim().to_string();

    match user_input.parse::<i32>() {
        Ok(n) => user_input_num = n,
        Err(_e) => {
            println!("Invalid Input");
            get_user_input();
        },
    }

    return user_input_num;
}
