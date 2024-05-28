use super::{token::Token, utils::is_operation};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let input = input.replace(' ', "");
    let input = input.chars();
    let mut number_acc = String::new();

    for val in input {
        match val {
            c if c.is_numeric() => {
                number_acc.push(c);
                continue;
            }
            c if is_operation(c) => {
                if !number_acc.is_empty() {
                    result.push(Token::Number(number_acc.parse().unwrap()));
                    number_acc = String::new();
                }

                result.push(Token::from(c));
            }
            other => {
                if !number_acc.is_empty() {
                    result.push(Token::Number(number_acc.parse().unwrap()));
                    number_acc = String::new();
                }

                result.push(Token::from(other));
            }
        }
    }

    if !number_acc.is_empty() {
        result.push(Token::Number(number_acc.parse().unwrap()));
    }

    result
}
