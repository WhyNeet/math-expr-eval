use mee_core::{
    parser::parse,
    tokenizer::{token::Operation, tokenize},
};

#[test]
pub fn check_if_parser_works() {
    let expr = "1 * 2 + 3";
    let tokens = tokenize(expr);

    let parsed = parse(&tokens);

    println!("{parsed:#?}");

    assert!(parsed.value().get_operation().is_some());
    assert!(matches!(
        parsed.value().get_operation().unwrap(),
        Operation::Add
    ));

    let right = parsed.right();
    assert!(right.is_some());

    let right = right.unwrap();

    assert!(right.value().get_number().is_some());
    assert_eq!(right.value().get_number().unwrap(), 3f64);

    assert!(right.left().is_none());
    assert!(right.right().is_none());

    let left = parsed.left();
    assert!(left.is_some());

    let left = left.unwrap();
    assert!(left.value().get_operation().is_some());
    assert!(matches!(
        left.value().get_operation().unwrap(),
        Operation::Multiply
    ));

    let left_left = left.left();
    assert!(left_left.is_some());

    let left_left = left_left.unwrap();
    assert!(left_left.value().get_number().is_some());
    assert_eq!(left_left.value().get_number().unwrap(), 1f64);

    let left_right = left.right();
    assert!(left_right.is_some());

    let left_right = left_right.unwrap();
    assert!(left_right.value().get_number().is_some());
    assert_eq!(left_right.value().get_number().unwrap(), 2f64);
}
