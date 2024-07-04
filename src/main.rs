#![allow(dead_code, unused)]
mod arena;
mod def;
mod lex;
mod parse;
mod stream;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("line: {}", &line);
                rl.add_history_entry(line.as_str())?;
                let mut node_pool = arena::Arena::<parse::AstNode>::default();
                let mut stream = stream::Stream::new(line.into_bytes());
                let mut tokens: Vec<lex::Token> = Vec::new();
                loop {
                    let token = lex::generate_token(&mut stream);
                    match token {
                        lex::Token::Whitespace => {}
                        _ => println!("[TOKEN: {:?}", token),
                    }
                    if let lex::Token::EndOfFile = token {
                        break;
                    }
                    tokens.push(token);
                }
                let expression_ref = parse::make_tree(&mut node_pool, tokens);
                println!("{}", parse::print_node(&node_pool, &expression_ref, 0));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Err: {}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}
