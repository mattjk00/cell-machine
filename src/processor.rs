use rand::{prelude::ThreadRng, Rng};

use crate::{bio::{BioRule, RuleSet}, config::RenderRules};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    pub x:usize,
    pub y:usize
}

impl Point {
    pub fn new(x:usize, y:usize) -> Point {
        Point { x, y }
    }

    /// Adds the value of the i32 vector to this point.
    /// Values will not go negative, they will be clamped at zero.
    fn add_v(&mut self, v:V) {
        let mut nx = self.x as i32 + v.x;
        let mut ny = self.y as i32 + v.y;
        if nx < 0 {
            nx = 0;
        }
        if ny < 0 {
            ny = 0;
        }
        self.x = nx as usize;
        self.y = ny as usize;
    }
}

// Simple Vector
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct V {
    x:i32,
    y:i32
}

// Neighbors
// 3 2 1
// 4 * 0
// 5 6 7
const NEIGHBOR_POS:[V; 8] = [V { x:1, y:0 }, V { x:1, y:1 }, V { x:0, y:1 }, V {x:-1, y:1}, V { x:-1, y:0 }, V { x:-1, y:-1}, V { x:0, y:-1 }, V {x:1, y:-1} ];
//const MOVE_VALS:HashMap<char, V> = [ ('l', V {x:-1, y:0}) ].into();

pub struct Processor {
    pub rule_set:RuleSet,
    grid:Vec<Vec<i32>>,
    pub render_rules:RenderRules,
    pub cell_map:HashMap<Point, i32>,     // Keeps track of where active cells are. Does not keep track of state 0, aka dead state.
    rand:ThreadRng
}

impl Processor {
    pub fn new(rules:RuleSet, render_rules:RenderRules) -> Processor {
        let ngrid = vec![vec![0; render_rules.grid_width]; render_rules.grid_height];
        Processor { rule_set:rules, grid:ngrid, render_rules:render_rules, cell_map:HashMap::new(), rand:rand::thread_rng() }
    }

    pub fn set_cell(&mut self, val:i32, x:usize, y:usize) {
        self.grid[y][x] = val;
        self.cell_map.insert(Point::new(x, y), val);
    }

    /// Generates a random starting generation for simulation.
    pub fn gen_random_seed(&mut self, states:Vec<i32>) {
        let count = (self.render_rules.grid_width * self.render_rules.grid_height) / 4;

        for _i in 0..count {
            let mut state:i32 = 0;
            let x = self.rand.gen_range(0..self.render_rules.grid_width);
            let y = self.rand.gen_range(0..self.render_rules.grid_height);
            
            // Generate any state if none were specified for random generation.
            if states.len() == 0 {
                state = self.rand.gen_range(1..self.rule_set.nstates) as i32;
            } else {
                // Pick a random state from the specified list
                state = states[self.rand.gen_range(0..states.len())];
            }
            self.set_cell(state, x, y);
        }
    }

    pub fn step(&mut self) {
        let exec_rules = self.get_exec_rules();
        //println!("Executing {} rules", exec_rules.len());
        

        for rule in exec_rules.iter() {
            //green!("\tRule {} // ", rule.1.calc_hash());
            //e_yellow_ln!("({}, {})", rule.0.x, rule.0.y);
            //rule.1.print();

            // ensure grid is in the correct state.
            if self.grid[rule.0.y][rule.0.x] == rule.1.owner_state {
                self.execute_rule(rule.1, rule.0.to_owned());
            }
            
        }
    }

    /// Checks the conditions for every rule in the rule set, then returns a hashmap stating which rules meet the criteria for execution.
    fn get_exec_rules(&mut self) -> HashMap<Point, BioRule> {
        let mut exec_rules:HashMap<Point, BioRule> = HashMap::new();

        for c in self.cell_map.iter() {
            //let c = self.cell_map.;
            let cell = c.0;
            // Get the state of the current cell.
            let c_state = self.grid[cell.y][cell.x] as usize;
            if c_state == 0 {
                continue; // happens if second rule is trying to be applied to a cell that has already changed.
            }
            // Get the rules that should be applied to this cell.
            let rules = self.rule_set.state_rules(c_state).expect(format!("Unexpected State found in system: {}", c_state).as_str());

            //println!("Cell: {}, {} state: {}, {} rules", cell.x, cell.y, c_state, rules.len());
            

            // Process the rules
            for rule in rules.iter() {
                if rule.any_neighbor == false {
                    // Explicit Neighbor Check
                    for n in &rule.neighbors {
                        // Check individual neighbors for the desired state
                        let neighbor = self.get_neighbor_state(cell.to_owned(), n.to_owned() as usize);
                        match neighbor {
                            Some(s) => {
                                if s.to_owned() == rule.neighbors_state {
                                    
                                    exec_rules.insert(cell.to_owned(), rule.clone());
                                }
                            },
                            None => ()
                        }
                    }
                }
                else if rule.any_neighbor {
                    let mut counted_neighbors = 0;
                    let neighbors = self.get_all_neighbors(cell.to_owned());
                    
                    // Count matching neighbors
                    // Looks at every neighbor and counts how many match the desired state.
                    for n in neighbors.iter() {
                        match n {
                            Some(s) => {
                                if s.to_owned() == rule.neighbors_state {
                                    counted_neighbors += 1;
                                }
                            }
                            None => {}
                        }
                    }
                    if (rule.any_neighbor_exact && counted_neighbors == rule.any_neighbor_count) 
                        || (!rule.any_neighbor_exact && counted_neighbors >= rule.any_neighbor_count) {                        
                        exec_rules.insert(cell.to_owned(), rule.clone());
                    }
                }
            }
            
            
        }
        
        exec_rules
    }

