use std::collections::HashMap;
use crate::parser::types::Type;

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
        Self(vec![HashMap::new()])
    }

    pub fn push_scope(&mut self){
        self.scopes.push(HashMap::new())
    }

}