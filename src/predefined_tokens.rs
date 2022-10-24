use std::{fs, io};
use std::ops::Deref;

pub struct PredefinedTokens {
    tokens: Vec<String>
}

impl PredefinedTokens {
    pub fn new(tokens_file_path: &str) -> Result<PredefinedTokens, io::Error> {
        let predefined_tokens = fs::read_to_string(tokens_file_path)?.split_whitespace()
            .map(|str| String::from(str))
            .collect();
        Ok(PredefinedTokens { tokens: predefined_tokens })
    }
}

impl Deref for PredefinedTokens {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}