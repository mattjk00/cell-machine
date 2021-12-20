mod tokenizer;
use crate::tokenizer::Tokenizer;

fn main() {
    let t = Tokenizer::new("abcabc".to_string());
    
    println!("Hello, world!");
}
