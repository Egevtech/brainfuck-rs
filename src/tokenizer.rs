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

    Comma(String),
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

    let ftokens = str
        .chars()
        .into_iter()
        .filter(|s| !s.is_whitespace())
        .collect::<Vec<char>>();

    let mut i = 0;
    while i < ftokens.len() {
        tokens.push(match ftokens[i] {
            '+' => Token::Plus,
            '-' => Token::Minus,

            'D' => Token::D,
            '.' => Token::Dot,

            '<' => Token::LeftAngleBracket,
            '>' => Token::RightAngleBracket,

            '[' => Token::LeftSquareBracket,
            ']' => Token::RightSquareBracket,

            '#' => {
                let mut c = String::new();

                i += 1;

                while ftokens[i] != ';' {
                    c.push(ftokens[i]);

                    i += 1;
                }
                Token::Comma(c)
            }

            _ => return Err(TokenizerError::UnexpectedToken(ftokens[i])),
        });

        i += 1;
    }

    Ok(tokens)
}
