use pijersi::position::Position;

pub fn split(pos: &Position, depth: i32) -> () {
    let moves = pos.legal_moves();
    let mut total_nodes = 0;

    for mv in moves {
        let npos = pos.after_move(&mv);
        let nodes = npos.perft(depth - 1);
        total_nodes += nodes;

        println!("{} {}", mv, nodes);
    }

    println!("nodes {}", total_nodes);
}
