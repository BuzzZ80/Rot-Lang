mod lexer;
mod node;
mod parser;
mod runtime;

use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input filename supplied");
        return Ok(());
    }

    let source = fs::read_to_string(&args[1])?;

    runtime::execute(parser::parse(lexer::Lexer::new(&source)).unwrap());

    Ok(())
}
