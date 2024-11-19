use pijersi::position::Position;
use std::time::{Duration, Instant};
use ugi::go::GoSettings;

use crate::random;

pub fn benchmark() {
    let positions = [
        "startpos",
        "srp-r-s-2/p-1s-1r-1p-/4p-1/1RPw-3sr/2w-1RSP-/P-2WWS-2/R-S-S-1PR1 w 0 1",
        "1p-sr1p-r-/p-r-1w-w-s-p-/4r-1/3ss3/SPR-S-2R-/P-2WW2P-/R-2R-SPS- w 0 1",
    ];

    println!(
        "{:<3}  {:>10}  {:>12}  {:>10}  {:>10}  {:>6}  {:>7}   {}",
        "Pos", "Nodes", "ΣNodes", "NPS", "ΣNPS", "Time", "ΣTime", "FEN"
    );

    let mut total_nodes = 0;
    let mut total_time = Duration::default();
    for (idx, fen) in positions.iter().enumerate() {
        let pos = Position::from_fen(fen);
        let t0 = Instant::now();
        let result = random::root::root(&pos, &GoSettings::from_depth(6));
        let elapsed = t0.elapsed();

        total_nodes += result.nodes;
        total_time += elapsed;
        let nps = if elapsed.is_zero() {
            0
        } else {
            (result.nodes as f32 / elapsed.as_secs_f32()) as u64
        };
        let total_nps = if total_time.is_zero() {
            0
        } else {
            (total_nodes as f32 / total_time.as_secs_f32()) as u64
        };

        println!(
            "{:<3}  {:>10}  {:>12}  {:>10}  {:>10}  {:>6.3}  {:>7.3}   {}",
            idx + 1,
            result.nodes,
            total_nodes,
            nps,
            total_nps,
            elapsed.as_secs_f32(),
            total_time.as_secs_f32(),
            fen
        );
    }

    let millis = total_time.subsec_millis();
    let seconds = total_time.as_secs() % 60;
    let minutes = (total_time.as_secs() / 60) % 60;
    let hours = (total_time.as_secs() / 60) / 60;
    println!(
        "time {:0>2}:{:0>2}:{:0>2}.{:0>3}",
        hours, minutes, seconds, millis
    );
}
