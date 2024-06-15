use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    let config = parse_config();

    println!("Searching for {}", config.read_write);
    println!("In file {}", config.file_path);

    if config.read_write == "r" {
        let contents = read_file(config.file_path);
        println!("{contents}");
    } else {
        write_to_file(config.file_path);
    }
}

struct Config {
    read_write: String,
    file_path: String,
}

fn parse_config() -> Config {
    println!("Type 'r' for Read. Type 'w' for write.");
    let read_write = get_user_input();
    println!("Enter a file path. If you want this projects main directory type ./ followed by the file name/directory.");
    let file_path = get_user_input();
    let config = Config {
        read_write: read_write.trim().to_string().clone(),
        file_path: file_path.trim().to_string().clone(),
    };
    return config;
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    return user_input;
}

fn read_file(file_path: String) -> String {
    let mut contents = String::new();
    match fs::read_to_string(file_path) {
        Ok(n) => contents = n,
        Err(_e) => println!("Failed to read file"),
    }

    return contents;
}

fn write_to_file(file_path: String) {
    match File::create(file_path) {
        Ok(n) => {
            let mut file = n;
            println!("Enter your content to write to a file.");
            let content = get_user_input();

            file.write_all(content.as_ref()).expect("Failed to write to file");

        }
        Err(_e) => println!("Failed to create file"),
    }
}
