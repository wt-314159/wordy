const ANSWERS: &'static str = include_str!("../resources/answers.txt");

fn main() {
    for answer in ANSWERS.lines() {
        wordy::Wordle::play(answer, wordy::algorithms::Naive::new(), 32);
    }
}

