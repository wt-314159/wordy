use crate::{Guess, Solver, Tiles};

const DICTIONARY: &'static str = include_str!("../../resources/dictionary.txt");

pub struct Naive {
    remaining: Vec<(&'static str, u32)>,
}

impl Naive {
    pub fn new() -> Self {
        let mut remaining = Vec::new();
        for line in DICTIONARY.lines() {
            let mut parts = line.split_whitespace();
            let word = parts.next().expect("Empty line");
            let freq = parts
                .next()
                .expect(&format!("Word without frequency: {line}"));
            let freq = freq
                .parse::<u32>()
                .expect(&format!("Failed to parse frequency {freq}"));
            remaining.push((word, freq));
        }
        // Sort from most frequent to least frequent, will be helpful later on
        remaining.sort_by(|(_, c1), (_, c2)| c1.cmp(c2).reverse());
        Naive { remaining }
    }
}

impl Solver for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if history.len() == 0 {
            // Guess the most popular word, this is quite naive as this isn't likely
            // to be the 'best' word, but just taking the easy route for now and
            // looking to optimise later
            return self.remaining.first().unwrap().0.to_string();
        } else {
            // Need to use the last guess and it's results to reduce remaining
            // Check how the last guess's word computes against each remaining
            // word, if the resulting pattern of tiles is the same as resulted
            // last guess when computed against actual answer, keep candidate
            let last_guess = history.last().unwrap();
            self.remaining.retain(|(word, _)| {
                Tiles::matches(&last_guess.word, word, &last_guess.result)
            });
            return self.remaining.first().unwrap().0.to_string();
        }
    }
}
