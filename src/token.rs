#[derive(Debug)]
pub enum TokenType {
    TokPlus,
    TokMinus,
    TokDiv,
    TokMul,
    TokEq,
    TokLcurl,
    TokRcurl,
    TokIf,
    TokElse,
    TokUnknown,
    TokInt,
    TokIdentif,
}

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    buffer: Option<Vec<char>>,
}

impl Token {
    pub fn new(tok_type: TokenType) -> Self {
        Token {
            tok_type: tok_type,
            buffer: None,
        }
    }

    pub fn new_with_buffer(tok_type: TokenType, buffer: Vec<char>) -> Self {
        Token {
            tok_type: tok_type,
            buffer: Some(buffer),
        }
    }
}
