use crate::{Solver, Guess};

const DICTIONARY: &'static str = include_str!("../../resources/dictionary.txt");

pub struct Naive {

}

impl Naive {
    pub fn new() -> Self {
        Naive {}
    }
}

impl Solver for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        todo!()
    }
}