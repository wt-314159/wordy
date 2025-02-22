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
    pub fn permutations() -> impl Iterator<Item = [Self; 5]> {
        iproduct!(
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow],
            [Self::Green, Self::Grey, Self::Yellow]
        ).map(|(a, b, c, d, e)| [a, b, c, d, e])
    }

    pub fn compute(guess: &str, answer: &str) -> [Self; 5] {
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

    pub fn matches(guess: &str, answer: &str, pattern: &[Self; 5]) -> bool {
        let mut used = [false; 5];
        for i in 0..5 {
            if guess[i..i+1] == answer[i..i+1] {
                // If the ith character of guess and answer match,
                // but the ith tile in the pattern isn't green, 
                // return false
                if pattern[i] != Self::Green {
                    return false;
                }
                used[i] = true;
            }
            // or if they don't match, but the ith tile is green,
            // also return false
            else if pattern[i] == Self::Green {
                return false;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            // already accounted for
            if pattern[i] == Self::Green {
                continue;
            }
            let mut misplaced = false;
            for (j, a) in answer.chars().enumerate() {
                if used[j] {
                    continue;
                }
                if a == g {
                    // if a character from guess is contained in the answer, but 
                    // the pattern we're comparing to says it isn't, return false
                    if pattern[i] != Self::Yellow {
                        return false;
                    }
                    used[j] = true;
                    misplaced = true;
                    break;
                }
            }
            // If the character from guess didn't match any from the answer,
            // but the pattern says it should have, also return false
            if !misplaced && pattern[i] == Self::Yellow {
                return false;
            }
        }
        true
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
        assert!(Tiles::matches("abcde", "abcde", &tiles![G G G G G]));
        assert!(!Tiles::matches("abcde", "abcde", &tiles![X X X X X]));
        assert!(!Tiles::matches("abcde", "abcde", &tiles![G G G G Y]));
    }

    #[test]
    fn all_grey() {
        assert_eq!(Tiles::compute("abcde", "fghij"), tiles![X X X X X]);
        assert!(Tiles::matches("abcde", "fghij", &tiles![X X X X X]));
        assert!(!Tiles::matches("abcde", "fghij", &tiles![G G G G G]));
    }

    #[test]
    fn all_yellow() {
        assert_eq!(Tiles::compute("abcde", "eabcd"), tiles![Y Y Y Y Y]);
        assert!(Tiles::matches("abcde", "eabcd", &tiles![Y Y Y Y Y]));
        assert!(!Tiles::matches("abcde", "eabcd", &tiles![G G G G G]));
        assert!(!Tiles::matches("abcde", "eabcd", &tiles![X X X X X]));
        assert!(!Tiles::matches("abcde", "eabcd", &tiles![Y Y Y G Y]));
    }

    #[test]
    fn one_wrong() {
        assert_eq!(Tiles::compute("ababa", "babab"), tiles![Y Y Y Y X]);
        assert!(Tiles::matches("ababa", "babab", &tiles![Y Y Y Y X]));
        assert!(!Tiles::matches("ababa", "babab", &tiles![Y Y X Y X]));
        assert!(!Tiles::matches("ababa", "babab", &tiles![Y Y G Y X]));
        assert!(!Tiles::matches("ababa", "babab", &tiles![Y G Y Y X]));
        assert!(!Tiles::matches("ababa", "babab", &tiles![Y Y Y Y Y]));
    }

    #[test]
    fn typical_case() {
        assert_eq!(Tiles::compute("hello", "world"), tiles![X X X G Y]);
        assert!(Tiles::matches("hello", "world", &tiles![X X X G Y]));
        assert!(!Tiles::matches("hello", "world", &tiles![X X X X Y]));
        assert!(!Tiles::matches("hello", "world", &tiles![X X X Y Y]));
        assert!(!Tiles::matches("hello", "world", &tiles![X X Y G Y]));
        assert!(!Tiles::matches("hello", "world", &tiles![X G X G Y]));
    }
}
