use parser::Parser;
use token::TokenType;

mod lexer;
mod parser;
mod symtable;
mod token;
mod utils;

fn main() {
    let file = utils::chars("tests/simple_prog.si".to_string());

    let mut lexer = lexer::Lexer::new(file);

    let mut parser: Parser = parser::Parser::new(lexer);

    parser.rule_prog();

    // let mut token = lexer.next_token();

    // while token.tok_type != TokenType::TokEof {
    //     token = lexer.next_token();
    //     println!("{:?}", token);
    // }
}
