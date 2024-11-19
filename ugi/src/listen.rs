use std::io;

use crate::go;
use crate::moves;
use crate::position;
use crate::setoption;
use crate::UGI;

pub fn listen(state: &mut impl UGI) -> io::Result<()> {
    println!("id name {}", state.name());
    println!("id author {}", state.author());
    state.print_options();
    println!("ugiok");

    let mut input = String::new();

    // Pre isready setup
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let mut stream = input.split_ascii_whitespace().peekable();
        match stream.next().unwrap_or("") {
            "setoption" => {
                setoption::parse(&mut stream, |name, value| {
                    state.set_option(name, value);
                });
                input.clear();
            }
            "quit" => return Ok(()),
            _ => {
                break;
            }
        }
    }

    state.init();

    // Post isready
    loop {
        let mut stream = input.split_ascii_whitespace().peekable();

        while stream.peek().is_some() {
            match stream.next() {
                Some("uginewgame") => {
                    state.uginewgame();
                }
                Some("isready") => state.isready(),
                Some("setoption") => setoption::parse(&mut stream, |name, value| {
                    state.set_option(name, value);
                }),
                Some("query") => match stream.next() {
                    Some("p1turn") => state.query_p1turn(),
                    Some("gameover") => state.query_gameover(),
                    Some("result") => state.query_result(),
                    _ => {}
                },
                Some("position") => position::parse(&mut stream, |fen| {
                    state.position(fen);
                }),
                Some("moves") => moves::parse(&mut stream, |movestr| {
                    state.moves(movestr);
                }),
                Some("print") => state.print(),
                Some("options") => state.print_options(),
                Some("go") => go::parse(&mut stream, |settings| {
                    state.go(settings);
                }),
                Some("stop") => state.stop(),
                Some("quit") => return Ok(()),
                Some(_) => {}
                _ => return Ok(()),
            }
        }

        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    state.shutdown();

    Ok(())
}
