use token::TokenType;

mod lexer;
mod token;
mod utils;

fn main() {
    let file = utils::chars("tests/simple_op.si".to_string());

    let mut lexer = lexer::Lexer::new(file);

    let mut token = lexer.next_token();

    while token.tok_type != TokenType::TokEof {
        token = lexer.next_token();
    }
}
