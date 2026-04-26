use crate::lexer::token::{Token};

#[derive(Debug)]
pub enum ParserError{
    UnexpectedToken(Token),  
    InvalidScopeError(usize),
    SymbolAlreadyDefined(String),
    NoSuchSymbolError(String),
}