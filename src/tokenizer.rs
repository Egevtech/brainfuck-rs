use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,

    Dot,
    D,

    LeftAngleBracket,
    RightAngleBracket,

    LeftSquareBracket,
    RightSquareBracket,
}

#[derive(Debug)]
pub enum TokenizerError {
    UnexpectedToken(char),
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::UnexpectedToken(c) => {
                write!(f, "Unexpected token: {}", c)
            }
        }
    }
}
impl std::error::Error for TokenizerError {}

pub type Tokens = Vec<Token>;

pub fn tokenize(str: String) -> Result<Tokens, TokenizerError> {
    let mut tokens = vec![];

    let ftokens = str.chars().into_iter().filter(|s| !s.is_whitespace());

    for token in ftokens {
        tokens.push(match token {
            '+' => Token::Plus,
            '-' => Token::Minus,

            'D' => Token::D,
            '.' => Token::Dot,

            '<' => Token::LeftAngleBracket,
            '>' => Token::RightAngleBracket,

            '[' => Token::LeftSquareBracket,
            ']' => Token::RightSquareBracket,

            _ => return Err(TokenizerError::UnexpectedToken(token)),
        });
    }

    Ok(tokens)
}
