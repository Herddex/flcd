use crate::symbol_table::SymbolTable;
use crate::predefined_tokens::PredefinedTokens;

pub struct Classifier<'a> {
    tokens: &'a PredefinedTokens
}

impl<'a> Classifier<'a> {
    pub fn new(tokens: &'a PredefinedTokens) -> Classifier {
        Classifier {
            tokens
        }
    }

    pub fn classify(&self, tokens: Vec<String>) -> SymbolTable {
        let mut symbol_table = SymbolTable::new();

        tokens.iter()
            .filter(|token| !self.tokens.contains(token))
            .for_each(|token| {
                if symbol_table.search(token).is_none() {
                    symbol_table.insert(token);
                }
            });

        symbol_table
    }
}
