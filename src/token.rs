#[derive(Debug, Clone)]
pub enum TokenType {
    TokPlus,
    TokMinus,
    TokDiv,
    TokMul,
    TokGt,
    TokGte,
    TokLt,
    TokLte,
    TokEq,
    TokIseq,
    TokLcurl,
    TokRcurl,
    TokLparen,
    TokRparen,
    TokLbrace,
    TokRbrace,
    TokIf,
    TokElse,
    TokUnknown,
    TokInt { val: i32 },
    TokFloat { val: f32 },
    TokIdentif { val: String },
    TokString { val: String },
    TokSemi,
    TokEof,
    TokNot,
    TokNoteq,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Debug)]
pub struct Token {
    pub tok_type: TokenType,
}

impl Token {
    pub fn new(tok_type: TokenType) -> Self {
        Token { tok_type }
    }

    pub fn new_with_identif(tok_type: TokenType) -> Self {
        Token { tok_type: tok_type }
    }

    pub fn new_with_int(buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();
        let num: i32 = buffer
            .parse::<i32>()
            .expect(&format!("[LEX ERR]: Can't parse {} into integer!", buffer));

        Token {
            tok_type: TokenType::TokInt { val: num },
        }
    }

    pub fn new_with_float(buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();
        let num: f32 = buffer
            .parse::<f32>()
            .expect(&format!("[LEX ERR]: Can't parse {} into float!", buffer));

        Token {
            tok_type: TokenType::TokFloat { val: num },
        }
    }

    pub fn new_with_string(buffer: &Vec<char>) -> Self {
        let buffer: String = buffer.into_iter().collect();

        Token {
            tok_type: TokenType::TokString { val: buffer },
        }
    }

    pub fn parse_keyword(chars: &Vec<char>) -> TokenType {
        let val: String = chars.iter().collect::<String>();

        match val.as_str() {
            "if" => return TokenType::TokIf,
            "else" => return TokenType::TokElse,
            _ => return TokenType::TokIdentif { val },
        }
    }
}
