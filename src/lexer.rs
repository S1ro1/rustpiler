use crate::token::*;

#[derive(Debug)]
pub enum LexerState {
    LexerInt,
    LexerStart,
    LexerIdentif,
    LexerFloat,
    LexerString,
}

#[derive(Debug)]
pub struct Lexer {
    pub curr_index: usize,
    pub source: Vec<char>,
    pub tmp_buffer: Vec<char>,
    pub state: LexerState,
}

impl Lexer {
    pub fn new(mut source: Vec<char>) -> Self {
        source.push('\u{10ffff}');
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

    pub fn peek_next(&mut self) -> Option<char> {
        if self.source.len() == self.curr_index {
            return None;
        } else {
            Some(self.source[self.curr_index])
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            let c = self.get_next();
            match self.state {
                LexerState::LexerStart => match c {
                    '\u{10ffff}' => return Token::new(TokenType::TokEof),
                    '+' => return Token::new(TokenType::TokPlus),
                    '-' => return Token::new(TokenType::TokMinus),
                    '/' => return Token::new(TokenType::TokDiv),
                    '*' => return Token::new(TokenType::TokMul),
                    '>' => match self.peek_next() {
                        Some('=') => {
                            let _ = self.get_next();
                            self.state = LexerState::LexerStart;
                            return Token::new(TokenType::TokGte);
                        }
                        _ => return Token::new(TokenType::TokGt),
                    },
                    '<' => match self.peek_next() {
                        Some('=') => {
                            let _ = self.get_next();
                            self.state = LexerState::LexerStart;
                            return Token::new(TokenType::TokLte);
                        }
                        _ => return Token::new(TokenType::TokLt),
                    },
                    '=' => match self.peek_next() {
                        Some('=') => {
                            let _ = self.get_next();
                            self.state = LexerState::LexerStart;
                            return Token::new(TokenType::TokIseq);
                        }
                        _ => return Token::new(TokenType::TokEq),
                    },
                    '!' => match self.peek_next() {
                        Some('=') => {
                            let _ = self.get_next();
                            self.state = LexerState::LexerStart;
                            return Token::new(TokenType::TokNoteq);
                        }
                        _ => return Token::new(TokenType::TokNot),
                    },
                    '{' => return Token::new(TokenType::TokLcurl),
                    '}' => return Token::new(TokenType::TokRcurl),
                    ';' => return Token::new(TokenType::TokSemi),
                    '(' => return Token::new(TokenType::TokLparen),
                    ')' => return Token::new(TokenType::TokRparen),
                    '[' => return Token::new(TokenType::TokLbrace),
                    ']' => return Token::new(TokenType::TokRbrace),
                    '\"' => self.state = LexerState::LexerString,
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
                        self.tmp_buffer.clear();
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
                        let token: Token = Token::new_with_identif(tok_type, &self.tmp_buffer);
                        self.tmp_buffer.clear();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
                LexerState::LexerFloat => {
                    if c.is_numeric() {
                        self.tmp_buffer.push(c);
                    } else {
                        let token: Token =
                            Token::new_with_float(TokenType::TokFloat, &self.tmp_buffer);
                        self.tmp_buffer.clear();
                        self.curr_index -= 1;
                        self.state = LexerState::LexerStart;
                        return token;
                    }
                }
                LexerState::LexerString => {
                    if c == '\"' {
                        let token: Token =
                            Token::new_with_string(TokenType::TokString, &self.tmp_buffer);
                        self.tmp_buffer.clear();
                        self.state = LexerState::LexerStart;
                        return token;
                    } else {
                        self.tmp_buffer.push(c);
                    }
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::utils;

    #[test]
    fn single_op() {
        let text = utils::chars("tests/simple_op.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokPlus);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokMinus);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokDiv);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokMul);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokGt);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokLt);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokNot);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokEof);
    }

    #[test]
    fn braces() {
        let text = utils::chars("tests/braces.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokLbrace);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokRbrace);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokLparen);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokRparen);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokLcurl);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokRcurl);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokEof);
    }

    #[test]
    fn multi_op() {
        let text = utils::chars("tests/multi_op.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokGte);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokLte);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokIseq);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokNoteq);
        assert_eq!(lexer.next_token().tok_type, TokenType::TokEof);
    }

    #[test]
    fn identifiers() {
        let text = utils::chars("tests/identif.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);

        let mut token = lexer.next_token();
        assert_eq!(token.tok_type, TokenType::TokIdentif);
        assert_eq!(token.str, Some("a".to_string()));

        token = lexer.next_token();
        assert_eq!(token.tok_type, TokenType::TokIdentif);
        assert_eq!(token.str, Some("a25".to_string()));

        assert_eq!(lexer.next_token().tok_type, TokenType::TokEof);
    }

    #[test]
    fn literals() {
        let text = utils::chars("tests/literals.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);

        let mut token = lexer.next_token();

        assert_eq!(token.tok_type, TokenType::TokString);
        assert_eq!(token.str, Some("a".to_string()));

        token = lexer.next_token();
        assert_eq!(token.tok_type, TokenType::TokFloat);
        assert_eq!(token.flt, Some(4.2));

        token = lexer.next_token();
        assert_eq!(token.tok_type, TokenType::TokInt);
        assert_eq!(token.int, Some(42));

        assert_eq!(lexer.next_token().tok_type, TokenType::TokEof);
    }

    #[test]
    fn simple_prog() {
        use crate::token::TokenType::*;
        let text = utils::chars("tests/simple_prog.si".to_string());
        let mut lexer: Lexer = Lexer::new(text);

        let results: Vec<TokenType> = vec![
            TokIdentif, TokEq, TokInt, TokSemi, TokIdentif, TokEq, TokInt, TokSemi, TokIdentif,
            TokEq, TokInt, TokSemi, TokIf, TokIdentif, TokLt, TokIdentif, TokLcurl, TokIdentif,
            TokEq, TokInt, TokSemi, TokRcurl, TokElse, TokLcurl, TokIdentif, TokEq, TokFloat,
            TokSemi, TokRcurl, TokEof,
        ];

        let mut token = lexer.next_token();
        let mut curr: usize = 0;

        while token.tok_type != TokEof {
            let result = results.get(curr).unwrap();
            println!("{curr}");
            assert_eq!(&token.tok_type, result);
            token = lexer.next_token();
            curr += 1;
        }
    }
}
