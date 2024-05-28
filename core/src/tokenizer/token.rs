#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown(char),
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operation(Operation),
    LeftParenthesis,
    RightParenthesis,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Operation::Add,
            '-' => Operation::Subtract,
            '*' => Operation::Multiply,
            '/' => Operation::Divide,
            other => Operation::Unknown(other),
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            c if char::is_numeric(c) => Token::Number(c.to_string().parse::<f64>().unwrap()),
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            op => Token::Operation(Operation::from(op)),
        }
    }
}
