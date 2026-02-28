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
    TypeArray, //Strings ought to be arrays of i8;
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
    Dereference, //@ operator
    //Ponctuation
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
}

impl Token {
    pub fn new(lexem: String, kind: TokenType, line: usize, col: usize) -> Self {
        return Self {
            lexem,
            kind,
            span: Span { line, col },
        };
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
