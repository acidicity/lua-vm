#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token<'a> {
    Illegal,
    EOF,

    // Identifiers + literals
    Identifier(&'a str),
    Integer(i32), // 1343456
    
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

pub fn lookup_identifier(ident: &str) -> Token{
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        ident => Token::Identifier(ident)
    }
}