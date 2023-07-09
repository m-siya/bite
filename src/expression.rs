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
    let rule = self.get_rule(operator_type);

    self.parse_precedence(rule.precedence.next());

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

fn parse_precedence(&self, precedence: Precedence) // After unary
{

}

fn get_rule(&self, ttype: TokenType) -> ParseRule // After parse_precedence
{
    match ttype
    {
        /*
        TokenType::LeftParen => ParseRule
        {
            prefix: Some(Compiler::grouping),
            infix: None,
            precedence: Precedence::None,
        }
        */
        _ => ParseRule
        {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }
}

fn expression(&mut self) // After get_rule
{
    self.parse_precedence(Precedence::Assignment);
}






