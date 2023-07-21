//TO DO, ONLY BASIC ROUGH TRANSLATION RIGHT NOW
use chunk::OpCode;
use chunk::Chunk;
use scanner::Token;
use scanner::TokenType;

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

// pub struct Compiler;

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
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

    //return false if error occurred
    //look into Result for this
    fn compile(&mut self, source: &str, chunk: Chunk) {
        init_scanner(source);

        compiling_chunk = chunk;

        parser.had_error = false;
        parser.panic_mode = false;

        advance();
        expression();
        consume(TOKEN_EOF, "Expect end of Expression")
        end_compiler();

        !parser.had_error
    }

}

fn advance() {
    parser.previous = parser.current;
    
    while true {
        parser.current = scan_token();
        if parser.current.type != TOKEN_ERROR {
            break;
        }

        error_at_current(parser.current.start);
    }
}

fn consume(type: TokenType, message: &str) {
    if parser.current.type == type {
        advance();
        return;
    }

    error_at_current(message);
}

// translate to bytecode
fn emit_byte(chunk: &Chunk, byte: u8) {
    chunk.write(byte, parser.previous.line);
}

fn emit_bytes(byte1: u8, byte2: u8) {
    emit_byte(byte1);
    emit_byte(byte2);
}

fn emit_return() {
    emit_byte(OpCode::OP_RETURN);
}

fn end_compiler() {
    emit_return();
}

fn error_at_current(message: &str) {
    error_at(parser.current, message);
}

fn error(message: &str) {
    error_at(parser.previous, message);
}

fn error_at(token: Token, message: &str) {
    if parser.panic_mode {
        return;
    }
    parser.panic_mode = true;
    println!("[line {}] Error", token.line);

    match token.type {
        TokenType::TOKEN_EOF => println!(),
        TokenTyep::TOKEN_ERROR => !unimplemented!(),
        _ => println!(" at {} {}", token.length, token.start),
    }

    println!(": {}", message);
    parser.had_error = true;
}
