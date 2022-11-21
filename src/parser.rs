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

    pub fn check_token(&mut self, tok_type: TokenType) -> Result<bool, String> {
        if self.token.tok_type == tok_type {
            self.token = self.lexer.next_token();
            return Ok(true);
        } else {
            self.token = self.lexer.next_token();
            return Err(format!("Expected: {:?} but received: {:?}", tok_type, self.token.tok_type));
        }
    }

    pub fn token_is_operator(&mut self) -> Result<bool, String> {
        let ret: bool = self.token.tok_type == TokenType::TokPlus || self.token.tok_type == TokenType::TokMinus || self.token.tok_type == TokenType::TokDiv || self.token.tok_type == TokenType::TokMul || self.token.tok_type == TokenType::TokEq;

        if ret {
            self.token = self.lexer.next_token();
            return Ok(ret);
        } else {
            return Err(format!("Token isn't operator: {:?}", self.token));
        }
    }

    pub fn rule_prog(&mut self) {
        self.check_token(TokenType::TokIdentif { val: "a".to_string() }).unwrap();
        self.token_is_operator().unwrap();
        self.check_token(TokenType::TokInt { val: -1 }).unwrap();
    }
}
