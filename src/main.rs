pub mod lexer;
mod parser;
use lexer::Lexer;


fn main() {
    let mut lexer: Lexer = Lexer::new();
    lexer.load(
        ("var chi : i16 = 0xAf;\n".to_owned()
            + "var benny : u32 = 0x20;\n"
            + "defun (x : f32)(){\n"
            + "   ret 1.092;\n"
            + "}")
            .as_str(),
    );

}
