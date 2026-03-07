use std::env;

use chilangc::lexer::Lexer;
use chilangc::utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chilangc <input_file>");
        return;
    }

    let src = match read_source_file(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading file: {e}");
            return;
        }
    };

    let mut lexer = Lexer::new();
    lexer.load(&src);

    for token in lexer {
        match token {
            Ok(tok) => println!("{tok}"),
            Err(e) => eprintln!("Lexer Error {:?}", e),
        }
    }
}
