use std::{env, process};

use mee_core::{
    tokenizer::{Tokenize, Tokenizer},
    *,
};

fn main() {
    let expr = env::args().nth(1).unwrap();

    let tokenizer = Tokenizer {};
    let tokens = tokenizer.tokenize(&expr);

    validator::validate(&tokens).unwrap_or_else(|err| {
        eprintln!("\nan error occured: {err}\n");
        process::exit(1);
    });

    let ast = parser::parse(&tokens);

    let result = evaluator::evaluate(&ast);

    println!("{expr} = {result}");
}
