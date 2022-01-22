#[macro_use]
extern crate colour;


use cellm::simple_renderer::SimpleRenderer;
use cellm::tokenizer::{Tokenizer, print_tokens};
use cellm::parser::Parser;
use cellm::bio::RuleSet;
use macroquad::prelude::*;
use cellm::processor::Processor;
use cellm::cli::{parse_args, print_help, Arguments};

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
    // Begin by interpreting the command line arguments.
    let args:Vec<String> = std::env::args().collect();
    let mut p_args = Arguments::new_blank();

    // If there is atleast one command line argument (besides the default working directory one)
    if args.len() > 1 {
        // Try and parse the arguments into p_args
        p_args = parse_args(&args[1..].to_vec()).expect("Invalid Command Line Arguments.");
    }
    else {
        print_help();
        return;
    }
    // Read the inputted source file and tokenize it.
    let mut t = Tokenizer::new_from_file(p_args.file_path.to_string());
    let result = t.start();

    // Check that the result is valid.
    match result {
        Ok(()) => { 
            if p_args.verbose {
                println!("Tokenizer Success.");
                print_tokens(&t.tokens); 
            }
        },
        Err(e) => println!("{}", e)
    };

    // Parse the tokens
    let mut parser = Parser::new(t.tokens);
    parser.start();
    
    // Create the simulation ruleset from the parsed rules.
    let rule_set = RuleSet::new(parser.rules, parser.n_states as usize);
    
    if p_args.verbose {
        // Print success message
        println!("Finished Parse!\nSystem with {} states.", parser.n_states);
        rule_set.print();
    }
    
    // alias the width and height
    let w = parser.render_rules.grid_width.clone();
    let h = parser.render_rules.grid_height.clone();

    // Prepare the processor for simulation.
    let mut processor = Processor::new(rule_set, parser.render_rules);

    // Fill in the grid if the -fill option was used.
    if p_args.fill_state != 0 {
        for y in 0..h {
            for x in 0..w {
                processor.set_cell(p_args.fill_state, x, y);
            }
        }
    }
    processor.gen_random_seed(p_args.gen_states);

    // Load up the default renderer and run the simulation.
    let mut sr = SimpleRenderer::new(0.1);
    loop {
        sr.update(&mut processor).await;
    }
}
