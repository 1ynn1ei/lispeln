mod expression;
mod printer;
use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::lex::Token;
use multipeek::multipeek;
pub type TokenIter<'a> = multipeek::MultiPeek<std::vec::IntoIter<&'a Token>>;
pub use printer::print_node;

pub enum AstNode {
    Program { body: Vec<ArenaRef> },
    Boolean(bool),
    String(String),
    Number(String),
    Variable(String),
}

pub fn make_tree(node_pool: &mut Arena<AstNode>, tokens: Vec<Token>) -> ArenaRef {
    let mut iter = tokens
        .iter()
        .filter(|token| !matches!(token, Token::Whitespace))
        .collect::<Vec<_>>();
    let mut iter = multipeek(iter);
    let mut program = AstNode::Program {
        body: vec![command_or_definition(node_pool, &mut iter)],
    };
    node_pool.add(program)
}

fn command_or_definition(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    //! a program has either commands, or definitions within it.
    //! a command is essentially an expression.
    return expression::expression(node_pool, iter);
    todo!("implement definitions as well");
}
fn definition(node_pool: &mut Arena<AstNode>, iter: &mut TokenIter) -> ArenaRef {
    todo!("write out the code path for definitions");
}
