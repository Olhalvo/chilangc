pub mod token;
pub mod errors;
use token::{Token, TokenType};
use crate::lexer::errors::LexerError;
const DEFAULT_CAPACITY: usize = 4 * 1024; //4kb(page size)



pub struct Lexer {
    line: usize,
    col: usize,
    start_of_token: usize,
    idx: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            start_of_token: 0,
            idx: 0,
            input: Vec::<char>::with_capacity(DEFAULT_CAPACITY),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.idx).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.idx += 1;

        if ch == '\n' {
            self.col = 0;
            self.line += 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<TokenType, LexerError>{
        let start_idx = self.idx;
        return Err(LexerError::InvalidNumberFormat)
    }

    fn read_identifier(&mut self) -> TokenType {
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme: String = self.input[self.start_of_token..self.idx].iter().collect();

        match lexeme.as_str() {
            "var" => TokenType::Var,
            "defun" => TokenType::Defun,
            "if" => TokenType::If,
            "elif" => TokenType::Elif,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "ret" => TokenType::Ret,
            "i8" => TokenType::TypeI8,
            "u8" => TokenType::TypeU8,
            "i16" => TokenType::TypeI16,
            "u16" => TokenType::TypeU16,
            "i32" => TokenType::TypeI32,
            "u32" => TokenType::TypeU32,
            "i64" => TokenType::TypeI64,
            "u64" => TokenType::TypeU64,
            "f32" => TokenType::TypeF32,
            "f64" => TokenType::TypeF64,
            "ptr" => TokenType::TypePtr,
            "void" => TokenType::TypeVoid,
            "bool" => TokenType::TypeBool,
            _ => TokenType::Identifier(lexeme),
        }
    }

    fn make_token(&self, kind: TokenType) -> Token {
        let lexeme: String = self.input[self.start_of_token..self.idx].iter().collect();

        Token::new(lexeme, kind, self.line, self.col)
    }

    fn next_token(&mut self) -> Option<Result<Token, LexerError>>{
        self.skip_whitespace();
        self.start_of_token = self.idx;

        match self.peek() {
            None =>{
                Some(Ok(self.make_token(TokenType::EOF)))
            },
            Some(ch) if ch.is_alphabetic() || ch=='_' =>{
                let kind = self.read_identifier();
                Some(Ok(self.make_token(kind)))
            },
            Some(ch) if ch.is_numeric() =>{
                Some(self.read_number().map(|kind| self.make_token(kind))) 
            }
            Some('+') =>{
                self.advance();
                Some(Ok(self.make_token(TokenType::Plus)))
            },
            Some('*') =>{
                self.advance();
                Some(Ok(self.make_token(TokenType::Star)))
            },
            Some('-') =>{
                self.advance();
                Some(Ok(self.make_token(TokenType::Minus)))
            },
            Some('%') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Mod)))
            },
            Some('@') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::At)))
            },
            Some('#') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Hash)))
            },
            Some('~') =>{
                self.advance();
                Some(Ok(self.make_token(TokenType::BitNot)))
            },
            Some('^') =>{
                self.advance();
                Some(Ok(self.make_token(TokenType::BitXor)))
            },
            Some('/') =>{
                self.advance();
                if self.peek() == Some('/') {//comment
                    while self.peek() != Some('\n') && self.peek() != None {
                        self.advance();
                    }
                    None
                }else{
                    Some(Ok(self.make_token(TokenType::Slash)))
                }
            }
            Some(c) =>{
                Some(Err(LexerError::InvalidCharacter(c)))
            }
        }
    }
}
