 // Inside compiler.rs

    pub fn compile(&mut self, source: &str) -> Result<(), InterpretResult> // Inside impl compile
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