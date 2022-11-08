use crate::program_internal_form::ProgramInternalForm;
use crate::symbol_table::SymbolTable;
use crate::{PredefinedTokens, Tokenizer};
use crate::classifier::{classify, LexicalError};
use crate::finite_automata::FiniteAutomaton;

pub fn scan(tokenizer: Tokenizer, predefined_tokens: PredefinedTokens,
            identifier_fa: &FiniteAutomaton, integer_constant_fa: &FiniteAutomaton)
    -> Result<(ProgramInternalForm, SymbolTable<'static>), LexicalError> {
    let mut program_internal_form = ProgramInternalForm::new();
    let mut symbol_table = SymbolTable::new();

    for (token, line) in tokenizer {
        let class = match classify(&token, &predefined_tokens, identifier_fa, integer_constant_fa) {
            Err(_) => return Err(LexicalError::new(
                format!("Lexical error at line {}, token: {}", line, token))),
            Ok(class) => class
        };
        if (class == 0 || class == 1) && symbol_table.search(&token).is_none() {
            symbol_table.insert(&token);
        }
        program_internal_form.add(&token, class, symbol_table.search(&token))
    }

    Ok((program_internal_form, symbol_table))
}