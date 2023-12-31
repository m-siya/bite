use std::marker::PhantomData;

use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Token, TokenType, Scanner};
use crate::vm::InterpretResult;
use crate::value::Value;




#[derive(Copy, Clone)]
pub struct ParseRule // After pub struct parser
{
    prefix: Option<fn(&mut Compiler<'_>)>,
    infix: Option<fn(&mut Compiler<'_>)>,
    precedence: Precedence,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)] // After ParseRule
pub enum Precedence 
{
    None = 0,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparision,// < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl From<usize> for Precedence // After enum Precedence 
{
    fn from(v: usize) -> Self
    {
        match v 
        {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparision,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            v => panic!("cannot convert {v} into Precedence"),
        }
    }
}


impl Precedence // After From<usize> for Precedence 
{
    fn next(self) -> Self
    {
        if self == Precedence::Primary
        {
            panic!("no previous before None");
        }
        let p = self as usize;
        (p - 1).into()
    }
}

#[derive(Default)]
pub struct Parser<'a> {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    _marker: std::marker::PhantomData<&'a str>,
}

impl <'a> Parser<'a> {
    pub fn new() -> Parser <'a>{
        Parser{current: Token::default(), previous: Token::default(), had_error: false, panic_mode: false, _marker: PhantomData}
    }
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
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
        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(|c| c.literal()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(|c| c.literal()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::Nil as usize] = ParseRule {
            prefix: Some(|c| c.literal()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::Bang as usize] = ParseRule {
            prefix: Some(|c| c.literal()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::BangEqual as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Equality,
        };
        rules[TokenType::Equal as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Equality,
        };
        rules[TokenType::Greater as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Comparision,
        };
        rules[TokenType::GreaterEqual as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Comparision,
        };
        rules[TokenType::Less as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Comparision,
        };
        rules[TokenType::LessEqual as usize] = ParseRule {
            prefix: Some(|c| c.binary()),
            infix: None,
            precedence: Precedence::Comparision,
        };


        Self {
            parser: Parser::default(),
            scanner: Scanner::new(""),
            chunk,
            rules,
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        
        loop {
            self.parser.current = self.scanner.scan_token();

            if self.parser.current.token_type != TokenType::Error {
                break;
            }
    
            self.error_at_current(&self.parser.current.start.clone());
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current.clone(), message);
    }
    
    fn error(&mut self, message: &str) {
        self.error_at(self.parser.previous.clone(), message);
    }
    
    fn error_at(&mut self, token: Token, message: &str) {
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
    
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }

    fn make_constant(&mut self, value: Value) -> u8 // After emit_return
    {
        // if let Some(constant) = self.chunk.add_constant(value)
        // {
        //     constant
        // }
        // else
        // {
        //     self.error("Too many constant in one chunk.");
        //     0
        // }

        self.chunk.add_constant(value) as u8

    }

    fn emit_constant(&mut self, value: Value) // After make_constant
    {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant.into(), constant);
    }
    
    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn binary(&mut self) // After end_compiler
    {
        let operator_type = self.parser.previous.token_type;
        let rule = self.rules[operator_type as usize].precedence.next();

        self.parse_precedence(rule);

        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual.into(), OpCode::OpNot.into()),
            TokenType::Equal => self.emit_byte(OpCode::OpEqual.into()),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater.into()),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess.into(), OpCode::OpNot.into()),
            TokenType::Less => self.emit_byte(OpCode::OpLess.into()),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater.into(), OpCode::OpNot.into()),
            TokenType::Plus => self.emit_byte(OpCode::OpAdd.into()),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract.into()),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply.into()),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide.into()),
            _ => todo!(),
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OpFalse.into()),
            TokenType::Nil => self.emit_byte(OpCode::OpNil.into()),
            TokenType::True => self.emit_byte(OpCode::OpTrue.into()),
            _ => (),
        }
    }

    fn grouping(&mut self) // After binary
    {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) // After grouping
    {
        let value: Value = Value::ValNumber(self.parser.previous.start.parse().unwrap());
        self.emit_constant(value);
    }

    fn unary(&mut self) // After number
    {
        let operator_type = self.parser.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::OpNegate.into()),
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into()),
            _ => unimplemented!(),
            
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) // After unary
    {
        self.advance();
        if let Some(prefix_rule) = self.rules[self.parser.previous.token_type as usize].prefix
        {
            prefix_rule(self);

            while precedence <= self.rules[self.parser.current.token_type as usize].precedence
            {
                self.advance();
                if let Some(infix_rule) = self.rules[self.parser.previous.token_type as usize].infix
                {
                    infix_rule(self);
                }
            }
        }
        else
        {
            self.error("Expect expression.");
        }
    }

    fn get_rule(&self, ttype: TokenType) -> &ParseRule // After parse_precedence
    {
        &self.rules[ttype as usize]
    }

    fn expression(&mut self) // After get_rule
    {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn compile(&mut self, source: &'a str) -> Result<(), InterpretResult>
    {
        self.scanner = Scanner::new(source);
        self.advance();

        self.expression();

        self.consume(TokenType::Eof, "Expect end of expression.");

        self.end_compiler();

        if self.parser.had_error == true
        {
            Err(InterpretResult::CompileError)
        }
        else
        {
            Ok(())
        }
    }

}


// translate to bytecode
