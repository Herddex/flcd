use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::predefined_tokens::PredefinedTokens;
use regex::Regex;

pub struct LexicalError {
    message: String
}

impl LexicalError {
    pub fn new(message: String) -> LexicalError {
        LexicalError {
            message
        }
    }
}

impl Debug for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LexicalError {}

pub fn classify(token: &str, predefined_tokens: &PredefinedTokens) -> Result<u32, LexicalError> {
    if let Some(pif_code) = predefined_tokens.get(token) {
        Ok(*pif_code)
    } else if Regex::new(r"^[a-zA-Z]([a-zA-Z]|\d|_)*$").unwrap().is_match(token) {
        Ok(0)
    } else if Regex::new(r"^\d+$").unwrap().is_match(token) {
        Ok(1)
    } else if Regex::new("^\"[^\"]*\"$").unwrap().is_match(token) {
        Ok(1)
    } else if Regex::new("^'[^']'$").unwrap().is_match(token) {
        Ok(1)
    } else {
        Err(LexicalError { message: String::from("Lexical error at token: ".to_owned() + token) })
    }
}