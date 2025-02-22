use clap::{Parser, ValueEnum};

const ANSWERS: &'static str = include_str!("../resources/answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of games to play.
    #[arg(short, long)]
    games: Option<usize>,

    #[arg(value_enum)]
    algorithm: Algorithm
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Algorithm {
    /// Chooses the option that gives us the most information.
    Entropy,
    /// Chooses the most common word that matches the history of guesses.
    Naive
}

fn main() {
    let args = Args::parse();
    let num_games = args.games.unwrap_or(usize::MAX);
    match dbg!(args.algorithm) {
        Algorithm::Entropy => play(wordy::algorithms::Entropy::new, num_games),
        Algorithm::Naive => play(wordy::algorithms::Naive::new, num_games)
    }
}

fn play<S>(mut maker: impl FnMut() -> S, games: usize) 
where 
    S: wordy::Solver,
{
    let mut scores = Vec::new();
    let mut failed = 0;
    let mut over_6 = 0;
    let mut game_count = 0;
    for answer in ANSWERS.lines().take(games) {
        if let Some(score) = wordy::Wordle::play(answer, maker(), 32) {
            println!("Guessed {} in {}", answer, score);
            scores.push(score);
            if score > 6 {
                over_6 += 1;
            }
        } else {
            println!("Failed to guess {}", answer);
            failed += 1;
        }
        game_count +=1;
    }
    let average = scores.iter().sum::<usize>() as f64 / (game_count - failed) as f64;
    println!(
        "Average score: {:.2}, Failed to solve: {}, Solved in more than 6 guesses: {}",
        average, failed, over_6
    );
}