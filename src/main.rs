#[macro_use]
extern crate colour;


use cellm::simple_renderer::SimpleRenderer;
use cellm::tokenizer::{Tokenizer, print_tokens};
use cellm::parser::Parser;
use cellm::bio::RuleSet;
use macroquad::prelude::*;
use cellm::processor::Processor;
use cellm::cli::parse_args;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cell-Machine".to_owned(),
        fullscreen: false,
        window_width:640,
        window_height:640,
        window_resizable:true,
        icon:None,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args:Vec<String> = std::env::args().collect();
    
    let p_args = parse_args(&args[1..].to_vec()).expect("Invalid Command Line Arguments.");


    let mut t = Tokenizer::new_from_file(p_args.file_path.to_string());
    let result = t.start();

    
    match result {
        Ok(()) => { 
            if p_args.verbose {
                println!("Tokenizer Success.");
                print_tokens(&t.tokens); 
            }
        },
        Err(e) => println!("{}", e)
    };

    let mut parser = Parser::new(t.tokens);
    parser.start();

    let rule_set = RuleSet::new(parser.rules, parser.n_states as usize);
    
    if p_args.verbose {
        println!("Finished Parse!\nSystem with {} states.", parser.n_states);
        rule_set.print();
    }
    

    let w = parser.render_rules.grid_width.clone();
    let h = parser.render_rules.grid_height.clone();

    let mut processor = Processor::new(rule_set, parser.render_rules);
    for y in 0..h {
        for x in 0..w {
            processor.set_cell(2, x, y);
        }
    }
    processor.gen_random_seed();

    let mut sr = SimpleRenderer::new(0.1);
    
    loop {
        sr.update(&mut processor).await;
    }
}
