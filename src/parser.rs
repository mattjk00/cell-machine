use std::io::{Error, Result, ErrorKind};
use crate::{tokenizer::{Token, TokenType, print_tokens}, bio::BioRule};
/*
Grammar
-------
N used to denote any number, can be nullable

<sys> 	-> 'states' N<nl><rules><render><EOF>
<rules> -> <id> <neigh> <id> <move> <id><nl><rules>
<rules> -> lambda
<id>	->  N
<neigh>	-> <op>.N
<op>	-> ^N
<op>	-> *
<move>	-> ^
<move>	-> @
<move>	-> N
<render>-> 'render'<nl><rrule>
<rrule> -> N <hex><nl><rrule>
<rrule> -> lambda
<hex>	-> NNNNNN
 */

/// Used to Parse cell-machine Tokens.
/// The parser is used to create a list of BioRules
pub struct Parser {
    pub input: Vec<Token>,
    rules: Vec<BioRule>,
    cur_token:Token,
    cur_index:usize,
    n_states:i32
}

impl Parser {
    /// Create a new parser with given token input.
    pub fn new(inp:Vec<Token>) -> Parser {
        let curt:Token = inp[0].clone();
        Parser { input:inp, rules:vec![], cur_index:0, cur_token:curt, n_states:0 }
    }

    /// Resets parser helper fields to initial state and begins parse.
    pub fn start(&mut self) {
        self.cur_index = 0;
        self.cur_token = self.input[0].clone();
        self.n_states = 0;
        //self.sanitize_whitespace();
        print_tokens(&self.input);

        self.sys();

        println!("Finished Parse!\nSystem with {} states.", self.n_states);
    }

    // ---- Parsing Functions ---- //

    fn sys(&mut self) {
        if self.cur_token.lexeme == "states" {
            self.advance();
            //self.consume(TokenType::Space);
            self.num();
            self.consume(TokenType::Newline);
        }
    }

    fn num(&mut self) {
        let str_val = self.consume(TokenType::Number);
        let i_val:i32 = str_val.parse().unwrap();
        self.n_states = i_val;
    }

    // ---- End Parsing Functions ---- //

    /// Removes unnecessary repeated whitespace tokens.
    /// This function is perhaps not needed, a space() production may be more efficient.
    #[deprecated]
    fn sanitize_whitespace(&mut self) {
        let mut is_space:bool;
        let mut search_i:usize = 0;
        let mut first_space = true;

        let input_copy = self.input.clone();

        for t in input_copy {
            match t.ttype {
                TokenType::Space | TokenType::Tab => is_space = true,
                _ => is_space = false
            };

            if is_space && !first_space {
                // Mark excess whitespace for removal
                self.input[search_i].ttype = TokenType::Invalid;
            }
            else if is_space && first_space {
                first_space = false;
            }
            else if !is_space {
                first_space = true;
            }
            search_i += 1;
        }
        // Clear out excess whitespace
        self.input.retain(|t| match t.ttype {
            TokenType::Invalid => false,
            _ => true
        });
    }

    fn advance(&mut self) -> bool {
        self.cur_index += 1;
        if self.cur_index < self.input.len() {
            self.cur_token = self.input[self.cur_index].clone();
            return true;
        }
        false
    }

    fn consume(&mut self, tok:TokenType) -> String {
        // Ignore Excess whitespace
        if tok != TokenType::Space && tok != TokenType::Tab {
            while self.cur_token.ttype == TokenType::Space || self.cur_token.ttype == TokenType::Tab {
                self.advance();
            }
        }
        if self.cur_token.ttype == tok {
                let lex_copy = self.cur_token.lexeme.clone();
                println!("Consuming {} -> {}", lex_copy, tok);
                self.advance();
                return lex_copy;
        }
        self.error(format!("Paring Error! Expected type {}, found {}", tok, self.cur_token.ttype));
        String::new()
    }

    fn error(&self, msg:String) {
        println!("{}", msg);
        std::process::exit(1);
    }
}