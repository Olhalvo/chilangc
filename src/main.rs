mod lexer;
mod parser;
use lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    lexer.load("var chi : ptr i16");
    loop {
        let opt = lexer.next_token();
    }
}
