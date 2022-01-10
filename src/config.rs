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
    colors:HashMap<i32, u32>,
    pub cell_size:usize,
    pub grid_width:usize,
    pub grid_height:usize,
    seed:Vec<StatePoint>
}

impl RenderRules {
    pub fn new_blank() -> RenderRules {
        RenderRules { colors:HashMap::new(), cell_size:10, grid_width:10, grid_height:10, seed:vec![] }
    }

    pub fn get_colors(&self) -> &HashMap<i32, u32> {
        &self.colors
    }

    pub fn get_color(&self, state:i32) -> u32 {
        self.colors[&state]
    }

    pub fn set_color(&mut self, state:i32, color:u32) {
        self.colors.insert(state, color);
    }

    pub fn add_state_point(&mut self, sp:StatePoint) {
        self.seed.push(sp);
    }
}