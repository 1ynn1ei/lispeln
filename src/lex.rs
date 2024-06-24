use crate::stream::Stream;
use crate::def::grammar;
#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Quote,
    Comment(String),
    Boolean(bool),
    String(String),
    Numeric(String),
    Whitespace,
    EndOfFile
}

pub fn generate_token(stream: &mut Stream) -> Token {
    if stream.is_eof() {
	Token::EndOfFile
    } else {
	let current_symbol = stream.current();
	match current_symbol {
	    b'0'..=b'9' => grammar::numeric_literal(stream),
	    b'(' => { stream.step(); Token::LeftParen},
	    b')' => { stream.step(); Token::RightParen},
	    b'\'' => { stream.step(); Token::Quote},
	    
	    _ => todo!()
	}
	
    }
}
