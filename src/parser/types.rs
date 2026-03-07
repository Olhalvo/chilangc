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
pub enum LiteralType {
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub enum BinaryOps{

}

pub enum UnaryOps{

}

pub enum PostfixOps{

}