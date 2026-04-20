
use std::iter::Peekable;
use crate::lexer::token::Token;

pub mod node;
pub mod types;

pub struct Parser<I:Iterator<Item=Token>>{
    tokens : Peekable<I>
}