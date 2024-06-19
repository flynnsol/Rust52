use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Debug, PartialEq)]
enum Token {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next(); // skip whitespace
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            'n' => {
                let literal: String = chars.by_ref().take(4).collect();
                if literal == "null" {
                    tokens.push(Token::Null);
                } else {
                    return Err(format!("Unexpected token: {}", literal));
                }
            }
            't' => {
                let literal: String = chars.by_ref().take(4).collect();
                if literal == "true" {
                    tokens.push(Token::Bool(true));
                } else {
                    return Err(format!("Unexpected token: {}", literal));
                }
            }
            'f' => {
                let literal: String = chars.by_ref().take(5).collect();
                if literal == "false" {
                    tokens.push(Token::Bool(false));
                } else {
                    return Err(format!("Unexpected token: {}", literal));
                }
            }
            '"' => {
                chars.next(); // consume the quote
                let string: String = chars.by_ref().take_while(|&c| c != '"').collect();
                tokens.push(Token::String(string));
            }
            '0'..='9' | '-' => {
                let number: String = chars.by_ref().take_while(|c| c.is_digit(10) || *c == '.').collect();
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    Ok(tokens)
}

fn parse(tokens: &[Token]) -> Result<JsonValue, String> {
    let (value, _) = parse_value(tokens, 0)?;
    Ok(value)
}

fn parse_value(tokens: &[Token], mut index: usize) -> Result<(JsonValue, usize), String> {
    match tokens.get(index) {
        Some(Token::Null) => Ok((JsonValue::Null, index + 1)),
        Some(Token::Bool(b)) => Ok((JsonValue::Bool(*b), index + 1)),
        Some(Token::Number(n)) => Ok((JsonValue::Number(*n), index + 1)),
        Some(Token::String(s)) => Ok((JsonValue::String(s.clone()), index + 1)),
        Some(Token::LeftBracket) => {
            let mut array = Vec::new();
            index += 1;
            while let Some(token) = tokens.get(index) {
                if token == &Token::RightBracket {
                    return Ok((JsonValue::Array(array), index + 1));
                }
                let (value, next_index) = parse_value(tokens, index)?;
                array.push(value);
                index = next_index;
                if tokens.get(index) == Some(&Token::Comma) {
                    index += 1;
                }
            }
            Err("Expected closing bracket".to_string())
        }
        Some(Token::LeftBrace) => {
            let mut object = HashMap::new();
            index += 1;
            while let Some(token) = tokens.get(index) {
                if token == &Token::RightBrace {
                    return Ok((JsonValue::Object(object), index + 1));
                }
                if let Some(Token::String(key)) = tokens.get(index) {
                    index += 1;
                    if tokens.get(index) == Some(&Token::Colon) {
                        index += 1;
                        let (value, next_index) = parse_value(tokens, index)?;
                        object.insert(key.clone(), value);
                        index = next_index;
                        if tokens.get(index) == Some(&Token::Comma) {
                            index += 1;
                        }
                    } else {
                        return Err(format!("Expected colon at index {}, found {:?}", index, tokens.get(index)));
                    }
                } else {
                    return Err(format!("Expected string key at index {}, found {:?}", index, tokens.get(index)));
                }
            }
            Err("Expected closing brace".to_string())
        }
        _ => Err(format!("Unexpected token at index {}, found {:?}", index, tokens.get(index))),
    }
}

fn read_file(file_path: &str) -> String {
    let mut contents = String::new();
    match fs::read_to_string(file_path) {
        Ok(n) => contents = n,
        Err(_e) => println!("Failed to read file"),
    }
    return contents;
}

fn main() {
    let json = read_file("./test.json");

    match tokenize(&*json) {
        Ok(tokens) => {
            println!("{:?}", tokens);
            match parse(&tokens) {
                Ok(parsed_json) => println!("{:?}", parsed_json),
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to tokenize JSON: {}", e),
    }
}
