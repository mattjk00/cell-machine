use rand::{Rng, prelude::ThreadRng};
const MOVES:[char; 4] = ['l', 'r', 'u', 'd']; // Left Up Right Down

pub struct BioMove {
    is_random:bool,
    constant:char,
    t_rng:ThreadRng
}

impl BioMove {
    fn new_const(m:char, rng:ThreadRng)  -> BioMove {
        BioMove { is_random:false, constant:m, t_rng:rng }
    }
    fn new_rand(rng:ThreadRng) -> BioMove {
        BioMove { is_random:true, constant:'\0', t_rng:rng }
    }
    fn value(&mut self) -> char {
        let rand:usize = self.t_rng.gen_range(0..4) as usize;
        match self.is_random {
            false => self.constant,
            true => MOVES[rand]
        }
    }
}

pub struct BioRule {
    neighbors:Vec<i32>,
    neighbors_state:i32,
    owner_state:i32,
    next_state:i32,
    move_to:BioMove
}