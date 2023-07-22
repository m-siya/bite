use crate::{chunk, scanner, expression};
use chunk::{Chunk, OpCode};
use scanner::{Token, TokenType, Scanner};
use expression::{ParseRule, Precedence};


#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser{current: Token::default(), previous: Token::default(), had_error: false, panic_mode: false}
    }
}

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner<'a>,
    chunk: &'a mut Chunk,
    rules: Vec<ParseRule>
 }

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        // lazy_static could be a better option for performance
        let mut rules = vec! [
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            };
            TokenType::NumberOfTokens as usize
        ];
        rules[TokenType::LeftParen as usize] = ParseRule {
            prefix: Some(|c| c.grouping()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::Minus as usize] = ParseRule {
            prefix: Some(|c| c.unary()),
            infix: Some(|c| c.binary()),
            precedence: Precedence::Term,
        };
        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Term,
        };
        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(|c| c.number()),
            infix: None,
            precedence: Precedence::None,
        };

        Self {
            parser: Parser::default(),
            scanner: Scanner::new(&"".to_string()),
            chunk,
            rules,
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        
        loop {
            self.parser.current = self.scanner.scan_token();

            if self.parser.current.token_type != TokenType::Error {
                break;
            }
    
            self.error_at_current(&self.parser.current.start);
        }
    }

    fn error_at_current(&self, message: &str) {
        self.error_at(self.parser.current, message);
    }
    
    fn error(&self, message: &str) {
        self.error_at(self.parser.previous, message);
    }
    
    fn error_at(&self, token: Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);
    
        match token.token_type {
            TokenType::Eof => eprint!(" at the end"),
            TokenType::Error => !unimplemented!(),
            _ => eprint!(" at {} {}", token.length, token.start),
        }
    
        println!(": {}", message);
        self.parser.had_error = true;
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str) {
        match self.parser.current.token_type {
            token_type => {
                self.advance();
            }
            _ => {
                self.error_at_current(message);
            }
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }
    
    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
    
    fn emit_return(&self) {
        self.emit_byte(OpCode::OpReturn.into());
    }
    
    fn end_compiler(&self) {
        self.emit_return();
    }

    fn expression(&mut self) {
    }
    
    //return false if error occurred
    pub fn compile(&mut self, source: &str, chunk: Chunk) -> bool{
        //init_scanner(source);
        self.scanner = Scanner::new(source);

        let compiling_chunk: Chunk = chunk;

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of Expression");
        self.end_compiler();

        !self.parser.had_error
    }

}


// translate to bytecode
