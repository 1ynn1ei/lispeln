use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::lex::Token;
use crate::parse::AstNode;
use crate::parse::TokenIter;

pub fn expression(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    //! An expression can be expressed by the following grammar structures:
    //! literal, procedure call, lambda expression, conditional, assignment, derived expression, macro use, or macro block.

    // literal expressions are always going to start with Token::Quote, Token::Boolean, Token::Numeric, or Token::String
    todo!("write the code path for literal expressions");
    // lambda expressions are always going to start with Token::LParen Token::Keyword("lambda")
    todo!("write the code path for lambda expressions");
    // conditional expressions are always going to start with Token::LParen Token::Keyword("if")
    todo!("write the code path for conditional expressions");
    // assignment expressions are always going to start with Token::LParen Token::Keyword("set!")
    todo!("write the code path for assignment expressions");
    // derived expressions are quite complicated and need a better understanding of quasinotation
    todo!("get a better understanding of quasinotation");
    todo!("better describe the requirements for an expression to be a derived expression");
    todo!("write the code path for derived expressions");
    // macro use expressions are always going to start with Token::LParen and then follow with a Token::Keyword, which makes the logic difficult
    todo!("better describe the requirements for an expression to be a macro use expression");
    todo!("write the code path for macro use expressions");
    // macro block expressions are always going to start with Token::LParen and then either Token::Keyword("let-syntax") or Token::Keyword("letrec-syntax")
    todo!("write the code path for macro block expressions");
    // procedure calls are always going to start with Token::LParen. this puts its priority lower than lambda expressions
    todo!("write the code path for procedure calls");
}
