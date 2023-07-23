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