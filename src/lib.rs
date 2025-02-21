use itertools::iproduct;
pub mod algorithms;

pub struct Wordle {}

impl Wordle {
    // play 6 rounds (or more), trying to guess the answer,
    // return the number of rounds it took to solve
    pub fn play<S: Solver>(
        answer: &'static str,
        mut solver: S,
        max_rounds: usize,
    ) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..=max_rounds {
            let guess = solver.guess(&history);
            if guess == answer {
                return Some(i);
            }
            let result = Tiles::compute(&guess, answer);
            history.push(Guess {
                word: guess,
                result,
            });
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tiles {
    /// Correct
    Green,
    /// Misplaced
    Yellow,
    /// Wrong
    Grey,
}

impl Tiles {
    fn permutations() -> impl Iterator<Item = [Self; 5]> {
        iproduct!(
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow]
        ).map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
    fn compute(guess: &str, answer: &str) -> [Self; 5] {
        let mut used = [false; 5];
        let mut tiles = [Self::Grey; 5];

        // Find characters that match
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                tiles[i] = Self::Green;
                used[i] = true;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            // Skip any chars we've already used (matched correctly)
            if tiles[i] == Tiles::Green {
                continue;
            }
            // Go through every non-used letter in answer, to see
            for (j, a) in answer.chars().enumerate() {
                if used[j] {
                    continue;
                }
                if a == g {
                    tiles[i] = Self::Yellow;
                    used[j] = true;
                    break;
                }
            }
        }

        tiles
    }
}

pub trait Solver {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[derive(Debug)]
pub struct Guess {
    word: String,
    result: [Tiles; 5],
}

struct Candidate {
    word: &'static str,
    score: f64,
}

#[cfg(test)]
mod tests {
    use super::Tiles;

    macro_rules! tiles {
        (X) => {
            Tiles::Grey
        };
        (Y) => {
            Tiles::Yellow
        };
        (G) => {
            Tiles::Green
        };
        ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt) => {
            [
                tiles!($t1),
                tiles!($t2),
                tiles!($t3),
                tiles!($t4),
                tiles!($t5),
            ]
        };
    }

    #[test]
    fn check_tiles_macro() {
        assert_eq!(
            tiles![G Y X G G],
            [
                Tiles::Green,
                Tiles::Yellow,
                Tiles::Grey,
                Tiles::Green,
                Tiles::Green
            ]
        );
    }

    #[test]
    fn all_green() {
        assert_eq!(Tiles::compute("abcde", "abcde"), tiles![G G G G G]);
    }

    #[test]
    fn all_grey() {
        assert_eq!(Tiles::compute("abcde", "fghij"), tiles![X X X X X]);
    }

    #[test]
    fn all_yellow() {
        assert_eq!(Tiles::compute("abcde", "eabcd"), tiles![Y Y Y Y Y]);
    }

    #[test]
    fn one_wrong() {
        assert_eq!(Tiles::compute("ababa", "babab"), tiles![Y Y Y Y X]);
    }

    #[test]
    fn typical_case() {
        assert_eq!(Tiles::compute("hello", "world"), tiles![X X X G Y]);
    }
}
