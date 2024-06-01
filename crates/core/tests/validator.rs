use mee_core::{
    tokenizer::{Tokenize, Tokenizer},
    validator::{self, error::ValidationError},
};

#[test]
pub fn check_if_validator_works() {
    let expr = "1 + 6 / 2";
    let tokenizer = Tokenizer {};
    let tokens = tokenizer.tokenize(expr);

    assert!(validator::validate(&tokens).is_ok());

    let expr = "1 + 6 / 2 -";
    let tokens = tokenizer.tokenize(expr);

    let validation = validator::validate(&tokens);

    assert!(validation.is_err());
    assert!(matches!(
        validation.unwrap_err(),
        ValidationError::SyntaxError
    ));

    let expr = "1 + 6 p 2 -";
    let tokens = tokenizer.tokenize(expr);

    let validation = validator::validate(&tokens);

    assert!(validation.is_err());
    assert!(matches!(
        validation.unwrap_err(),
        ValidationError::InvalidOperation('p')
    ));

    let expr = "(1 + 6) / ((2 - 5)";
    let tokens = tokenizer.tokenize(expr);

    let validation = validator::validate(&tokens);

    println!("{validation:?}");

    assert!(validation.is_err());
    assert!(matches!(
        validation.unwrap_err(),
        ValidationError::InvalidParentheses
    ));
}
