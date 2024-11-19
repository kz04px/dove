use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&str) -> ()) {
    // Parse startpos/fen
    let fen = match stream.next() {
        Some("startpos") => "startpos".to_string(),
        Some("fen") => {
            let mut fen = String::new();
            while stream.peek().is_some() && stream.peek().unwrap() != &"moves" {
                if !fen.is_empty() {
                    fen += " ";
                }
                fen += stream.next().unwrap();
            }
            fen
        }
        _ => "".to_string(),
    };

    // Set FEN
    func(&fen);
}
