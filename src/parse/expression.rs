use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::lex::Token;
use crate::parse::AstNode;
use crate::parse::TokenIter;

pub fn expression(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    //! An expression can be expressed by the following grammar structures:
    //! variable, literal, procedure call, lambda expression, conditional, assignment, derived expression, macro use, or macro block.
    if let Some(token) = iter.peek() {
        match &token {
            Token::NonKeyword(value) => {
                // variable expressions are always going to start with Token::NonKeyword
                todo!("write the code path for variable expressions");
            }
            Token::Quote => {
                iter.next(); // consume the quotation
                quotation_literal(node_pool, iter)
            }
            Token::Boolean(_) | Token::Numeric(_) | Token::String(_) => {
                evaluated_literal(node_pool, iter)
            }
            Token::LeftParen => {
                // lambda expressions are always going to start with Token::LParen Token::Keyword("lambda")
                todo!("write the code path for lambda expressions");
                // conditional expressions are always going to start with Token::LParen Token::Keyword("if")
                todo!("write the code path for conditional expressions");
                // assignment expressions are always going to start with Token::LParen Token::Keyword("set!")
                todo!("write the code path for assignment expressions");
                // derived expressions are quite complicated and need a better understanding of quasinotation
                todo!("get a better understanding of quasinotation");
                todo!(
                    "better describe the requirements for an expression to be a derived expression"
                );
                todo!("write the code path for derived expressions");
                // macro use expressions are always going to start with Token::LParen and then follow with a Token::Keyword, which makes the logic difficult
                todo!("better describe the requirements for an expression to be a macro use expression");
                todo!("write the code path for macro use expressions");
                // macro block expressions are always going to start with Token::LParen and then either Token::Keyword("let-syntax") or Token::Keyword("letrec-syntax")
                todo!("write the code path for macro block expressions");
                // procedure calls are always going to start with Token::LParen. this puts its priority lower than lambda expressions
                todo!("write the code path for procedure calls");
            }
            _ => {
                todo!("write the code path for handling unexpected tokens here");
            }
        }
    } else {
        todo!("write appropriate error types for unexpected end of file");
    }
}

fn variable_expresion(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    if let Some(token) = iter.next() {
        let node = match token {
            Token::NonKeyword(name) => AstNode::Variable {
                value: name.clone(),
            },
            _ => todo!("never reach this path, which means we should rewrite this"),
        };
        return node_pool.add(node);
    }
    todo!("handle code path for hitting variable unexpectedly")
}

fn quotation_literal(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    //! A quotation literal.
    todo!("write the document comment for this function");
    todo!("write the code path for handling short-hand quotation literals");
    todo!("write teh code path for long hand qutation literals, ensuring the consumption of the closing parenthesis");
}

fn evaluated_literal(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    //! Evaluated literals are exactly as they are described in the token
    if let Some(token) = iter.next() {
        let node = match token {
            Token::Numeric(number) => AstNode::Number {
                value: number.clone(),
            },
            Token::Boolean(boolean) => AstNode::Boolean { value: *boolean },
            Token::String(string) => AstNode::String {
                value: string.clone(),
            },
            _ => todo!("handle the code path for unexpected non-literal token"),
        };
        return node_pool.add(node);
    }
    todo!("handle code path for hitting end of token stream instead of a literal");
}
