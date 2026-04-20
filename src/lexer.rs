pub mod errors;
pub mod token;

use errors::LexerError;
use token::{Token, TokenType};
const DEFAULT_CAPACITY: usize = 4 * 1024; //4kb(page size)

pub struct Lexer {
    line: usize,
    col: usize,
    start_of_token: usize,
    idx: usize,
    input: Vec<char>,
    eof_yielded: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            start_of_token: 0,
            idx: 0,
            input: Vec::<char>::with_capacity(DEFAULT_CAPACITY),
            eof_yielded: false,
        }
    }

    pub fn load(&mut self, input: &str) {
        self.input.clear();
        self.input.extend(input.chars());

        self.idx = 0;
        self.line = 0;
        self.col = 0;
        self.start_of_token = 0;
        self.eof_yielded = false;
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

    fn read_number(&mut self) -> Result<TokenType, LexerError> {
        let mut start_idx = self.idx;
        let mut is_hex: bool = false;

        if self.peek() == Some('0') {
            self.advance();
            if self.peek() == Some('x') {
                self.advance();
                is_hex = true;
                start_idx = self.idx;
            }
        }
        while let Some(ch) = self.peek() {
            if is_hex {
                if ch.is_ascii_hexdigit() {
                    self.advance();
                } else {
                    break;
                }
            } else {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        if self.peek() == Some('.') {
            if is_hex {
                return Err(LexerError::InvalidNumberFormat);
            }
            self.advance();
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
            return Ok(TokenType::FloatLiteral(
                self.input[start_idx..self.idx].iter().collect(),
            ));
        }
        if is_hex {
            Ok(TokenType::IntLiteralHex(
                self.input[start_idx..self.idx].iter().collect(),
            ))
        } else {
            Ok(TokenType::IntLiteralDec(
                self.input[start_idx..self.idx].iter().collect(),
            ))
        }
    }

    fn read_identifier(&mut self) -> TokenType {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
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
            "continue" => TokenType::Continue,
            "break" => TokenType::Break,
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
            "true" => TokenType::BooleanLiteral(true),
            "false" => TokenType::BooleanLiteral(false),
            _ => TokenType::Identifier(lexeme),
        }
    }

    fn read_string(&mut self) -> Result<TokenType, LexerError> {
        let mut buffer = String::new();
        self.advance();

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(TokenType::StringLiteral(buffer));
                }
                '\\' => {
                    self.advance();
                    match self.peek() {
                        Some('"') => buffer.push('"'),
                        Some('n') => buffer.push('\n'),
                        Some('t') => buffer.push('\t'),
                        Some('0') => buffer.push('\0'),
                        Some('\\') => buffer.push('\\'),
                        _ => return Err(LexerError::InvalidEscape),
                    }
                    self.advance();
                }
                '\n' => return Err(LexerError::UnterminatedString),
                _ => {
                    buffer.push(ch);
                    self.advance();
                }
            }
        }
        Err(LexerError::UnterminatedString)
    }

    fn make_token(&self, kind: TokenType) -> Token {
        let lexeme: String = self.input[self.start_of_token..self.idx].iter().collect();

        Token::new(lexeme, kind, self.line, self.col)
    }

    pub fn next_token(&mut self) -> Option<Result<Token, LexerError>> {
        self.skip_whitespace();
        self.start_of_token = self.idx;

        match self.peek() {
            None => Some(Ok(self.make_token(TokenType::EOF))),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                let kind = self.read_identifier();
                Some(Ok(self.make_token(kind)))
            }
            Some(ch) if ch.is_ascii_digit() => {
                Some(self.read_number().map(|kind| self.make_token(kind)))
            }
            Some('"') => Some(self.read_string().map(|kind| self.make_token(kind))),
            Some('+') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Plus)))
            }
            Some('*') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Star)))
            }
            Some('-') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Minus)))
            }
            Some('%') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Mod)))
            }
            Some('@') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::At)))
            }
            Some('#') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Hash)))
            }
            Some('~') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::BitNot)))
            }
            Some('^') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::BitXor)))
            }
            Some(':') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Colon)))
            }
            Some(';') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Semicolon)))
            }
            Some('(') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::LParen)))
            }
            Some(')') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::RParen)))
            }
            Some('[') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::LBrack)))
            }
            Some(']') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::RBrack)))
            }
            Some('{') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::LBrace)))
            }
            Some('}') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::RBrace)))
            }
            Some(',') => {
                self.advance();
                Some(Ok(self.make_token(TokenType::Comma)))
            }
            Some('/') => {
                self.advance();
                if self.peek() == Some('/') {
                    //comment
                    while self.peek() != Some('\n') && self.peek() != None {
                        self.advance();
                    }
                    None
                } else {
                    Some(Ok(self.make_token(TokenType::Slash)))
                }
            }
            Some('&') => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::And)));
                }
                Some(Ok(self.make_token(TokenType::BitAnd)))
            }
            Some('|') => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::Or)));
                }
                Some(Ok(self.make_token(TokenType::BitOr)))
            }
            Some('!') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::Neq)));
                }
                Some(Ok(self.make_token(TokenType::Not)))
            }
            Some('=') => {
                self.advance();
                if self.peek() == Some('=') {
                    //comparasion
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::Eq)));
                }
                Some(Ok(self.make_token(TokenType::Assign)))
            }
            Some('>') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::Gte)));
                }
                Some(Ok(self.make_token(TokenType::Gt)))
            }
            Some('<') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    return Some(Ok(self.make_token(TokenType::Lte)));
                }
                Some(Ok(self.make_token(TokenType::Lt)))
            }
            Some(c) => Some(Err(LexerError::InvalidCharacter(c))),
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_yielded {
            return None;
        }

        loop {
            match self.next_token() {
                None => continue,

                Some(res) => {
                    if let Ok(token) = &res {
                        if token.eof() {
                            self.eof_yielded = true
                        }
                    }

                    return Some(res);
                }
            }
        }
    }
}
