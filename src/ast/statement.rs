use crate::token::Token;

use super::{ExprNode, FnDecl, ForLoop, Ident, Match, WhileLoop};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprNode),
    Block(Vec<Stmt>),
    If(ExprNode, Box<Stmt>),
    Match(Match),
    VarDecl(Ident, Option<ExprNode>),
    FnDecl(FnDecl),
    Print(ExprNode),
    For(Box<ForLoop>),
    Break(Token),
    While(Box<WhileLoop>),
    Return(Option<ExprNode>),
}

impl From<Stmt> for Vec<Stmt> {
    fn from(s: Stmt) -> Self {
        match s {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        }
    }
}
