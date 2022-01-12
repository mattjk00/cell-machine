use macroquad::prelude::*;
use crate::processor::Processor;


pub struct SimpleRenderer {
    tick_rate:f32,
    timer:f32
}

impl SimpleRenderer {
    pub fn new(tick_rate:f32) -> SimpleRenderer {
        SimpleRenderer { tick_rate: tick_rate, timer:0.0 }
    }

    pub async fn update(&mut self, processor:&mut Processor) {
        clear_background(Color::from_rgba(222, 222, 222, 255));

        let S:f32 = processor.render_rules.cell_size as f32;

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

        self.timer += get_frame_time();
        if self.timer > self.tick_rate {
            processor.step();
            self.timer = 0.0;
        }
        

        next_frame().await
    }
}