use crate::{token::*, utils};

#[derive(Debug)]
pub enum LexerState {
    LexerInt,
    LexerStart,
    LexerIdentif,
    LexerFloat,
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
            source,
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
                    '>' => return Token::new(TokenType::TokGt),
                    '<' => return Token::new(TokenType::TokLt),
                    '=' => return Token::new(TokenType::TokEq),
                    '{' => return Token::new(TokenType::TokLcurl),
                    '}' => return Token::new(TokenType::TokRcurl),
                    ';' => return Token::new(TokenType::TokSemi),
                    cn => {
                        if cn.is_alphabetic() {
                            self.state = LexerState::LexerIdentif;
                            self.tmp_buffer.push(cn);
                        } else if cn.is_ascii_digit() {
                            self.state = LexerState::LexerInt;
                            self.tmp_buffer.push(cn);
                        } else if cn.is_whitespace() {
                            continue;
                        } else {
                            return Token::new(TokenType::TokUnknown);
                        }
                    }
                },
                LexerState::LexerInt => {
                    if c.is_numeric() {
                        self.tmp_buffer.push(c);
                    } else if c == '.' {
                        self.tmp_buffer.push(c);
                        self.state = LexerState::LexerFloat;
                    } else {
                        let token: Token = Token::new_with_int(TokenType::TokInt, &self.tmp_buffer);
                        self.tmp_buffer = Vec::new();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
                LexerState::LexerIdentif => {
                    if c.is_alphanumeric() {
                        self.tmp_buffer.push(c);
                    } else {
                        let tok_type: TokenType = Token::parse_keyword(&self.tmp_buffer);
                        let token: Token = Token::new_with_string(tok_type, &self.tmp_buffer);
                        self.tmp_buffer = Vec::new();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
                LexerState::LexerFloat => {
                    if c.is_numeric() {
                        self.tmp_buffer.push(c);
                    } else {
                        let token: Token = Token::new_with_float(TokenType::TokInt, &self.tmp_buffer);
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
