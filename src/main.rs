mod tokenizer;
mod parser;
mod bio;
mod processor;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use bio::RuleSet;
use macroquad::prelude::*;
use macroquad::prelude::coroutines::wait_seconds;
use processor::Processor;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cell-Machine".to_owned(),
        fullscreen: false,
        window_width:640,
        window_height:640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut t = Tokenizer::new("states 3\n2 ^3.1 _ _ 1\n2 ^2.2 _ _ 2\n2 ^3.2 _ _ 2\n2 ^4.2 _ _ 1\n1 ^3.2 _ _ 2".to_string());
    let result = t.start();

    let rng = ::rand::thread_rng();
    
    match result {
        Ok(()) => { 
            println!("Parse Success.");
            //print_tokens(&t.tokens); 
        },
        Err(e) => println!("{}", e)
    };

    let mut parser = Parser::new(t.tokens, rng);
    parser.start();
    ..parser.print_results();
    //print_tokens(&parser.input);

    let size:usize = 32;
    let rule_set = RuleSet::new(parser.rules, parser.n_states as usize);
    
    //rule_set.print();
    let mut processor = Processor::new(rule_set, size, size);
    for y in 0..size {
        for x in 0..size {
            processor.set_cell(1, x, y);
        }
    }
    processor.set_cell(2, 9, 9);
    processor.set_cell(2, 10, 9);
    processor.set_cell(2, 9, 10);
    
    let mut timer:f32 = 0.0;

    loop {
        clear_background(BLUE);

        
        for p in &processor.cell_map {
            let state = p.1.to_owned();
            let pos = p.0.to_owned();
            let mut color = BLACK;
            if state == 2 {
                color = GREEN;
            }

            draw_rectangle(pos.x as f32 * 20.0, pos.y as f32 * 20.0, 20.0, 20.0, color);
        }

        for i in 0..size {
            let fi = i as f32;
            draw_line(fi * 20.0, 0.0, fi * 20.0, 640.0, 1.0, BLACK);
        }
        for i in 0..size {
            let fi = i as f32;
            draw_line(0.0, fi * 20.0, 640.0, fi * 20.0, 1.0, BLACK);
        }

        timer += get_frame_time();
        if timer > 1.0 {
            processor.step();
            timer = 0.0;
        }
        

        next_frame().await
    }
}
