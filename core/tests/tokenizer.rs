use mee_core::tokenizer::{
    token::{Operation, Token},
    tokenize,
};

#[test]
pub fn check_if_working() {
    let input = "1 + 2";
    let result = tokenize(input);

    println!("{result:#?}");

    assert_eq!(result.len(), 3);
    match result.first().unwrap() {
        Token::Number(num) => assert_eq!(*num, 1.),
        _ => panic!("wrong token type"),
    }

    match result.get(1).unwrap() {
        Token::Operation(op) => assert!(matches!(op, Operation::Add)),
        _ => panic!("wrong token type"),
    }

    match result.get(2).unwrap() {
        Token::Number(num) => assert_eq!(*num, 2.),
        _ => panic!("wrong token type"),
    }
}
