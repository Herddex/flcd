use std::{fs, io};

pub struct Tokenizer {
    scanned_program: Vec<char>,
    char_index: usize,
    line_number: u32
}

impl Iterator for Tokenizer {
    type Item = (String, u32);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(char) = self.scanned_program.get(self.char_index) {
            self.char_index += 1;
            if *char == '"' || *char == '\'' {
                self.char_index -= 1;
                let constant = self.read_delimited_constant();
                return self.prepare_result(constant);
            } else if "=<>!".contains(*char) {
                if let Some(next_char) = self.scanned_program.get(self.char_index) {
                    if *next_char == '=' {
                        return self.prepare_result(format!("{}=", char));
                    }
                }
                return self.prepare_result(char.to_string());
            } else if "()[]{},:.+-*/%".contains(*char) {
                return self.prepare_result(char.to_string());
            } else if char.is_whitespace() {
                if *char == '\n' {
                    self.line_number += 1
                }
            } else {
                let mut next_token= String::new();
                next_token.push(*char);
                while let Some(char) = self.scanned_program.get(self.char_index) {
                    if char.is_whitespace() || "()[]{},:.+-*/%=<>!'\"".contains(*char) {
                        return self.prepare_result(next_token);
                    } else {
                        next_token.push(self.scanned_program[self.char_index]);
                        self.char_index += 1;
                    }
                }
                return self.prepare_result(next_token);
            }
        }
        None
    }
}

impl Tokenizer {
    pub fn new(program_path: &str) -> io::Result<Tokenizer> {
        Ok(Tokenizer {
            scanned_program: fs::read_to_string(program_path)?.chars().collect(),
            char_index: 0,
            line_number: 1
        })
    }

    pub fn prepare_result(&self, token: String) -> Option<(String, u32)> {
        Some((token, self.line_number))
    }

    fn read_delimited_constant(&mut self) -> String {
        let delimiter = self.scanned_program[self.char_index];
        let mut constant = String::from(delimiter);
        self.char_index += 1;
        while let Some(char) = self.scanned_program.get(self.char_index) {
            constant.push(*char);
            self.char_index += 1;
            if *char == delimiter {
                break
            }
        }
        constant
    }
}