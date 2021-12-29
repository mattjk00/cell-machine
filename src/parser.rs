use std::io::{Error, Result, ErrorKind};
use rand::prelude::ThreadRng;

use crate::{tokenizer::{Token, TokenType, print_tokens}, bio::{BioRule, BioMove}};
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
    cur_rule:BioRule,
    cur_token:Token,
    cur_index:usize,
    n_states:i32
}

impl Parser {
    /// Create a new parser with given token input.
    pub fn new(inp:Vec<Token>, rng:ThreadRng) -> Parser {
        let curt:Token = inp[0].clone();
        Parser { input:inp, rules:vec![], cur_rule:BioRule::new_blank(), cur_index:0, cur_token:curt, n_states:0 }
    }

    /// Resets parser helper fields to initial state and begins parse.
    pub fn start(&mut self) {
        self.cur_index = 0;
        self.cur_token = self.input[0].clone();
        self.n_states = 0;
        //self.sanitize_whitespace();

        self.input.retain(|t| match t.ttype {
            TokenType::Space | TokenType::Tab => false,
            _ => true
        });

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
        while self.cur_token.ttype == TokenType::Number {
            self.cur_rule.owner_state = self.cur_token.lexeme.parse().unwrap();
            self.advance();
            self.neigh();
            self.offspring();
            self.mov();

            // Parse the next state
            let lex = self.consume(TokenType::Number);
            self.cur_rule.next_state = lex.parse().unwrap();

            self.rules.push(self.cur_rule.clone());
            self.cur_rule = BioRule::new_blank();

            if self.cur_token.ttype == TokenType::EOF {
                self.advance();
            }
            else {
                self.consume(TokenType::Newline);
            }
        }
    }

    fn neigh(&mut self) {
        // Determine which neighbors and their state
        if self.cur_token.ttype == TokenType::Number {
            self.cur_rule.neighbors.push(self.cur_token.lexeme.parse().unwrap());
            self.advance();
        }
        else {
            match self.cur_token.lexeme.as_ref() {
                "^" => self.any_neigh(),
                "*" => self.all_neigh(),
                _ => self.error(String::from("Expecting number or operator."))
            };
            //self.advance();
            self.consume(TokenType::Dot);
            let lex = self.consume(TokenType::Number);
            self.cur_rule.neighbors_state = lex.parse().unwrap();
        }
        
        
    }

    fn any_neigh(&mut self) {
        self.cur_rule.any_neighbor = true;
        self.advance();
        
        if self.cur_token.ttype == TokenType::Number {
            let num:i32 = self.cur_token.lexeme.parse().unwrap();
            self.advance();
            self.cur_rule.any_neighbor_count = num;
        }
    }

    fn all_neigh(&mut self) {
        self.cur_rule.neighbors.extend(0..self.n_states);
        self.advance();
    }

    fn offspring(&mut self) {
        // Check for offspring
        if self.cur_token.ttype == TokenType::Null {
            self.advance();
        }
        else {
            let lex = self.consume(TokenType::Number);
            self.cur_rule.offspring = lex.parse().unwrap();
        }
    }

    fn num(&mut self) {
        let str_val = self.consume(TokenType::Number);
        let i_val:i32 = str_val.parse().unwrap();
        self.n_states = i_val;
    }

    fn mov(&mut self) {
        if self.cur_token.ttype == TokenType::Direction {
            self.cur_rule.move_to = BioMove::new_const(self.cur_token.lexeme.chars().nth(0).unwrap());
            self.advance();
        }
        else if self.cur_token.ttype == TokenType::Any {
            self.cur_rule.move_to = BioMove::new_rand();
            self.advance();
        }
        else if self.cur_token.ttype == TokenType::Null {
            self.cur_rule.move_to = BioMove::new_const('_');
            self.advance();
        }
        else {
            self.error(String::from("Invalid MOVE syntax."));
        }
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
        let mut prev_token = String::from("BOF");
        if self.cur_index > 0 {
            prev_token = self.input[self.cur_index - 1].lexeme.clone();
        }
        let mut next_token = String::from("EOF");
        if self.cur_index < self.input.len()-1 {
            next_token = self.input[self.cur_index + 1].lexeme.clone();
        }

        println!("... {:4} {:4} {:4} ...\n\t^ {} at token {} {}", prev_token, self.cur_token.lexeme, next_token, msg, self.cur_index, self.cur_token.lexeme);
        std::process::exit(1);
    }

    pub fn print_results(&self) {
        println!("Parser Results:\n");
        for r in self.rules.iter() {
            r.print();
        }
    }
}