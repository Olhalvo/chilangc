use chilangc::lexer::*;
use chilangc::lexer::token::*;
use chilangc::lexer::errors::*;
#[test]
fn test_numbers_and_hex() {
    let mut lexer = Lexer::new();
    lexer.load("123 0x1A");
    
    let tokens: Vec<_> = lexer.collect();
    assert!(matches!(tokens[0], Ok(Token { kind: TokenType::IntLiteralDec(..), .. })));
    assert!(matches!(tokens[1], Ok(Token { kind: TokenType::IntLiteralHex(..), .. })));
}
#[test]
fn test_strings_with_escapes() {
    let mut lexer = Lexer::new();
    lexer.load(r#""hello \"world\" \n""#);
    
    let token = lexer.next().unwrap().unwrap();
    if let TokenType::StringLiteral(val) = token.kind {
        assert_eq!(val, "hello \"world\" \n");
    } else {
        panic!("Expected StringLiteral");
    }
}
#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new();
    lexer.load(r#""this string never ends"#);
    
    let result = lexer.next().unwrap();
    assert!(matches!(result, Err(LexerError::UnterminatedString)));
}
#[test]
fn test_invalid_escape() {
    let mut lexer = Lexer::new();
    lexer.load(r#""\z""#);
    
    let result = lexer.next().unwrap();
    assert!(matches!(result, Err(LexerError::InvalidEscape)));
}
#[test]
fn test_comments_skipping() {
    let mut lexer = Lexer::new();
    lexer.load("var // comment\n x");
    
    let tokens: Vec<_> = lexer.collect();
    // Should only be 'var' and 'x' and 'EOF', comment is skipped
    assert_eq!(tokens.len(), 3);
}

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new();
    lexer.load("");
    
    let token = lexer.next().unwrap().unwrap();
    assert_eq!(token.kind, TokenType::EOF);
}

#[test]
fn test_lots_of_whitespace() {
    let mut lexer = Lexer::new();
    lexer.load("  \n\t  var\t\n  x  ");
    
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 3);
}

#[test]
fn test_numeric_boundaries() {
    let mut lexer = Lexer::new();
    lexer.load("100+200;");
    
    let tokens: Vec<_> = lexer.collect();
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_nested_token_patterns() {
    let mut lexer = Lexer::new();
    lexer.load("defun variable var");
    
    let tokens: Vec<_> = lexer.collect();
    assert!(matches!(tokens[0].as_ref().unwrap().kind, TokenType::Defun));
    assert!(matches!(tokens[1].as_ref().unwrap().kind, TokenType::Identifier(_)));
    assert!(matches!(tokens[2].as_ref().unwrap().kind, TokenType::Var));
}

#[test]
fn test_error_resilience() {
    let mut lexer = Lexer::new();

    lexer.load("var @ invalid_token"); 
    
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 4); 
}