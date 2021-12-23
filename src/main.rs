mod tokenizer;
mod parser;
mod bio;
use crate::tokenizer::{Tokenizer, print_tokens};
use crate::parser::Parser;
use rand::Rng;

fn main() {
    let mut t = Tokenizer::new("states\t4 \n1 *.0 _ ^ 2".to_string());
    let result = t.start();

    let mut rng = rand::thread_rng();
    
    match result {
        Ok(()) => { 
            println!("Parse Success.");
            //print_tokens(&t.tokens); 
        },
        Err(e) => println!("{}", e)
    };

    let mut parser = Parser::new(t.tokens);
    parser.start();
    //print_tokens(&parser.input);
}
