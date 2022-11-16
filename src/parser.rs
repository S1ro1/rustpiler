use crate::{lexer::Lexer, token::Token, token::TokenType};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParserRule {
    Prog,
    Statement,
    Expression,
}

#[derive(Debug)]
pub struct Parser {
    token: Token,
    lexer: Lexer,
    rule: ParserRule,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    Received: TokenType,
    Expected: TokenType,
}

impl ParseError {
    pub fn new(received: TokenType, expected: TokenType) -> Self {
        ParseError {
            Received: received,
            Expected: expected,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Parser error]: Expected {:?}, received {:?}",
            self.Expected, self.Received
        )
    }
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token: Token = lexer.next_token();
        Self {
            token,
            lexer,
            rule: ParserRule::Prog,
        }
    }

    pub fn check_next(&mut self, tok_type: TokenType) {
        self.token = self.lexer.next_token();
        if self.token.tok_type == tok_type {
            return;
        } else {
        }
    }

    pub fn rule_prog(&mut self) -> Result<i32, ParseError> {
        if self.token.tok_type
            != (TokenType::TokIdentif {
                val: "a".to_string(),
            })
        {
            Err(ParseError::new(
                self.token.tok_type.clone(),
                TokenType::TokIdentif {
                    val: "-".to_string(),
                },
            ))
        } else {
            Ok(0)
        }
    }
}
