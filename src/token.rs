#[derive(Debug)]
pub enum TokenType {
    TokPlus,
    TokMinus,
    TokDiv,
    TokMul,
    TokGt,
    TokLt,
    TokEq,
    TokLcurl,
    TokRcurl,
    TokIf,
    TokElse,
    TokUnknown,
    TokInt,
    TokIdentif,
    TokSemi,
}

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    str: Option<String>,
    int: Option<i32>,
    flt: Option<f32>,
}

impl Token {
    pub fn new(tok_type: TokenType) -> Self {
        Token {
            tok_type: tok_type,
            str: None,
            int: None,
            flt: None,
        }
    }

    pub fn new_with_string(tok_type: TokenType, buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();
        Token {
            tok_type: tok_type,
            str: Some(buffer),
            int: None,
            flt: None,
        }
    }

    pub fn new_with_int(tok_type: TokenType, buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();
        let num: i32 = buffer
            .parse::<i32>()
            .expect(&format!("[LEX ERR]: Can't parse {} into integer!", buffer));

        Token {
            tok_type: tok_type,
            str: None,
            int: Some(num),
            flt: None,
        }
    }

    pub fn new_with_float(tok_type: TokenType, buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();
        let num: f32 = buffer
            .parse::<f32>()
            .expect(&format!("[LEX ERR]: Can't parse {} into float!", buffer));

        Token {
            tok_type: tok_type,
            str: None,
            int: None,
            flt: Some(num),
        }
    }

    pub fn parse_keyword(chars: &Vec<char>) -> TokenType {
        let val: String = chars.into_iter().collect();

        match val.as_str() {
            "if" => return TokenType::TokIf,
            "else" => return TokenType::TokElse,
            _ => return TokenType::TokIdentif,
        }
    }
}
