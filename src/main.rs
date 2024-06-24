#![allow(dead_code, unused)]
mod stream;
mod arena;
mod def;
mod lex;
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
		let mut stream = stream::Stream::new(line.into_bytes());
		lex::generate_token(&mut stream);
	    },
	    Err(ReadlineError::Interrupted) => {
		println!("CTRL-C");
		break
	    },
	    Err(ReadlineError::Eof) => {
		println!("CTRL-D");
		break
	    },
	    Err(err) => {
		println!("Err: {}", err);
		break
	    }
	}
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}
