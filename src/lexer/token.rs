use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    lexem: String,
    kind: TokenType,
    span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Var,
    Defun,
    If,
    Elif,
    Else,
    While,
    Ret, //language Statements

    //Types
    TypeVoid,
    TypeBool,
    TypeI8,
    TypeU8,
    TypeI16,
    TypeU16,
    TypeI32,
    TypeU32,
    TypeI64,
    TypeU64,
    TypeF32,
    TypeF64,
    TypePtr,

    //Literals
    IntLiteralDec(String),
    IntLiteralHex(String),
    BooleanLiteral(bool),
    FloatLiteral(String),
    StringLiteral(String),

    Identifier(String), //Variable/Function names
    //Operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Assign,
    Eq,
    Neq,
    Gte,
    Lte,
    Lt,
    Gt,
    Not,
    Or,
    And,
    BitNot,
    BitOr,
    BitAnd,
    BitXor,
    At,   //@ operator (dereference suffix)
    Hash, //# operator (reference prefix)
    //Ponctuation
    Colon,
    Semicolon,
    LParen,
    LBrace,
    RParen,
    RBrace,
    LBrack,
    RBrack,
    Comma,

    EOF,
}

impl Token {
    pub fn new(lexem: String, kind: TokenType, line: usize, col: usize) -> Self {
        return Self {
            lexem,
            kind,
            span: Span { line, col },
        };
    }

    pub fn is_statement(&self) -> bool {
        matches!(
            self.kind,
            TokenType::Var
                | TokenType::Defun
                | TokenType::If
                | TokenType::Elif
                | TokenType::Else
                | TokenType::While
                | TokenType::Ret
        )
    }

    pub fn is_type(&self) -> bool {
        matches!(
            self.kind,
            TokenType::TypeVoid
                | TokenType::TypeBool
                | TokenType::TypeU8
                | TokenType::TypeI8
                | TokenType::TypeU16
                | TokenType::TypeI16
                | TokenType::TypeU32
                | TokenType::TypeI32
                | TokenType::TypeU64
                | TokenType::TypeI64
                | TokenType::TypePtr
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenType::IntLiteralDec(_)
                | TokenType::IntLiteralHex(_)
                | TokenType::BooleanLiteral(_)
                | TokenType::FloatLiteral(_)
                | TokenType::StringLiteral(_)
        )
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenType::Identifier(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Mod
                | TokenType::Assign
                | TokenType::Eq
                | TokenType::Neq
                | TokenType::Gte
                | TokenType::Lte
                | TokenType::Lt
                | TokenType::Gt
                | TokenType::Not
                | TokenType::Or
                | TokenType::And
                | TokenType::BitNot
                | TokenType::BitOr
                | TokenType::BitAnd
                | TokenType::BitXor
                | TokenType::At
                | TokenType::Hash
        )
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(
            self.kind,
            TokenType::Colon
                | TokenType::Semicolon
                | TokenType::LParen
                | TokenType::LBrace
                | TokenType::RParen
                | TokenType::RBrace
                | TokenType::LBrack
                | TokenType::RBrack
                | TokenType::Comma
        )
    }

    pub fn eof(&self) -> bool {
        matches!(self.kind, TokenType::EOF)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token({:?}, '{}', Line: {}, Col: {})",
            self.kind, self.lexem, self.span.line, self.span.col
        )
    }
}
