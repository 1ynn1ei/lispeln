use crate::def::grammar;
use crate::stream::Stream;
#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Quote,
    Comment(String),
    Boolean(bool),
    Keyword(String),
    NonKeyword(String),
    String(String),
    Numeric(String),
    Asterisk,
    ForwardSlash,
    Plus,
    Minus,
    Whitespace,
    EndOfFile,
}

pub fn generate_token(stream: &mut Stream) -> Token {
    if stream.is_eof() {
        Token::EndOfFile
    } else {
        let current_symbol = stream.current();
        match current_symbol {
            b' ' => {
                stream.step();
                Token::Whitespace
            }
            b'0'..=b'9' => grammar::numeric_literal(stream),
            b'+' => {
                stream.step();
                Token::Plus
            }
            b'-' => {
                stream.step();
                Token::Minus
            }
            b'\\' => {
                stream.step();
                Token::ForwardSlash
            }
            b'*' => {
                stream.step();
                Token::Asterisk
            }
            b'(' => {
                stream.step();
                Token::LeftParen
            }
            b')' => {
                stream.step();
                Token::RightParen
            }
            b'\'' => {
                stream.step();
                Token::Quote
            }
            _ => todo!(),
        }
    }
}
