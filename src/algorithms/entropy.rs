use crate::{Guess, Solver, Tiles};

const DICTIONARY: &'static str = include_str!("../../resources/dictionary.txt");

pub struct Entropy {
    remaining: Vec<(&'static str, u32)>,
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
                .parse::<u32>()
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
        if history.is_empty() {
            // Guess the most popular word, this is quite naive as this isn't likely
            // to be the 'best' word, but just taking the easy route for now and
            // looking to optimise later
            return self.remaining.first().unwrap().0.to_string();
        }
        // we need to work out the expected entroy, or given information,
        // of a guess. We do this by multiplying the information of each
        // possible outcome (given by -log2(p)) by the probability of that
        // outcome (given by p, calculated by the number of words remaining
        // given that outcome), and we sum this over every possible outcome
        // N.B. this calculation of probability assumes every word is equally
        // likely, this clearly isn't the case, but it's an ok starting point
        let last_guess = history.last().unwrap();
        self.remaining.retain(|(word, _)| {
            Tiles::compute(&last_guess.word, word) == last_guess.result
        });

        let num_remaining = self.remaining.len();
        // if num_remaining is small enough, say 10 words, go for the win
        if num_remaining < 10 {
            println!("Going for the win!");
            return self.remaining.first().unwrap().0.to_string();
        }
        // instead of checking all words, only check the most likely
        let num_to_check = (num_remaining / 4).max(20);
        let mut best_word = None;
        let highest_score = 0.0;
        for (word, _) in self.remaining.iter().take(num_to_check) {
            let mut sum = 0.0;
            for pattern in Tiles::permutations() {
                let mut matching_words = 0;
                for (pos_word, _) in &self.remaining {
                    if Tiles::compute(word, pos_word) == pattern {
                        matching_words += 1;
                    }
                }
                if matching_words == 0 {
                    continue;
                }

                let probability = matching_words as f64 / num_remaining as f64;
                sum += probability * probability.log2();
            }

            let expected_info = -sum;
            if expected_info > highest_score {
                best_word = Some(word);
                println!("\tBest word is {}, with score {}", word, expected_info);
            }
        }
        if let Some(word) = best_word {
            println!("\tChoosing {}, score {}", word, highest_score);
            return word.to_string();
        }
        eprintln!("No best word found! going with first");
        self.remaining.first().expect("No remaining left!").0.to_owned()
    }
}
