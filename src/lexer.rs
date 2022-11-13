use crate::token::*;

#[derive(Debug)]
pub enum LexerState {
    LexerInt,
    LexerStart,
    LexerIdentif,
}

#[derive(Debug)]
pub struct Lexer {
    pub curr_index: usize,
    pub source: Vec<char>,
    pub tmp_buffer: Vec<char>,
    pub state: LexerState,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Lexer {
            curr_index: 0,
            tmp_buffer: Vec::new(),
            source: source,
            state: LexerState::LexerStart,
        }
    }
    pub fn get_next(&mut self) -> char {
        let c = self.source[self.curr_index];
        self.curr_index += 1;
        return c;
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            let c = self.get_next();

            match self.state {
                LexerState::LexerStart => match c {
                    '+' => return Token::new(TokenType::TokPlus),
                    '-' => return Token::new(TokenType::TokMinus),
                    '/' => return Token::new(TokenType::TokDiv),
                    '*' => return Token::new(TokenType::TokMul),
                    '=' => return Token::new(TokenType::TokEq),
                    '{' => return Token::new(TokenType::TokLcurl),
                    '}' => return Token::new(TokenType::TokRcurl),
                    cn => {
                        if cn.is_alphabetic() {
                            self.state = LexerState::LexerIdentif;
                            self.tmp_buffer.push(cn);
                        } else if cn.is_ascii_digit() {
                            self.state = LexerState::LexerInt;
                            self.tmp_buffer.push(cn);
                        } else {
                            return Token::new(TokenType::TokUnknown);
                        }
                    }
                },
                LexerState::LexerInt => {
                    if c.is_numeric() {
                        self.tmp_buffer.push(c);
                    } else {
                        let token: Token =
                            Token::new_with_buffer(TokenType::TokInt, self.tmp_buffer.clone());
                        self.tmp_buffer = Vec::new();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
                LexerState::LexerIdentif => {
                    if c.is_alphabetic() {
                        self.tmp_buffer.push(c);
                    } else {
                        let token: Token =
                            Token::new_with_buffer(TokenType::TokIdentif, self.tmp_buffer.clone());
                        self.tmp_buffer = Vec::new();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
            };
        }
    }
}
