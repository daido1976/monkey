use crate::token::{lookup_identifier, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,      // current position in input
    read_position: usize, // current reading position in input (next char)
    char: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            char: 0,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip_whitespace();

        match self.char {
            b'=' => token = Self::new_token(TokenType::ASSIGN, self.char),
            b';' => token = Self::new_token(TokenType::SEMICOLON, self.char),
            b'(' => token = Self::new_token(TokenType::LPAREN, self.char),
            b')' => token = Self::new_token(TokenType::RPAREN, self.char),
            b'{' => token = Self::new_token(TokenType::LBRACE, self.char),
            b'}' => token = Self::new_token(TokenType::RBRACE, self.char),
            b',' => token = Self::new_token(TokenType::COMMA, self.char),
            b'+' => token = Self::new_token(TokenType::PLUS, self.char),
            0 => {
                token = Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            }
            _ => {
                if Self::is_letter(self.char) {
                    let identifier = self.read_identifier();
                    let token_type = lookup_identifier(&identifier);
                    return Token {
                        token_type,
                        literal: identifier,
                    };
                } else if Self::is_number(self.char) {
                    let literal = self.read_number();
                    let token_type = TokenType::INT;
                    return Token {
                        token_type,
                        literal,
                    };
                } else {
                    token = Self::new_token(TokenType::ILLEGAL, self.char)
                }
            }
        }
        self.read_char();
        token
    }

    fn new_token(token_type: TokenType, char: u8) -> Token {
        Token {
            token_type,
            literal: String::from_utf8(vec![char]).unwrap(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.char == b' ' || self.char == b'\t' || self.char == b'\n' || self.char == b'\r' {
            self.read_char()
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let prev_position = self.position;
        while Self::is_letter(self.char) {
            self.read_char()
        }

        (self.input[prev_position..self.position]).to_string()
    }

    fn is_letter(char: u8) -> bool {
        (b'a'..=b'z').contains(&char) || (b'A'..=b'Z').contains(&char) || char == b'_'
    }

    fn read_number(&mut self) -> String {
        let prev_position = self.position;
        while Self::is_number(self.char) {
            self.read_char()
        }

        (self.input[prev_position..self.position]).to_string()
    }

    fn is_number(char: u8) -> bool {
        (b'0'..=b'9').contains(&char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_lexer() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
        "
        .to_string();
        let mut l = Lexer::new(input);
        let expects = vec![
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "five".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "ten".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "add".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::FUNCTION, "fn".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::IDENT, "x".to_string()),
            (TokenType::COMMA, ",".to_string()),
            (TokenType::IDENT, "y".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::IDENT, "x".to_string()),
            (TokenType::PLUS, "+".to_string()),
            (TokenType::IDENT, "y".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::LET, "let".to_string()),
            (TokenType::IDENT, "result".to_string()),
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::IDENT, "add".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::IDENT, "five".to_string()),
            (TokenType::COMMA, ",".to_string()),
            (TokenType::IDENT, "ten".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::EOF, "".to_string()),
        ];

        for (token_type, literal) in expects {
            let t = l.next_token();
            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal);
        }
    }
}