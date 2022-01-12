use colour::red;

use crate::{tokenizer::{Token, TokenType, print_tokens}, bio::{BioRule, BioMove}, config::RenderRules};
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
    pub rules: Vec<BioRule>,
    pub render_rules: RenderRules,
    cur_rule:BioRule,
    cur_token:Token,
    cur_index:usize,
    pub n_states:i32
}

impl Parser {
    /// Create a new parser with given token input.
    pub fn new(inp:Vec<Token>) -> Parser {
        let curt:Token = inp[0].clone();
        Parser { input:inp, rules:vec![], cur_rule:BioRule::new_blank(), cur_index:0, cur_token:curt, n_states:0, render_rules:RenderRules::new_blank() }
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
        if self.cur_token.ttype != TokenType::EOF && self.cur_token.lexeme == "render" {
            self.advance();
            self.parse_render_section();
        }
        

        println!("Finished Parse!\nSystem with {} states.", self.n_states);
    }

    // ---- Parsing Functions ---- //

    fn sys(&mut self) {
        // Begin looking for the 'states' keyword
        if self.cur_token.lexeme == "states" {
            self.advance();
            self.num();
            self.consume(TokenType::Newline);
        }
        // Look through rule definitions
        while self.cur_token.ttype == TokenType::Number || self.cur_token.ttype == TokenType::Newline {
            // Skip over blank lines
            if self.cur_token.ttype == TokenType::Newline {
                self.advance();
                continue;
            }

            // Parse the first part of the rule, the owner state.
            self.cur_rule.owner_state = self.cur_token.lexeme.parse().expect("Invalid token lexeme for owner state.");

            // Rules for state 0 are not allowed
            if self.cur_rule.owner_state == 0 {
                self.error("Rules for state 0 (Dead State) are not permitted.".to_string());
            }

            // Parse the next chunk of the rule definition.
            self.advance();
            self.neigh();
            self.offspring();
            self.mov();

            // Parse the 'next state' part of the rule.
            let lex = self.consume(TokenType::Number);
            self.cur_rule.next_state = lex.parse().expect("Invalid next state for rule!");
            // Save the parsed rule.
            self.rules.push(self.cur_rule.clone());
            // Reset the rule for the next parser pass
            self.cur_rule = BioRule::new_blank();

            self.advance();
            
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
                "^" => self.any_neigh(false),
                "=" => self.any_neigh(true),
                "*" => self.all_neigh(),
                _ => self.error(String::from("Expecting number or operator."))
            };
            //self.advance();
            self.consume(TokenType::Dot);
            let lex = self.consume(TokenType::Number);
            self.cur_rule.neighbors_state = lex.parse().unwrap();
        }
        
        
    }

    fn any_neigh(&mut self, exact:bool) {
        self.cur_rule.any_neighbor = true;
        self.cur_rule.any_neighbor_exact = exact;
        self.advance();
        
        if self.cur_token.ttype == TokenType::Number {
            let num:i32 = self.cur_token.lexeme.parse().unwrap();
            self.advance();
            self.cur_rule.any_neighbor_count = num;
        }
    }

    fn all_neigh(&mut self) {
        self.cur_rule.neighbors.extend(0..8);
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
        else if self.cur_token.ttype == TokenType::Absorb {
            self.cur_rule.move_to = BioMove::new_const('@');
            self.advance();
        }
        else {
            self.error(String::from("Invalid MOVE syntax."));
        }
    }

    fn parse_sizes(&mut self) {
        for i in 0..3 {
            let size_result = usize::from_str_radix(&self.cur_token.lexeme, 10);
            let mut size = 0;
            match size_result {
                Ok(s) => size = s,
                Err(e) => self.error("Invalid size parameter for render section!".to_string())
            };
            red!("PARSING SIZE {}", size);

            match i {
                0 => self.render_rules.cell_size = size,
                1 => self.render_rules.grid_width = size,
                2 => self.render_rules.grid_height = size,
                _ => ()
            };
            self.advance();
        }
        self.consume(TokenType::Newline);
    }

    fn parse_render_section(&mut self) {

        self.parse_sizes();

        while self.cur_token.ttype == TokenType::Number || self.cur_token.ttype == TokenType::Newline {

            while self.cur_token.ttype == TokenType::Newline {
                self.advance();
            }
            if self.cur_token.ttype == TokenType::EOF {
                self.advance();
                break;
            }

            // Attempt to parse the state
            let state_result = i32::from_str_radix(&self.cur_token.lexeme, 10);
            let mut state = 0;
            match state_result {
                Ok(s) => state = s,
                Err(e) => self.error("Invalid state definition in render section!".to_string())
            };
            self.advance();

            // Attempt to parse the color for the state
            let color_result = u32::from_str_radix(&self.cur_token.lexeme, 16);
            let mut color:u32 = 0;

            match color_result {
                Ok(c) => color = c,
                Err(e) => self.error(format!("Invalid 32 bit color assignment for state {}", state))
            };
            self.advance();
            self.render_rules.set_color(state, color);
            
            // If end of file reached, advance on and end. Otherwise, expect a newline char.
            /*if self.cur_token.ttype == TokenType::EOF {
                self.advance();
            }
            else {
                self.consume(TokenType::Newline);
            }*/
        }
    }

    // ---- End Parsing Functions ---- //    

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

    #[allow(dead_code)]
    pub fn print_results(&self) {
        println!("Parser Results:\n");
        for r in self.rules.iter() {
            r.print();
        }
    }
}