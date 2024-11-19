use crate::searchresult::SearchResult;
use pijersi::position::Position;
use rand::seq::SliceRandom;
use ugi::go::GoSettings;

pub fn root(pos: &Position, _settings: &GoSettings) -> SearchResult {
    let legal_moves = pos.legal_moves();
    let choice = legal_moves.choose(&mut rand::thread_rng());
    let bestmove = if let Some(mv) = choice {
        Some(*mv)
    } else {
        None
    };

    println!("info depth 1 nodes 1");

    SearchResult {
        bestmove,
        ponder: None,
        nodes: 1,
    }
}
