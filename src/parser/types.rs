use crate::parser::node::AstNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    I8,     U8,
    I16,    U16,
    I32,    U32,
    I64,    U64,
    Bool,   Void,
    Ptr(Box<Type>),
    Array(Box<Type>, usize)
}
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOps{
    Add,
    Sub,
    Mul,
    Div,
    Mod,    
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Assign
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum UnaryOps{
    Not,
    Neg, 
    BitNot,
    Ref
}
#[derive(Debug,Clone,PartialEq)]
pub enum PostfixOps{
    Deref,
    Index(Box<AstNode>)
}