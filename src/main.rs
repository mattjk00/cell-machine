#[macro_use]
extern crate colour;


use cellm::simple_renderer::SimpleRenderer;
use cellm::tokenizer::{Tokenizer, print_tokens};
use cellm::parser::Parser;
use cellm::bio::RuleSet;
use macroquad::prelude::*;
use cellm::processor::Processor;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cell-Machine".to_owned(),
        fullscreen: false,
        window_width:640,
        window_height:640,
        window_resizable:false,
        icon:None,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //let mut t = Tokenizer::new("states 3\n2 ^3.1 _ _ 1\n2 ^2.2 _ _ 2\n2 ^3.2 _ _ 2\n2 ^4.2 _ _ 1\n1 ^3.2 _ _ 2".to_string());
    let mut t = Tokenizer::new_from_file("examples/conway.cell".to_string());
    let result = t.start();

    
    match result {
        Ok(()) => { 
            println!("Tokenizer Success.");
            print_tokens(&t.tokens); 
        },
        Err(e) => println!("{}", e)
    };

    let mut parser = Parser::new(t.tokens);
    parser.start();
    //..parser.print_results();
    //print_tokens(&parser.input);

    let rule_set = RuleSet::new(parser.rules, parser.n_states as usize);
    rule_set.print();

    red_ln!("W: {}, H: {}", parser.render_rules.grid_width, parser.render_rules.grid_height);

    let w = parser.render_rules.grid_width.clone();
    let h = parser.render_rules.grid_height.clone();
    //rule_set.print();
    let mut processor = Processor::new(rule_set, parser.render_rules);
    for y in 0..h {
        for x in 0..w {
            processor.set_cell(2, x, y);
        }
    }
    /*processor.set_cell(1, 9, 9);
    processor.set_cell(2, 10, 9);
    processor.set_cell(1, 9, 10);
    processor.set_cell(1, 9, 11);
    processor.set_cell(1, 10, 11);
    processor.set_cell(1, 10, 10);
    processor.set_cell(1, 10, 12);
    processor.set_cell(1, 11, 12);*/
    processor.gen_random_seed();

    //processor.step();
    //processor.step();

    let mut sr = SimpleRenderer::new(0.1);
    
    loop {
        sr.update(&mut processor).await;
    }
}
