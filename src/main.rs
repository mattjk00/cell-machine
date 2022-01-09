#[macro_use]
extern crate colour;
mod tokenizer;
mod parser;
mod bio;
mod processor;
mod config;

use crate::tokenizer::{Tokenizer, print_tokens};
use crate::parser::Parser;
use bio::RuleSet;
use macroquad::prelude::*;
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
    //let mut t = Tokenizer::new("states 3\n2 ^3.1 _ _ 1\n2 ^2.2 _ _ 2\n2 ^3.2 _ _ 2\n2 ^4.2 _ _ 1\n1 ^3.2 _ _ 2".to_string());
    let mut t = Tokenizer::new_from_file("conway.cell".to_string());
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

    let size:usize = 32;
    let rule_set = RuleSet::new(parser.rules, parser.n_states as usize);
    rule_set.print();

    red_ln!("W: {}, H: {}", parser.render_rules.grid_width, parser.render_rules.grid_height);

    //rule_set.print();
    let mut processor = Processor::new(rule_set, parser.render_rules);
    for y in 0..size {
        for x in 0..size {
            processor.set_cell(2, x, y);
        }
    }
    processor.set_cell(1, 9, 9);
    processor.set_cell(2, 10, 9);
    processor.set_cell(1, 9, 10);
    processor.set_cell(1, 9, 11);
    processor.set_cell(1, 10, 11);
    processor.set_cell(1, 10, 10);
    processor.set_cell(1, 10, 12);
    processor.set_cell(1, 11, 12);

    //processor.step();
    //processor.step();
    
    let mut timer:f32 = 0.0;

    let S:f32 = processor.render_rules.cell_size as f32;
    loop {
        clear_background(BLUE);

        
        for p in &processor.cell_map {
            let state = p.1.to_owned();
            let pos = p.0.to_owned();
            // Color stuff
            let color_data = processor.render_rules.get_color(state);
            let color_bytes = color_data.to_be_bytes();
            //red_ln!("Color: {}, {}, {}, {}", color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3]);
            let color = Color::from_rgba(color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3]);
            /*if state == 1 {
                color = YELLOW;
            }*/
            

            draw_rectangle(pos.x as f32 * S, pos.y as f32 * S, S, S, color);
        }

        for i in 0..processor.render_rules.grid_width {
            let fi = i as f32;
            draw_line(fi * S, 0.0, fi * S, 640.0, 1.0, BLACK);
        }
        for i in 0..processor.render_rules.grid_height {
            let fi = i as f32;
            draw_line(0.0, fi * S, 640.0, fi * S, 1.0, BLACK);
        }

        timer += get_frame_time();
        if timer > 0.1 {
            processor.step();
            timer = 0.0;
        }
        

        next_frame().await
    }
}
