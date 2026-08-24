use crate::tokenizer::{Token, Tokens};

#[derive(PartialEq, Debug)]
pub enum OptToken {
    ParAdd(u32),
    ParSub(u32),

    ParMoveFor(u32),
    ParMoveBack(u32),

    ParDebug,
    ParPrint,

    ParLoopStart,
    ParLoopEnd,
}

pub type OptTokens = Vec<OptToken>;

pub trait Optimizations {
    fn optimize(self) -> OptTokens;
}

macro_rules! proc_opt_token {
    ($tok:expr, $i:expr, $repeats:expr, $self:expr) => {
        while $i + 1 < $self.len() && $self[$i + 1] == $tok {
            $repeats += 1;
            $i += 1;
        }
    };
}

impl Optimizations for Tokens {
    fn optimize(self) -> OptTokens {
        let mut ot: OptTokens = vec![];

        let mut i = 0;
        while i < self.len() {
            let current = || &self[i];

            ot.push(match current() {
                Token::D => OptToken::ParDebug,
                Token::Dot => OptToken::ParPrint,

                Token::LeftSquareBracket => OptToken::ParLoopStart,
                Token::RightSquareBracket => OptToken::ParLoopEnd,

                Token::Plus => {
                    let mut repeats = 1;
                    proc_opt_token!(Token::Plus, i, repeats, self);
                    OptToken::ParAdd(repeats)
                }
                Token::Minus => {
                    let mut repeats = 1;
                    proc_opt_token!(Token::Minus, i, repeats, self);
                    OptToken::ParSub(repeats)
                }
                Token::LeftAngleBracket => {
                    let mut repeats = 1;
                    proc_opt_token!(Token::LeftAngleBracket, i, repeats, self);
                    OptToken::ParMoveBack(repeats)
                }
                Token::RightAngleBracket => {
                    let mut repeats = 1;
                    proc_opt_token!(Token::RightAngleBracket, i, repeats, self);
                    OptToken::ParMoveFor(repeats)
                }
            });
            i += 1;
        }

        ot
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn optimizer_test() {
        let tokens: Tokens = vec![Token::Plus, Token::Plus, Token::Plus, Token::Minus];
        let opt_tokens = tokens.optimize();

        assert_eq!(opt_tokens, vec![OptToken::ParAdd(3), OptToken::ParSub(1)])
    }
}
