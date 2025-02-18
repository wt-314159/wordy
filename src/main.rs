mod data;
use data::initialisation;

const DICTIONARY: &str = include_str!("../resources/dict.txt");

fn main() {
}

trait Solver {
    fn guess() -> String;
}
