use std::fs;
use crate::finite_automata::FiniteAutomaton;
use crate::predefined_tokens::PredefinedTokens;
use crate::scanner;
use crate::tokenizer::Tokenizer;

pub fn compile(path_to_input_file: &str) -> () {
    let tokenizer = match Tokenizer::new(path_to_input_file) {
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

    let identifier_fa = match FiniteAutomaton::new("identifier.fa") {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Failed to read identifier.fa: {}", error);
            return
        }
    };

    let integer_constant_fa = match FiniteAutomaton::new("integer_constant.fa") {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Failed to read integer_constant.fa: {}", error);
            return
        }
    };

    let pif_and_st = scanner::scan(tokenizer, predefined_tokens, &identifier_fa, &integer_constant_fa);
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