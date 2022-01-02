/**
 * Matthew Kleitz, 2021
 * -- Tokens --
 * states [0-9] . _ * & ^ @ render r l u d 
 */
use std::io;
use std::fmt;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,         // Decimal or Hex Number
    Dot,            // .
    Null,           // _
    All,            // *
    Link,           // &
    Any,            // ^
    Absorb,         // @
    Label,          // states, render
    Direction,      // l, r, u, d
    Newline,        // \n
    Space,
    Tab,
    Invalid,
    EOF
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}
#[derive(Clone)]
pub struct Token {
    pub ttype:TokenType,
    pub lexeme:String,
    pub line:usize
}

impl Token {
    fn new(tt:TokenType, lex:String, line:usize) -> Token {
        Token { ttype:tt, lexeme:lex, line:line }
    }
}

/// The tokenizer will parse and tokenize a cell-machine source file.
/// Give the input string when creating a new tokenizer and then call parse().
/// If the tokenization process is successful, you can retrieve the results from the tokens field.
pub struct Tokenizer {
    pub tokens:Vec<Token>,      // Stores tokens during parsing process.
    input:String,           // Inputted code.
    char_index:usize,       // Index of current char.
    cur_char:char,          // What char the tokenizer is currently looking at.
    word_stack:Vec<char>,   // Used to help parse user defined token words such as numbers.
    cur_line:usize          // Current line of input file.
}

impl Tokenizer {
    /// Creates a new tokenizer that is primed to process given input data.
    pub fn new(inp:String) -> Tokenizer {
        let first = inp.chars().next().unwrap();
        Tokenizer { tokens:vec![], input:inp, char_index:0, cur_char:first, word_stack:vec![], cur_line:1 }
    }

    pub fn new_from_file(path:String) -> Tokenizer {
        let data = fs::read_to_string(path).expect("Failed to open source file!");
        println!("File: {}", data);
        Tokenizer::new(data)
    }

    /// Creates and stores a token in the tokenizer's list.
    fn add_token(&mut self, t:TokenType, l:String) {
        let token = Token::new(t, l, self.cur_line);
        self.tokens.push(token);
    }
    
    /// Tries to advance to the next character in the input file.
    /// If the end input is reached, this function will return false.
    fn advance(&mut self) -> bool {
        self.char_index += 1;
        if self.char_index < self.input.len() {
            self.cur_char = self.input.chars().nth(self.char_index).unwrap();
            return true;
        }
        false
    }

    /// Peek at the next char in the input string without advancing.
    /// Returns None if the next char is invalid or the end of the file.
    fn peek_next(&self) -> Option<char> {
        let peek_index = self.char_index + 1;
        if peek_index < self.input.len() {
            return Some(self.input.chars().nth(peek_index).unwrap());
        }
        None
    }
    
    /// Call this function to begin the tokenizer. If the tokenizer is successful, this will return Ok()
    pub fn start(&mut self) -> io::Result<()> {
        self.tokens.clear();
        self.cur_char = self.input.chars().nth(0).unwrap();
        self.cur_line = 1;
        self.char_index = 0;
        //self.advance();
        let parse_result = self.parse();

        // Add an EOF token if parse was success
        match parse_result {
            Ok(o) => self.tokens.push(Token::new(TokenType::EOF, "EOF".to_string(), self.cur_line)),
            _ => ()
        };
        parse_result
    }

    /// Main function of the recursive descent parser.
    
    fn parse(&mut self) -> io::Result<()> {
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
            ' ' => self.add_token(TokenType::Space, String::from("~")),
            '\t' => self.add_token(TokenType::Tab, String::from("\\t")),
            '\n' => {
                self.add_token(TokenType::Newline, String::from("\\n"));
                self.cur_line += 1;
            },
            '#' => {
                self.advance();
                self.parse_comment();
            },
            's' => {
                let result = self.parse_kw_states();
                if !result {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unexpected Symbol {} when trying to parse 'states' label on line {}. Aborting Parse.", self.cur_char, self.cur_line)));
                }
            },
            _ => {
                if self.cur_char.is_digit(10) {
                    self.word_stack.push(self.cur_char);
                    self.parse_number();
                }
                else {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unexpected Symbol {} on line {}. Aborting Parse.\n", self.cur_char, self.cur_line)));
                }
            }
        };
        if self.advance() {
            self.parse().expect("PARSE FAIL");
        }
        
        Ok(())
    }

    /*fn parse_whitespace(&mut self) {
        let mut peek = self.peek_next();
        let mut is_ws = false;
        while is_ws {
            match peek {
                Some(c) => is_ws = c.is_whitespace(),
                None => is_ws = false
            };
            if is_ws {
                self.advance();
                peek = self.peek_next();
            }
        }
    }*/

    /// Parses a number token.
    /// This function assumes that the first digit of the number was placed in self.word_stack already.
    fn parse_number(&mut self) {
        let peek = self.peek_next();
        let mut is_dig = false;
        match peek {
            Some(c) => is_dig = c.is_digit(10),
            None => ()
        };
        if is_dig {
            self.advance();
            self.word_stack.push(self.cur_char);
            self.parse_number();
        }
        
        if self.word_stack.len() > 0 {
            let lex = self.word_stack.iter().collect();
            self.add_token(TokenType::Number, lex);
            self.word_stack.clear();
        }
    }

    /// Attempts to tokenize the 'states' keyword.
    /// Will return a bool value indicating the success of tokenizing.
    fn parse_kw_states(&mut self) -> bool {
        let key:Vec<char> = vec!['s', 't', 'a', 't', 'e', 's'];
        for i in 0..key.len() {
            if key[i] != self.cur_char {
                return false; // failed to parse whole word
            }
            if i != key.len() - 1 {
                self.advance();
            }
        }
        self.add_token(TokenType::Label, String::from("states"));
        true
    }

    /// Handles the parsing of comments. Advance past every char until a newline is reached.
    fn parse_comment(&mut self) {
        let peek = self.peek_next();
        match peek {
            Some(c) => {
                if c != '\n' {
                    self.advance();
                    self.parse_comment();
                }
            },
            None => ()
        }
    }
}

pub fn print_tokens(tokens:&Vec<Token>) {
    //println!("Input:\n----------\n{}\n\n\n~~TOKENS~~\n", self.input);
    println!("Type\t\tLexeme\t\tLine #\t\tIndex\n-----------------------------------------------------");
    let mut index = 0;
    for t in tokens.iter() {
        println!("{:4}\t\t{:4}\t\t{:4}\t\t{:4}", t.ttype, t.lexeme, t.line, index);
        index += 1;
        match t.ttype {
            TokenType::Newline => println!(),
            _ => ()
        };
    }
}