mod tokenizer;
use crate::tokenizer::Tokenizer;

fn main() {
    let mut t = Tokenizer::new("states 2\n\n1 *.0 _ ^ 2 #do something here\n1 ^.3 2 @ 1".to_string());
    let result = t.parse();
    
    match result {
        Ok(()) => { 
            println!("Parse Success.");
            t.print_tokens(); 
        },
        Err(e) => println!("{}", e)
    };
}
