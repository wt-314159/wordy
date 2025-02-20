use clap::{Parser, ValueEnum};

const ANSWERS: &'static str = include_str!("../resources/answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of games to play.
    #[arg(short, long)]
    games: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let num_games = args.games.unwrap_or(usize::MAX);
    let mut scores = Vec::new();
    let mut failed = 0;
    let mut over_6 = 0;
    for answer in ANSWERS.lines().take(num_games) {
        if let Some(score) = wordy::Wordle::play(answer, wordy::algorithms::Entropy::new(), 32) {
            println!("Guessed {} in {}", answer, score);
            scores.push(score);
            if score > 6 {
                over_6 += 1;
            }
        } else {
            println!("Failed to guess {}", answer);
            failed += 1;
        }
    }
    let average = scores.iter().sum::<usize>() as f64 / (num_games - failed) as f64;
    println!(
        "Average score: {:02}, Failed to solve: {}, Solved in more than 6 guesses: {}",
        average, failed, over_6
    );
}
