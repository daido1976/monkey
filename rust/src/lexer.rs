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

        token = match self.char {
            b';' => self.make_one_char_token(TokenType::SEMICOLON),
            b'(' => self.make_one_char_token(TokenType::LPAREN),
            b')' => self.make_one_char_token(TokenType::RPAREN),
            b'{' => self.make_one_char_token(TokenType::LBRACE),
            b'}' => self.make_one_char_token(TokenType::RBRACE),
            b',' => self.make_one_char_token(TokenType::COMMA),
            b'+' => self.make_one_char_token(TokenType::PLUS),
            b'-' => self.make_one_char_token(TokenType::MINUS),
            b'*' => self.make_one_char_token(TokenType::ASTERISK),
            b'/' => self.make_one_char_token(TokenType::SLASH),
            b'<' => self.make_one_char_token(TokenType::LT),
            b'>' => self.make_one_char_token(TokenType::GT),
            b'=' => match self.peek_char() {
                b'=' => self.make_two_char_token(TokenType::EQ),
                _ => self.make_one_char_token(TokenType::ASSIGN),
            },
            b'!' => match self.peek_char() {
                b'=' => self.make_two_char_token(TokenType::NOT_EQ),
                _ => self.make_one_char_token(TokenType::BANG),
            },
            0 => self.make_eof_token(),
            _ if Self::is_letter(self.char) => return self.make_letter_token(),
            _ if Self::is_number(self.char) => return self.make_number_token(),
            _ => self.make_one_char_token(TokenType::ILLEGAL),
        };
        self.read_char();
        token
    }

    fn make_one_char_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            literal: String::from_utf8(vec![self.char]).unwrap(),
        }
    }

    fn make_two_char_token(&mut self, token_type: TokenType) -> Token {
        let prev_char = self.char;
        self.read_char();
        let literal = String::from_utf8(vec![prev_char, self.char]).unwrap();
        Token {
            token_type,
            literal,
        }
    }

    fn make_eof_token(&self) -> Token {
        Token {
            token_type: TokenType::EOF,
            literal: "".to_string(),
        }
    }

    fn make_letter_token(&mut self) -> Token {
        let identifier = self.read_identifier();
        let token_type = lookup_identifier(&identifier);
        Token {
            token_type,
            literal: identifier,
        }
    }

    fn make_number_token(&mut self) -> Token {
        let literal = self.read_number();
        let token_type = TokenType::INT;
        Token {
            token_type,
            literal,
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

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            return self.input.as_bytes()[self.read_position];
        }
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

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
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
            (TokenType::BANG, "!".to_string()),
            (TokenType::MINUS, "-".to_string()),
            (TokenType::SLASH, "/".to_string()),
            (TokenType::ASTERISK, "*".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::GT, ">".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::IF, "if".to_string()),
            (TokenType::LPAREN, "(".to_string()),
            (TokenType::INT, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::RPAREN, ")".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::RETURN, "return".to_string()),
            (TokenType::TRUE, "true".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::ELSE, "else".to_string()),
            (TokenType::LBRACE, "{".to_string()),
            (TokenType::RETURN, "return".to_string()),
            (TokenType::FALSE, "false".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::RBRACE, "}".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::EQ, "==".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::SEMICOLON, ";".to_string()),
            (TokenType::INT, "10".to_string()),
            (TokenType::NOT_EQ, "!=".to_string()),
            (TokenType::INT, "9".to_string()),
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
