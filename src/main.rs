mod symbol_table;
mod classifier;
mod predefined_tokens;
mod tokenizer;
mod scanner;
mod program_internal_form;

use std::{env, fs};
use crate::tokenizer::Tokenizer;
use crate::predefined_tokens::PredefinedTokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to the input file as a command line argument.");
        return
    }

    let tokenizer = match Tokenizer::new(&args[1]) {
        Ok(tokenizer) => tokenizer,
        Err(error) => {
            eprintln!("Failed to read program file: {}", error);
            return
        }
    };

    let predefined_tokens = match PredefinedTokens::new("tokens.txt") {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Failed to read tokens.txt: {}", error);
            return
        }
    };

    let pif_and_st = scanner::scan(tokenizer, predefined_tokens);
    match pif_and_st {
        Err(error) => {
            eprintln!("{}", error);
        },
        Ok(pif_and_st) => {
            println!("Lexically correct");
            let (program_internal_form, symbol_table) = pif_and_st;
            if let Err(error) = fs::write("PIF.out", format!("{}", program_internal_form)) {
                eprintln!("Failed to write PIF.out: {}", error)
            }
            if let Err(error) = fs::write("ST.out", format!("{}", symbol_table)) {
                eprintln!("Failed to write ST.out: {}", error)
            }
        }
    };
}
