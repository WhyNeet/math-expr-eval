use super::token::{Operation, Parenthesis, Token};

pub trait Tokenize {
    fn tokenize(&self, input: &str) -> Vec<Token>;
}

pub struct Tokenizer {}

impl Tokenize for Tokenizer {
    fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut result = Vec::new();

        let input = input.replace(' ', "");
        let mut input = input.chars().peekable();
        let mut number_acc = String::new();

        while let Some(val) = input.next() {
            match val {
                c if c.is_numeric() || (c == '.' && !number_acc.is_empty()) => {
                    number_acc.push(c);

                    if let Some(char) = input.peek() {
                        if !char.is_numeric() {
                            result.push(Token::Number(number_acc.parse().unwrap()));
                            number_acc.clear();
                        }
                    } else {
                        result.push(Token::Number(number_acc.parse().unwrap()));
                    }

                    continue;
                }
                '(' | ')' => {
                    result.push(Token::Parenthesis(Parenthesis::from(val)));
                }
                operation => {
                    result.push(Token::Operation(Operation::from(operation)));
                }
            }
        }

        result
    }
}
