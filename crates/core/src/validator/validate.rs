use crate::tokenizer::token::{Operation, Parenthesis, Token};

use super::error::ValidationError;

pub fn validate(tokens: &[Token]) -> Result<(), ValidationError> {
    let mut par_counter = 0;

    println!("{tokens:?} (length: {})", tokens.len());

    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::Parenthesis(par) => match par {
                Parenthesis::Left => par_counter += 1,
                Parenthesis::Right => par_counter -= 1,
            },
            Token::Operation(op) => match op {
                Operation::Unknown(unknown) => {
                    return Err(ValidationError::InvalidOperation(*unknown));
                }
                _any => {
                    if idx == 0 || idx == tokens.len() - 1 {
                        println!("syntax error: {token:?} at {idx} (total: {})", tokens.len());
                        return Err(ValidationError::SyntaxError);
                    }

                    if (tokens[idx - 1].as_number().is_none()
                        || tokens[idx + 1].as_number().is_none())
                        && tokens[idx - 1].as_parenthesis().is_none()
                        && tokens[idx + 1].as_parenthesis().is_none()
                    {
                        println!("operator: ");
                        return Err(ValidationError::SyntaxError);
                    }
                }
            },
            _ => (),
        }
    }

    if par_counter != 0 {
        return Err(ValidationError::InvalidParentheses);
    }

    Ok(())
}
