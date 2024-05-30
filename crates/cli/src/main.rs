use std::env;

use mee_core::*;

fn main() {
    let expr = env::args().nth(1).unwrap();

    let tokens = tokenizer::tokenize(&expr);
    let ast = parser::parse(&tokens);

    let result = evaluator::evaluate(&ast);

    println!("{expr} = {result}");
}
