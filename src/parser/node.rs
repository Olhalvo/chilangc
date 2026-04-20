use crate::parser::types::{BinaryOps, LiteralValue, PostfixOps, Type, UnaryOps};
#[derive(Debug, Clone, PartialEq)]
pub struct Ast{
    nodes : Vec<AstNode>
}

#[derive(Debug,Clone,PartialEq)]
pub struct Elif{
    condition : Box<AstNode>,
    block : Vec<AstNode>,
}

#[derive(Debug,Clone,PartialEq)]
pub struct If{
    condition : Box<AstNode>,
    block :  Vec<AstNode>,
    elifs :  Vec<Elif>,
    else_block : Option<Vec<AstNode>>
}

#[derive(Debug,Clone,PartialEq)]
pub struct Function{
    name: String,
    ret_type: Type,
    params : Vec<Parameter>,
    body : Vec<AstNode>
}
#[derive(Debug,Clone,PartialEq)]
pub struct Parameter{
    pub name : String,
    pub param_type : Type
}


#[derive(Debug, Clone, PartialEq)]
pub enum AstNode{
    //Declaration 
    FuncDecl(Function),
    VarDecl{
        name : String,
        var_type : Type,
        value : Option<Box<AstNode>>,
    },
    //Statements7
    ExprStmt(Box<AstNode>),
    While{
        condition : Box<AstNode>,
        block : Vec<AstNode>,
    },
    Break,
    Continue,
    If(If),
    Elif(Elif),
    Ret(Option<Box<AstNode>>),

    //Expressions
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
    Literal(LiteralValue),
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
    pub fn is_variable(&self) -> bool{
        matches!(self, AstNode::VarDecl {..})
    }
    pub fn is_statement(&self) ->bool{
        matches!(self,
            AstNode::While{..} 
            | AstNode::If{..} 
            | AstNode::Elif{..}
            | AstNode::Ret {..}
            | AstNode::VarDecl {..}
            | AstNode::Break
            | AstNode::Continue
            | AstNode::ExprStmt(_)
        )  
    }
    pub fn is_expression(&self) ->bool{
        matches!(self,
            AstNode::BinaryOpr{..}
            | AstNode::UnaryOpr{..}
            | AstNode::PostfixOpr{..}
            | AstNode::Literal(_)
            | AstNode::Identifier(_)
            | AstNode::Call{..}
            | AstNode::Cast{..}
        )
    }
}