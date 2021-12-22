use crate::{tokenizer::Token, bio::BioRule};


pub struct Parser {
    input: Vec<Token>,
    rules: Vec<BioRule>
}