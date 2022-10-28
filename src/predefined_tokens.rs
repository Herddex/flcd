use std::{fs, io};
use std::collections::HashMap;
use std::ops::Deref;

pub struct PredefinedTokens {
    tokens: HashMap<String, u32>
}

impl PredefinedTokens {
    pub fn new(tokens_file_path: &str) -> Result<PredefinedTokens, io::Error> {
        let mut map = HashMap::new();
        let mut next_value = 2;
        fs::read_to_string(tokens_file_path)?.split_whitespace()
            .for_each(|str| {
                map.insert(String::from(str), next_value);
                next_value += 1;
            });
        Ok(PredefinedTokens { tokens: map })
    }
}

impl Deref for PredefinedTokens {
    type Target = HashMap<String, u32>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}