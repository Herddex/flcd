use std::{fs, io, mem};
use std::iter::Peekable;
use std::str::Chars;
use std::error::Error;
use std::io::ErrorKind;

pub struct Scanner {
    pub scanned_tokens: Vec<String>
}

impl Scanner {
    pub fn new(program_path: &str) -> Result<Scanner, Box<dyn Error>> {
        let file_contents = fs::read_to_string(program_path)?;
        let mut scanner = Scanner {
            scanned_tokens: Vec::new()
        };

        let mut char_iterator = file_contents.chars().peekable();
        let mut next_token= String::new();
        while let Some(char) = char_iterator.next() {
            if char == '"' {
                Scanner::save_token(&mut scanner, &mut next_token);
                next_token = Scanner::read_string_constant(&mut char_iterator)?;
            } else if "=<>!".contains(char) {
                Scanner::save_token(&mut scanner, &mut next_token);
                if let Some(next_char) = char_iterator.peek() {
                    if *next_char == '=' {
                        next_token.push(char);
                        next_token.push(*next_char);
                        Scanner::save_token(&mut scanner, &mut next_token);
                        char_iterator.next();
                    } else {
                        scanner.scanned_tokens.push(char.to_string());
                    }
                } else {
                    return Err(Box::new(io::Error::from(ErrorKind::InvalidInput)));
                }
            } else if "()[]{},:.+-*/%".contains(char) {
                Scanner::save_token(&mut scanner, &mut next_token);
                scanner.scanned_tokens.push(char.to_string());
            } else if char.is_whitespace() {
                Scanner::save_token(&mut scanner, &mut next_token);
            } else {
                next_token.push(char);
            }
        }

        if !next_token.is_empty() {
            scanner.scanned_tokens.push(next_token);
        }
        Ok(scanner)
    }

    fn save_token(scanner: &mut Scanner, token: &mut String) {
        if !token.is_empty() {
            scanner.scanned_tokens.push(mem::replace(token, String::new()));
        }
    }

    fn read_string_constant(iterator: &mut Peekable<Chars>) -> Result<String, Box<dyn Error>> {
        let mut string_constant = String::from("\"");
        while let Some(char) = iterator.next() {
            string_constant.push(char);
            if char == '"' {
                return Ok(string_constant);
            }
        }
        Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput)))
    }
}