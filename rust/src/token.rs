#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    ILLEGAL, // ILLEGAL
    EOF,     // EOF

    // identifier and literal
    IDENT, // IDENT
    INT,   // 123...

    // operator
    ASSIGN, // =
    PLUS,   // +
    LT,     // <
    GT,     // >
    EQ,     // ==

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
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "let" => TokenType::LET,
        "fn" => TokenType::FUNCTION,
        _ => TokenType::IDENT,
    }
}

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
