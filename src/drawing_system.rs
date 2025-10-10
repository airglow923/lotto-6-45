use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use std::vec::Vec;

#[derive(Debug)]
pub struct DrawingSystem {
    rng: ThreadRng,
    pub balls: Vec<usize>,
}

impl DrawingSystem {
    pub fn new(n_balls: usize) -> Self {
        Self {
            rng: rand::rng(),
            balls: (1..=n_balls).collect(),
        }
    }

    pub fn draw(&mut self) -> usize {
        self.balls.pop().unwrap_or(0)
    }

    pub fn shuffle(&mut self) {
        self.balls.shuffle(&mut self.rng);
    }
}
