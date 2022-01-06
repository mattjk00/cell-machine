use std::collections::HashMap;

pub struct StatePoint {
    x:usize,
    y:usize,
    state:i32
}

impl StatePoint {
    pub fn new(x:usize, y:usize, state:i32) -> StatePoint {
        StatePoint {x:x, y:y, state:state}
    }
}

pub struct RenderRules {
    colors:HashMap<i32, i32>,
    pub cell_size:usize,
    pub grid_width:usize,
    pub grid_height:usize,
    seed:Vec<StatePoint>
}

impl RenderRules {
    pub fn new_blank() -> RenderRules {
        RenderRules { colors:HashMap::new(), cell_size:0, grid_width:0, grid_height:0, seed:vec![] }
    }

    pub fn get_colors(&self) -> &HashMap<i32, i32> {
        &self.colors
    }

    pub fn set_color(&mut self, state:i32, color:i32) {
        self.colors.insert(state, color);
    }

    pub fn add_state_point(&mut self, sp:StatePoint) {
        self.seed.push(sp);
    }
}