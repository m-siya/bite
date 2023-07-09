use crate::value::*;

// add_constant returns u8 
pub fn add_constant(&mut self, value: Value) -> Option<u8> 
{
    let idx = self.constants.write(value);
    u8::try_from(idx).ok()
}

fn make_constant($mut self) -> u8 // After emit_return
{
    if let Some(constant) = self.chunk.add_constant(value)
    {
        constant
    }
    else
    {
        self.error("Too many constant in one chunk.");
        0
    }
}

fn emit_constant(&mut self, value: Value) // After make_constant
{
    let constant = self.make_constant(value);
    self.emit_bytes(OpCode::Constant, constant);
}

#[derive(Copy, Clone)]
struct ParseRule // After pub struct parser
{
    prefix: Option<fn(&mut Compiler)>,
    infix: Option<fn(&mut Compiler)>,
    precedence: Precedence,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)] // After ParseRule
enum Precedence 
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

fn binary(&mut self) // After end_compiler
{
    let operator_type = self.parser.previous.ttype;
    let rule = self.rules[operator_type as usize].precedence.next();

    self.parse_precedence(rule);

    match operator_type
    {
        TokenType::Plus => self.emit_byte(OpCode::Add.into());
        TokenType::Minus => self.emit_byte(OpCode::Subtract.into());
        TokenType::Star => self.emit_byte(OpCode::Multiply.into());
        TokenType::Slash => self.emit_byte(OpCode::Divide.into());
        _ => todo!(),
    }
}

fn grouping(&mut self) // After binary
{
    self.expression();
    self.consume(TokenType::RightParen, "Expect ')' after expression.");
}

fn number(&mut self) // After grouping
{
    let value = self.parser.previous.lexeme.parse::<Value>().unwrap();
    self.emit_constant(value);
}

fn unary(&mut self) // After number
{
    let operator_type = self.parser.previous.ttype;

    self.parse_precedence(Precedence::Unary);

    if operator_type == TokenType::Minus
    {
        self.emit_byte(OpCode::Negate.into())
    }
    else
    {
        unimplemented!("nope");
    }
}

fn parse_precedence(&mut self, precedence: Precedence) // After unary
{
    self.advance();
    if let Some(prefix_rule) = self.rules[self.parser.previous.ttype as usize].prefix
    {
        prefix_rule(self);

        while precedence <= self.rules[self.parser.current.ttype as usize].precedence
        {
            self.advance();
            if let Some(infix_rule) = self.rules[self.parser.previous.ttype as usize].infix
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

// Inside vm.rs

pub fn interpret(&mut self, source: &str) -> Result<(), InterpretResult>
{
    let mut chunk = Chunk::new();
    let mut compiler = Compiler::new(&mut chunk);
    compiler.compile(source)?;

    self.ip = 0;
    self.run(&chunk)
}

// Inside scanner.rs

pub fn new(source: &str) -> Self // Inside impl Scanner
{
    Self
    {
        source: source.chars().collect::<Vec<char>>(),
        start: 0,
        current: 0,
        line: 1,
    }
}

 // Inside compiler.rs

 pub struct Compiler<'a>
 {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a mut Chunk,
    rules: Vec<ParseRule>
 }

impl<'a> Compiler<'a>
{
    pub fn new(chunk: &'a mut Chunk) -> Self
    {
        // lazy_static could be a better option for performance
        let mut rules = vec!
        [
            ParseRule
            {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            };
            TokenType::NumberOfTokens as usize
        ];
        rules[TokenType::LeftParen as usize] = ParseRule
        {
            prefix: Some(|c| c.grouping()),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::Minus as usize] = ParseRule
        {
            prefix: Some(|c| c.unary()),
            infix: Some(|c| c.binary()),
            precedence: Precedence::Term,
        };
        rules[TokenType::Plus as usize] = ParseRule
        {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Term,
        };
        rules[TokenType::Slash as usize] = ParseRule
        {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Star as usize] = ParseRule
        {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Number as usize] = ParseRule
        {
            prefix: Some(|c| c.number()),
            infix: None,
            precedence: Precedence::None,
        };

        Self
        {
            parser: Parser::default(),
            scanner: Scanner::new(&"".to_string()),
            chunk,
            rules,
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<(), InterpretResult>
    {
        self.scanner = Scanner::new(source);
        self.advance();

        self.expression();

        self.consume(TokenType::Eof, "Exprect end of expression.");

        self.end_compiler();

        if *self.parser.had_error.borrow()
        {
            Err(InterpretResult::CompileError)
        }
        else
        {
            Ok(())
        }
    }
}

 // Inside token.rs

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
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Assign, // ('=')
    Equals, // ('==')
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

 // Inside Main.rs
 //remove use chunk::* 






