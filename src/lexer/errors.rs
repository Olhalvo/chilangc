#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumberFormat,
    UnterminatedString,
    InvalidEscape
}
