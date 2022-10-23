use crate::symbol_table::SymbolTable;
use crate::tokens::Tokens;

pub struct Classifier<'a> {
    tokens: &'a Tokens
}

impl<'a> Classifier<'a> {
    pub fn new(tokens: &'a Tokens) -> Classifier {
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
