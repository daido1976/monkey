use crate::token::{Token, TokenType};

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

        match self.char {
            b'=' => token = Self::new_token(TokenType::ASSIGN, self.char),
            b';' => token = Self::new_token(TokenType::SEMICOLON, self.char),
            b'(' => token = Self::new_token(TokenType::LPAREN, self.char),
            b')' => token = Self::new_token(TokenType::RPAREN, self.char),
            b'{' => token = Self::new_token(TokenType::LBRACE, self.char),
            b'}' => token = Self::new_token(TokenType::RBRACE, self.char),
            b',' => token = Self::new_token(TokenType::COMMA, self.char),
            b'+' => token = Self::new_token(TokenType::PLUS, self.char),
            0 => token = Self::new_token(TokenType::EOF, self.char),
            _ => {
                panic!()
            }
        }
        Self::read_char(self);
        token
    }

    fn new_token(token_type: TokenType, char: u8) -> Token {
        Token {
            token_type,
            literal: String::from_utf8(vec![char]).unwrap(),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_lexer() {
        let input = "=+(){},;".to_string();
        let mut l = Lexer::new(input);
        let expects = vec![
            (TokenType::ASSIGN, "=".to_string()),
            (TokenType::PLUS, "+".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::COMMA, ",".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
        ];

        for (token_type, literal) in expects {
            let t = l.next_token();
            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal);
        }
    }
}
