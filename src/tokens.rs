use std::{fs, io};
use std::ops::Deref;

pub struct Tokens {
    tokens: Vec<String>
}

impl Tokens {
    pub fn new(tokens_file_path: &str) -> Result<Tokens, io::Error> {
        let tokens = fs::read_to_string(tokens_file_path)?.split_whitespace()
            .map(|str| String::from(str))
            .collect();
        Ok(Tokens{ tokens })
    }
}

impl Deref for Tokens {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}