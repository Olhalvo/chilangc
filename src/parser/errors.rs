use lexer::token::{Token};

#[derive(Debug)]
pub enum ParserError{
    UnexpectedToken(Token)  
}