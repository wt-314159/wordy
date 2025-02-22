use crate::{Candidate, Guess, Solver, Tiles};

const DICTIONARY: &'static str = include_str!("../../resources/dictionary.txt");

pub struct Entropy {
    remaining: Vec<(&'static str, u64)>,
}

impl Entropy {
    pub fn new() -> Self {
        let mut remaining = Vec::new();
        for line in DICTIONARY.lines() {
            let mut parts = line.split_whitespace();
            let word = parts.next().expect("Empty line");
            let freq = parts
                .next()
                .expect(&format!("Word without frequency: {line}"));
            let freq = freq
                .parse::<u64>()
                .expect(&format!("Failed to parse frequency {freq}"));
            remaining.push((word, freq));
        }
        // Sort from most frequent to least frequent, will be helpful later on
        remaining.sort_by(|(_, c1), (_, c2)| c1.cmp(c2).reverse());
        Entropy { remaining }
    }
}

impl Solver for Entropy {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|(w, _)| {
                Tiles::compute(&last.word, w) == last.result
            })
        }
        else {
            // Guess a known good first guess
            return "tares".to_string();
        }

        let num_remaining = self.remaining.len();
        // if num_remaining is small enough, say 10 words, go for the win
        if num_remaining < 10 {
            return self.remaining.first().unwrap().0.to_string();
        }
        // instead of checking all words, only check the most likely
        let num_to_check = (num_remaining / 4).max(20);
        let remaining_frequency: u64 = self.remaining.iter().map(|(_, c)| *c).take(num_to_check).sum();
        let mut best:Option<Candidate> = None; 
        for (word, _) in self.remaining.iter().take(num_to_check) {
            let mut sum = 0.0;
            let mut pattern_frequency = 0;
            for pattern in Tiles::permutations() {
                for (candidate, count) in &self.remaining {
                    if Tiles::compute(word, candidate) == pattern {
                        pattern_frequency += count;
                    }
                }
                
                if pattern_frequency == 0 {
                    continue;
                }
                let probability = pattern_frequency as f64 / remaining_frequency as f64;
                sum += probability * probability.log2();
            }

            let sum = -sum;
            if let Some(b) = &best {
                if sum > b.score {
                    best = Some(Candidate { word, score: sum});
                }
            }
        }
        if let Some(b) = best {
            return b.word.to_string();
        }
        self.remaining.first().expect("No remaining left!").0.to_owned()
    }
}
