use mee_core::{
    evaluator, parser,
    tokenizer::{Tokenize, Tokenizer},
};

#[test]
pub fn check_if_evaluator_works() {
    let expr = "2 + 2 * 2";

    let tokenizer = Tokenizer {};
    let tokens = tokenizer.tokenize(expr);
    let ast = parser::parse(&tokens);

    let result = evaluator::evaluate(&ast);

    assert_eq!(result, 6f64);

    let expr = "((10 - 3) * (4 + 2) / (5 - 2)) - ((8 + 7) / (3 * 2)) + ((9 * 4) - (6 / 2)) * ((12 / 4) + (5 * 3)) - ((15 + 5) / (8 - 4))";
    let tokens = tokenizer.tokenize(expr);
    let ast = parser::parse(&tokens);

    let result = evaluator::evaluate(&ast);

    assert_eq!(result, 600.5);
}
