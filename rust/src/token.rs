#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    ILLEGAL, // ILLEGAL
    EOF,     // EOF

    // identifier and literal
    IDENT, // IDENT
    INT,   // 123...

    // operator
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    LT,     // <
    GT,     // >
    EQ,     // ==
    NOT_EQ, // !=

    // delimiter
    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    // keyword
    FUNCTION, // FUNCTION
    LET,      // LET
    TRUE,     // true
    FALSE,    // false
    IF,       // if
    ELSE,     // else
    RETURN,   // return
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "let" => TokenType::LET,
        "fn" => TokenType::FUNCTION,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}
