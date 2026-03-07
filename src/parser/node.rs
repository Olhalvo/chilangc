use crate::parser::types::{BinaryOps, LiteralType, PostfixOps, Type, UnaryOps};

pub struct Ast{
    procedures : Vec<AstNode>
}
pub enum AstNode{
    //Declaration 
    FuncDecl {
        name: String,
        ret_type: Type,
        params : Vec<(String, Type)>,
        body : Vec<AstNode>
    },
    VarDecl{
        name : String,
        var_type : Type,
        value : Option<Box<AstNode>>,
    },
    //Statements
    While{
        condition : Box<AstNode>,
        block : Vec<AstNode>,
    },
    If{
        condition : Box<AstNode>,
        block :  Vec<AstNode>,
        elifs :  Vec<AstNode>,
        else_block : Option<Vec<AstNode>>
    },
    Elif{
        condition : Box<AstNode>,
        block : Vec<AstNode>,
    },
    Ret{
        val : Box<AstNode>
    },
    BinaryOpr{
        left : Box<AstNode>,
        op : BinaryOps,
        right : Box<AstNode>,
    },
    UnaryOpr{
        op: UnaryOps,
        operand : Box<AstNode>,
    }, 
    PostfixOpr{
        op: PostfixOps,
        operand : Box<AstNode>,
    },
    Literal(LiteralType),
    Identifier(String),
    Call{
        called: String,
        args : Vec<AstNode>
    },
    Cast{
        target: Type,
        expr : Box<AstNode>        
    }
}



impl AstNode {
    pub fn is_function(&self) -> bool{
        matches!(self, AstNode::FuncDecl{..})
    }
}