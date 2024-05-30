use mee_core::{
    parser::{self, parse},
    tokenizer::{self, token::Operation, tokenize},
};

#[test]
pub fn check_if_parser_works() {
    let expr = "1 * 2 + 3";
    let tokens = tokenize(expr);

    let ast = parse(&tokens);

    println!("{ast:#?}");

    assert!(ast.value().as_operation().is_some());
    assert!(matches!(
        ast.value().as_operation().unwrap(),
        Operation::Add
    ));

    let right = ast.right();
    assert!(right.is_some());

    let right = right.unwrap();

    assert!(right.value().as_number().is_some());
    assert_eq!(right.value().as_number().unwrap(), 3f64);

    assert!(right.left().is_none());
    assert!(right.right().is_none());

    let left = ast.left();
    assert!(left.is_some());

    let left = left.unwrap();
    assert!(left.value().as_operation().is_some());
    assert!(matches!(
        left.value().as_operation().unwrap(),
        Operation::Multiply
    ));

    let left_left = left.left();
    assert!(left_left.is_some());

    let left_left = left_left.unwrap();
    assert!(left_left.value().as_number().is_some());
    assert_eq!(left_left.value().as_number().unwrap(), 1f64);

    let left_right = left.right();
    assert!(left_right.is_some());

    let left_right = left_right.unwrap();
    assert!(left_right.value().as_number().is_some());
    assert_eq!(left_right.value().as_number().unwrap(), 2f64);
}

#[test]
pub fn check_if_parentheses_parsed() {
    let expr = "5 * (1 + 2)";
    let tokens = tokenizer::tokenize(expr);

    let ast = parser::parse(&tokens);

    println!("{ast:#?}");

    assert!(ast.value().as_operation().is_some());
    assert!(matches!(
        ast.value().as_operation().unwrap(),
        Operation::Multiply
    ));

    let left = ast.left();

    assert!(left.is_some());
    let left = left.unwrap();

    assert!(left.value().as_number().is_some());
    assert_eq!(left.value().as_number().unwrap(), 5f64);

    let right = ast.right();

    assert!(right.is_some());
    let right = right.unwrap();

    assert!(right.value().as_operation().is_some());
    assert!(matches!(
        right.value().as_operation().unwrap(),
        Operation::Add
    ));

    let right_left = right.left();

    assert!(right_left.is_some());
    let right_left = right_left.unwrap();

    assert!(right_left.value().as_number().is_some());
    assert_eq!(right_left.value().as_number().unwrap(), 1f64);

    let right_right = right.right();

    assert!(right_right.is_some());
    let right_right = right_right.unwrap();

    assert!(right_right.value().as_number().is_some());
    assert_eq!(right_right.value().as_number().unwrap(), 2f64);
}
