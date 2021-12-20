/**
 * -- Tokens --
 * states [0-9] . _ * & ^ @ render r l u d 
 */
use std::io;

enum TokenType {
    Number,
    Dot,
    Null,
    All,
    Link,
    Any,
    Absorb,
    Label,      // states, render
    Direction
}

struct Token {
    ttype:TokenType,
    lexeme:String
}

impl Token {
    fn new(tt:TokenType, lex:String) -> Token {
        Token { ttype:tt, lexeme:lex }
    }
}

pub struct Tokenizer {
    tokens:Vec<Token>,
    input:String,
    char_index:usize,
    cur_char:char
}

impl Tokenizer {
    pub fn new(inp:String) -> Tokenizer {
        let first = inp.chars().next().unwrap();
        Tokenizer { tokens:vec![], input:inp, char_index:0, cur_char:first }
    }

    fn add_token(&mut self, t:TokenType, l:String) {
        let token = Token::new(t, l);
        self.tokens.push(token);
    }
    
    fn advance(&mut self) {
        self.char_index += 1;
        self.cur_char = self.input.chars().nth(self.char_index).unwrap();
    }

    pub fn start(&mut self) -> io::Result<()> {
        match self.cur_char {
            '_' => self.add_token(TokenType::Null, String::from("λ")),
            '.' => self.add_token(TokenType::Dot, String::from(".")),
            '*' => self.add_token(TokenType::All, String::from("*")),
            '^' => self.add_token(TokenType::Any, String::from("^")),
            '&' => self.add_token(TokenType::Link, String::from("&")),
            '@' => self.add_token(TokenType::Absorb, String::from("@")),
            'l' => self.add_token(TokenType::Direction, String::from("l")),
            'r' => self.add_token(TokenType::Direction, String::from("r")),
            'u' => self.add_token(TokenType::Direction, String::from("u")),
            'd' => self.add_token(TokenType::Direction, String::from("d")),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unexpected character! Failed to parse."))
        };
        self.advance();
        Ok(())
    }
}
