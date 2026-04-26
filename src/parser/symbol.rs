use std::collections::hash_map::*;

use crate::parser::{errors::ParserError, types::Type};

pub enum SymbolKind{
    Variable,
    Function,
}

pub struct Symbol{
    pub name : String,
    pub symbol_type : Type,
    pub kind: SymbolKind,
}


pub struct SymbolTable{
    scopes: Vec<HashMap<String,Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self{
        Self{
            scopes : vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self){
        self.scopes.push(HashMap::new())
    }

    pub fn pop_scope(&mut self) -> Result<(),ParserError>{
        self.scopes
        .pop()
        .map(|_|())
        .ok_or(ParserError::InvalidScopeError(self.scopes.len()))
    }


    pub fn define(&mut self, name: String, symbol_type : Type, kind : SymbolKind) -> Result<(),ParserError>{
        let scope = self.scopes.last_mut()
        .expect("Compiler trying to define an array outside any scope");

        match scope.entry(name){
            Entry::Occupied(entry) =>
                Err(ParserError::SymbolAlreadyDefined(entry.key().clone())),
            Entry::Vacant(entry) =>{
                let key_clone = entry.key().clone();
                entry.insert(
                    Symbol{
                        name : key_clone,
                        symbol_type : symbol_type,
                        kind : kind,
                    }
                );
                Ok(())
            },
        }
    }
}