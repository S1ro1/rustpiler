mod lexer;
mod token;
mod utils;

fn main() {
    let file = utils::chars("test.si".to_string());

    let mut lexer = lexer::Lexer::new(file);

    while lexer.curr_index != lexer.source.len() {
        println!("{:?}", lexer.next_token());
    }
}
