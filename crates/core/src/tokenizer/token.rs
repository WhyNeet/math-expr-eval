#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown(char),
    Sentinel,
}

#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    Left,
    Right,
    Unknown(char),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operation(Operation),
    Parenthesis(Parenthesis),
}

impl Token {
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Token::Number(num) => Some(*num),
            _ => None,
        }
    }
}

impl Operation {
    pub fn is_sentinel(&self) -> bool {
        matches!(self, Operation::Sentinel)
    }
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