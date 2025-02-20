const ANSWERS: &'static str = include_str!("../resources/answers.txt");

fn main() {
    let num_games = 200;
    let mut scores = Vec::with_capacity(num_games);
    let mut failed = 0;
    for answer in ANSWERS.lines().take(num_games) {
        if let Some(score) = wordy::Wordle::play(answer, wordy::algorithms::Naive::new(), 32) {
            println!("Guessed {} in {}", answer, score);
            scores.push(score);
        }
        else {
            println!("Failed to guess {}", answer);
            failed += 1;
        }
    }
    let average = scores.iter().sum::<usize>() as f64 / (num_games - failed) as f64;
    println!("Average score: {:02}, Failed to solve: {}", average, failed);
}

