use mee_core::{evaluator, parser, tokenizer};

#[test]
pub fn check_if_evaluator_works() {
    let expr = "2 + 2 * 2";
    let tokens = tokenizer::tokenize(expr);
    let ast = parser::parse(&tokens);

    let result = evaluator::evaluate(&ast);

    assert_eq!(result, 6f64);
}
