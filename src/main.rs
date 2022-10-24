mod symbol_table;
mod classifier;
mod predefined_tokens;
mod scanner;

use std::env;
use crate::classifier::Classifier;
use crate::scanner::Scanner;
use crate::predefined_tokens::PredefinedTokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the path to the input file as a command line argument.");
        return
    }

    let scanner = match Scanner::new(&args[1]) {
        Ok(scanner) => scanner,
        Err(error) => {
            println!("Failed to scan program file: {}", error);
            return
        }
    };
    dbg!(&scanner.scanned_tokens);

    let predefined_tokens = match PredefinedTokens::new("tokens.txt") {
        Ok(tokens) => tokens,
        Err(error) => {
            println!("Could not read predefined tokens from tokens.txt: {}", error);
            return
        }
    };
    let classifier = Classifier::new(&predefined_tokens);
    let symbol_table = classifier.classify(scanner.scanned_tokens);
    dbg!(&symbol_table);
}
