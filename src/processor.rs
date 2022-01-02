use rand::{prelude::ThreadRng, Rng};

use crate::bio::{BioRule, RuleSet};
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
    grid_width:usize,
    grid_height:usize,
    pub cell_map:HashMap<Point, i32>,     // Keeps track of where active cells are. Does not keep track of state 0, aka dead state.
    rand:ThreadRng
}

impl Processor {
    pub fn new(rules:RuleSet, width:usize, height:usize) -> Processor {
        Processor { rule_set:rules, grid:vec![vec![0; width]; height], grid_width:width, grid_height:height, cell_map:HashMap::new(), rand:rand::thread_rng() }
    }

    pub fn set_cell(&mut self, val:i32, x:usize, y:usize) {
        self.grid[y][x] = val;
        self.cell_map.insert(Point::new(x, y), val);
    }

    pub fn step(&mut self) {
        let exec_rules = self.get_exec_rules();
        println!("Executing {} rules.", exec_rules.len());
        for rule in exec_rules.iter() {
            //println!("\tExecuting rule for state {}.", rule.1.owner_state);
            rule.1.print();
            self.execute_rule(rule.1, rule.0.to_owned());
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
            let rules = self.rule_set.state_rules(c_state);
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
                                    //self.execute_rule(rule, cell);
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
                    if counted_neighbors >= rule.any_neighbor_count {
                        //self.execute_rule(rule, cell.to_owned());
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
        if nx < 0 || ny < 0 || nx >= self.grid_width as i32 || ny >= self.grid_height as i32 {
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
            let translate = V { x:self.rand.gen_range(-1..1) as i32, y:self.rand.gen_range(-1..1) as i32 };
            let mut next_pos = pos.clone();
            next_pos.add_v(translate);
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