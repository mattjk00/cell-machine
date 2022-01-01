use rand::{Rng, prelude::ThreadRng};
const MOVES:[char; 4] = ['l', 'r', 'u', 'd']; // Left Up Right Down

#[derive(Clone)]
pub struct BioMove {
    pub is_random:bool,
    pub constant:char
}

impl BioMove {
    pub fn new_const(m:char)  -> BioMove {
        BioMove { is_random:false, constant:m }
    }
    pub fn new_rand() -> BioMove {
        BioMove { is_random:true, constant:'^' } 
    }
    /*fn value(&mut self) -> char {
        let rand:usize = self.t_rng.gen_range(0..4) as usize;
        match self.is_random {
            false => self.constant,
            true => MOVES[rand]
        }
    }*/
}
#[derive(Clone)]
pub struct BioRule {
    pub neighbors:Vec<i32>,     // Neighbors to check out
    pub neighbors_state:i32,    // What neighbor's state should be
    pub any_neighbor:bool,      // Set true if the rule could be about ANY neighbor
    pub any_neighbor_count:i32, // How many neighbors. If 0, then all neighbors
    pub any_neighbor_state:bool,// Set true if the neighbor's state can be anything
    pub owner_state:i32,        // What the owner's state should be
    pub next_state:i32,         // Transorm state
    pub move_to:BioMove,        // Where to move after rule,
    pub offspring:i32           // What the cell should leave behind if moving
}

impl BioRule {
    pub fn new_blank() -> BioRule {
        BioRule { neighbors:vec![], neighbors_state:0, owner_state:0,
             next_state:0, move_to:BioMove::new_const('_'),
            any_neighbor:false,
            any_neighbor_state:false, offspring:0, any_neighbor_count:1 }
    }

    pub fn print(&self) {
        print!("State {} Rule:\n\t", self.owner_state);
        print!("Neighbors: ");
        if self.any_neighbor == false {
            for n in self.neighbors.iter() {
                print!("{} ", n);
            }
        }
        else {
            print!("Any {}", self.any_neighbor_count);
        }
        print!("\n\t");
        println!("NState: {}\tMove:{}\tOffspring:{}\tNext:{}", self.neighbors_state, self.move_to.constant, self.offspring, self.next_state);
        
    }
}

/// RuleSet is a helper data structure for organizing a list of bio rules.
/// Given an unsorted vector of biorules, RuleSet will organize these into a 2d vector
/// with form { { n, n, n}, {m, m, m}, ... } where n is state 1 rules, m is state 2 rules, etc...
#[derive(Clone)]
pub struct RuleSet {
    rules:Vec<Vec<BioRule>>,
    nstates:usize
}

impl RuleSet {
    pub fn new(rules:Vec<BioRule>, nstates:usize) -> RuleSet {
        // size nstates - 1 because we won't store rules for state 0.
        let mut rs = RuleSet { rules:vec![vec![]; nstates - 1], nstates:nstates };
        println!("BUILDING RULESET: {}", rules.len());
        for r in &rules {
            rs.rules[ (r.owner_state - 1) as usize].push(r.clone());
        }
        println!("rs.rules[0].len() = {}", rs.rules[0].len());

        rs
    }

    /// Returns a list of the rules for a given state.
    pub fn state_rules(&self, state:usize) -> &Vec<BioRule> {
        &self.rules[state - 1]
    }

    pub fn print(&self) {
        println!("{} state RuleSet:", self.nstates);
        for i in 0..self.rules.len() {
            print!("{}: ", i);
            for j in 0..self.rules[i].len() {
                self.rules[i][j].print();
            }
            println!();
        }
    }
}