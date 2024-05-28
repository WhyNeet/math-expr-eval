#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown(char),
}

#[derive(Debug)]
pub enum Parenthesis {
    Left,
    Right,
    Unknown(char),
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operation(Operation),
    Parenthesis(Parenthesis),
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

impl From<char> for Parenthesis {
    fn from(value: char) -> Self {
        match value {
            '(' => Parenthesis::Left,
            ')' => Parenthesis::Right,
            other => Parenthesis::Unknown(other),
        }
    }
}
