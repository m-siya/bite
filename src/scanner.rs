#[derive(Debug, PartialEq, Copy, Clone)]
 pub enum TokenType
 {
    LeftParen = 0,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Assign, // ('=')
    Equal, // ('==')
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print, Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    Eof,
    Undefined,
    NumberOfTokens,
 }

#[derive(PartialEq, Clone)]
//#[derive(Copy, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: String,
    pub length: usize,
    pub line: i32,
}

impl Default for Token {
    fn default() -> Token {
        Token {token_type: TokenType::Nil, start: String::new(), length: 0, line: -1}
    }
}

pub struct Scanner<'a> {
    start: &'a str,
    current: &'a str,
    line: i32,
}

impl<'a> Scanner<'a> {

    pub fn new(source: &'a str) -> Self {
        Self {
            //source: source.chars().collect::<Vec<char>>(),
            start: source,
            current: source,
            line: 1,
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_at_end(&self) -> bool {
        self.current.is_empty()
    }

    fn advance(&mut self) -> char {
        let c = self.current.chars().next().unwrap();
        self.current = &self.current[c.len_utf8()..];
        c
    }

    fn peek(&self) -> char {
        self.current.chars().next().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current[self.current.chars().next().unwrap().len_utf8()..]
                .chars()
                .next()
                .unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start.to_string(),
            length: self.current.as_ptr() as usize - self.start.as_ptr() as usize,
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            start: message.to_string(),
            length: message.len(),
            line: self.line,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current.chars().next() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
    }
        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
    }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the ".".
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote.
        self.advance();
        self.make_token(TokenType::String)
    }

    fn identifier_type(&self) -> TokenType {
        match self.start.chars().next().unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => match self.start.len() {
                1 => TokenType::If,
                3 => self.check_keyword(1, 2, "or", TokenType::For),
                4 => self.check_keyword(1, 1, "un", TokenType::Fun),
                _ => TokenType::Identifier,
            },
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => match self.start.len() {
                2 => match self.start.chars().nth(1).unwrap() {
                    'h' => TokenType::This,
                    'r' => TokenType::True,
                    _ => TokenType::Identifier,
                },
                _ => TokenType::Identifier,
            },
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current.len() == start + length
            && &self.start[start..start + length] == rest
        {
            token_type
        } else {
            TokenType::Identifier
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if self.is_alpha(c) {
            return self.identifier();
        } else if self.is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::Equal)
                } else {
                    self.make_token(TokenType::Assign)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }
}

pub fn init_scanner(source: &str) {
    let scanner = Scanner::new(source); 
}

// ... Implement the remaining functions and logic ...