    fn get_neighbor_state(&self, cell:Point, neighbor_n:usize) -> Option<i32> {
        // Calculate neighbor's coordinates
        let nx = NEIGHBOR_POS[neighbor_n].x + cell.x as i32;
        let ny = NEIGHBOR_POS[neighbor_n].y + cell.y as i32;

        // If the neighbor is off the board, return none
        if nx < 0 || ny < 0 || nx >= self.render_rules.grid_width as i32 || ny >= self.render_rules.grid_height as i32 {
            None
        }
        // If it's a valid neighbor, return something
        else {
            Some(self.grid[ny as usize][nx as usize])
        }
    }

    fn get_all_neighbors(&self, cell:Point) -> Vec<Option<i32>> {
        let mut n = vec![];
        for i in 0..8 {
            n.push(self.get_neighbor_state(cell.clone(), i));
        }
        n
    }

    fn execute_rule(&mut self, rule:&BioRule, pos:Point) {
        let offspring = rule.offspring;
        let bmove = rule.move_to.clone();
        
        // Leave the offspring
        self.set_cell(offspring, pos.x, pos.y);

        if !bmove.is_random {
            // Get the constant move
            let translate = Processor::parse_dir(bmove.constant);
            let mut next_pos = pos.clone();
            next_pos.add_v(translate);
            // Move and change to the next state.
            self.set_cell(rule.next_state, next_pos.x, next_pos.y);
        }
        else {
            let mut next_pos = pos.clone();
            while self.grid[next_pos.y][next_pos.x] != 0 {
                next_pos = pos.clone();
                let translate = V { x:self.rand.gen_range(-1..2) as i32, y:self.rand.gen_range(-1..2) as i32 };            
                next_pos.add_v(translate);
            }
            self.set_cell(rule.next_state, next_pos.x, next_pos.y);
        }
    }

    fn parse_dir(dir:char) -> V {
        match dir {
            'r' => V { x:1, y:0 },
            'u' => V { x:0, y:-1 },
            'l' => V { x:-1, y:0 },
            'd' => V { x:0, y:1 },
            _ => V { x:0, y:0 }
        }
    }

}

//#[cfg(tests)]
mod tests {
    use crate::{processor::Point, bio::RuleSet, config::RenderRules};
    use super::Processor;

    fn blank_processor() -> Processor {
        Processor::new(
            RuleSet::new(vec![], 1),
            RenderRules::new_blank()
        )
    }

    #[test]
    fn get_neighbor_state_works() {
        let mut processor = blank_processor();
        processor.set_cell(1, 5, 5); // main cell
        processor.set_cell(2, 5, 6); // Top Neighbor
        processor.set_cell(3, 4, 6); // Upper Left
        let point = Point::new(5, 5);

        assert_eq!(processor.get_neighbor_state(point.clone(), 2).unwrap(), 2); // Top Neighbor
        assert_eq!(processor.get_neighbor_state(point.clone(), 3).unwrap(), 3); // Upper Left
        assert_eq!(processor.get_neighbor_state(point.clone(), 7).unwrap(), 0); // Bottom Right
    }

    #[test]
    fn count_valid_neighbors() {
        let processor = blank_processor();
        // Getting neighbors for cell against the wall. there should be 5 valid neighbors.
        let neighbors = processor.get_all_neighbors(Point::new(0, 2));
        let mut count = 0;
        for n in neighbors.iter() {
            match n {
                Some(_s) => count += 1,
                None => ()
            };
        }
        assert_eq!(count, 5);
    }
}