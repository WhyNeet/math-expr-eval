use super::token::{Operation, Parenthesis, Token};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let input = input.replace(' ', "");
    let input = input.chars();
    let mut number_acc = String::new();

    for val in input {
        match val {
            c if c.is_numeric() || (c == '.' && !number_acc.is_empty()) => {
                number_acc.push(c);
                continue;
            }
            parenthesis if parenthesis == '(' || parenthesis == ')' => {
                if !number_acc.is_empty() {
                    result.push(Token::Number(number_acc.parse().unwrap()));
                    number_acc = String::new();
                }

                result.push(Token::Parenthesis(Parenthesis::from(parenthesis)));
            }
            operation => {
                if !number_acc.is_empty() {
                    result.push(Token::Number(number_acc.parse().unwrap()));
                    number_acc = String::new();
                }

                result.push(Token::Operation(Operation::from(operation)));
            }
        }
    }

    if !number_acc.is_empty() {
        result.push(Token::Number(number_acc.parse().unwrap()));
    }

    result
}
