mod expression;
use crate::lex::Token;
pub type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, &'a Token>>;

pub enum AstNode {
    Boolean(bool),
    String(String),
}
