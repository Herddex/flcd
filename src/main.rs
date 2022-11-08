mod symbol_table;
mod classifier;
mod predefined_tokens;
mod tokenizer;
mod scanner;
mod program_internal_form;
mod finite_automata;
mod compiler;

use std::{env};
use crate::tokenizer::Tokenizer;
use crate::predefined_tokens::PredefinedTokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide command line arguments:");
        eprintln!("fa <path-to-fa-file>: FA mode");
        eprintln!("compile <path-to-source-file> : Compiler mode");
        return
    }

    let mode = &args[1];
    match mode.as_str() {
        "compile" => compiler::compile(&args[2]),
        "fa" => finite_automata::read_and_print_finite_automaton(&args[2]),
        _ => eprintln!("Mode not supported.")
    }
}
