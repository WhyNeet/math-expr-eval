use mee_core::tokenizer::{
    token::{Operation, Token},
    Tokenize, Tokenizer,
};

#[test]
pub fn check_if_tokenizer_works() {
    let expr = "1 + 2";
    let tokenizer = Tokenizer {};
    let tokens = tokenizer.tokenize(expr);

    println!("{tokens:#?}");

    assert_eq!(tokens.len(), 3);
    match tokens.first().unwrap() {
        Token::Number(num) => assert_eq!(*num, 1.),
        _ => panic!("wrong token type"),
    }

    match tokens.get(1).unwrap() {
        Token::Operation(op) => assert!(matches!(op, Operation::Add)),
        _ => panic!("wrong token type"),
    }

    match tokens.get(2).unwrap() {
        Token::Number(num) => assert_eq!(*num, 2.),
        _ => panic!("wrong token type"),
    }
}
