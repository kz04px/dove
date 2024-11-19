use crate::benchmark::benchmark;
use ugi::listen::listen;

mod alphabeta;
mod benchmark;
mod negamax;
mod perft;
mod protocol;
mod random;
mod searchresult;
mod searchstats;
mod split;
mod state;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    match input.trim() {
        "ugi" => {
            let mut state = state::EngineState::default();
            let _ = listen(&mut state);
        }
        "bench" | "benchmark" => {
            benchmark();
        }
        "about" => {
            if cfg!(debug_assertions) {
                println!("Debug enabled");
            }
        }
        "quit" => return,
        _ => println!("Unknown protocol"),
    };
}
