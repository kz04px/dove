use std::time::Instant;

use crate::{searchresult::SearchResult, searchstats::SearchStats};
use pijersi::{position::Position, result::GameResult, side::Side};
use ugi::go::GoSettings;

const INF_SCORE: i32 = 1_000_000_000;
const MATE_SCORE: i32 = 100_000_000;

#[must_use]
pub fn eval(pos: &Position) -> i32 {
    let us_lower = pos.get_short() & pos.get_us();
    let us_upper = pos.get_tall() & pos.get_us();
    let num_us = us_lower.count() + us_upper.count();

    let them_lower = pos.get_short() & pos.get_them();
    let them_upper = pos.get_tall() & pos.get_them();
    let num_them = them_lower.count() + them_upper.count();

    100 * (num_us - num_them)
}

#[must_use]
pub fn alphabeta(
    should_stop: &dyn Fn(&SearchStats) -> bool,
    stats: &mut SearchStats,
    pos: &Position,
    mut alpha: i32,
    beta: i32,
    depth: i32,
    ply: i32,
) -> i32 {
    debug_assert!(ply >= 0);
    debug_assert!(alpha < beta);

    stats.nodes += 1;

    let is_root = ply == 0;

    match pos.result() {
        Some(GameResult::WhiteWin) => {
            return if pos.turn == Side::White {
                MATE_SCORE - ply
            } else {
                -MATE_SCORE + ply
            };
        }
        Some(GameResult::BlackWin) => {
            return if pos.turn == Side::Black {
                MATE_SCORE - ply
            } else {
                -MATE_SCORE + ply
            };
        }
        Some(GameResult::Draw) => {
            return 0;
        }
        None => {}
    }

    if depth == 0 {
        return eval(pos);
    }

    if should_stop(&stats) {
        return 0;
    }

    let mut best_score = -INF_SCORE;
    let mut best_move = None;

    pos.move_generator(|mv| {
        let npos = pos.after_move(&mv);
        let score = -alphabeta(should_stop, stats, &npos, -beta, -alpha, depth - 1, ply + 1);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            return true;
        }

        false
    });

    debug_assert!(best_move.is_some());

    if is_root {
        stats.bestmove = best_move;
    }

    best_score
}

pub fn root(pos: &Position, settings: &GoSettings) -> SearchResult {
    let mut stats = SearchStats::default();
    let start = Instant::now();
    let mut bestmove = None;

    let should_stop = |stats: &SearchStats| -> bool {
        if let Some(_) = settings.depth {
            return false;
        }

        if let Some(nodes) = settings.nodes {
            return nodes >= stats.nodes;
        }

        if let Some(movetime) = settings.movetime {
            let now = Instant::now();
            let dt = now - start;
            return dt.as_millis() >= movetime as u128;
        }

        false
    };

    let max_depth = if let Some(depth) = settings.depth {
        depth
    } else {
        128
    };

    println!("info string alphabeta search");
    for i in 1..=max_depth {
        let score = alphabeta(&should_stop, &mut stats, pos, -INF_SCORE, INF_SCORE, i, 0);
        let elapsed = start.elapsed();
        let stopped = should_stop(&stats);

        if i > 1 && stopped {
            break;
        }

        bestmove = stats.bestmove;

        print!("info");
        print!(" depth {}", i);
        print!(" score cp {}", score);
        print!(" nodes {}", stats.nodes);
        print!(" time {}", elapsed.as_millis());
        if !elapsed.is_zero() {
            let nps = stats.nodes as f64 / elapsed.as_secs_f64();
            print!(" nps {}", nps as u64);
        }
        if let Some(mv) = stats.bestmove {
            print!(" pv {}", mv);
        }
        println!();
    }

    let elapsed = start.elapsed();
    println!("info nodes {} time {}", stats.nodes, elapsed.as_millis());

    SearchResult {
        bestmove,
        ponder: None,
        nodes: stats.nodes,
    }
}
