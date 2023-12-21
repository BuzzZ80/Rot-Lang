mod node;

use std::{
    env,
    fs,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input filename supplied");
        return Ok(());
    }

    let source = fs::read_to_string(&args[1])?;

    println!("{source}");

    Ok(())
}

