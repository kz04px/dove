use pijersi::position::Position;
use std::time::Instant;

pub fn perft(pos: &Position, depth: i32) -> () {
    let t0 = Instant::now();
    for i in 0..=depth {
        let nodes = pos.perft(i);
        let t1 = Instant::now();
        let dt = t1 - t0;
        if dt.is_zero() {
            println!(
                "info depth {} nodes {} time {}",
                i,
                nodes,
                (t1 - t0).as_millis()
            );
        } else {
            let nps = (nodes as f64 / dt.as_secs_f64()) as u64;
            println!(
                "info depth {} nodes {} time {} nps {}",
                i,
                nodes,
                (t1 - t0).as_millis(),
                nps
            );
        }

        if i == depth {
            println!("nodes {}", nodes);
        }
    }
}
