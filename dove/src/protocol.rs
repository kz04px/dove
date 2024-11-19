use crate::{alphabeta, negamax, perft::perft, random, split::split, state::EngineState};
use pijersi::{position::Position, result::GameResult, side::Side};
use ugi::{
    go::{GoKind, GoSettings},
    result::UGIResult,
    UGI,
};

impl UGI for EngineState {
    fn init(&mut self) -> () {
        if self.debug.value {
            println!("info string init begin");
        }

        if self.debug.value {
            println!("info string init end");
        }

        self.uginewgame();
    }

    fn shutdown(&mut self) -> () {
        if self.debug.value {
            println!("info string shutdown");
        }
    }

    fn name(&self) -> &str {
        "Dove"
    }

    fn author(&self) -> &str {
        "kz04px"
    }

    fn uginewgame(&mut self) -> () {
        if self.debug.value {
            println!("info string new game");
        }

        self.pos = Position::from_fen("startpos");
    }

    fn result(&mut self) -> Option<UGIResult> {
        match self.pos.result() {
            Some(GameResult::WhiteWin) => Some(UGIResult::P1win),
            Some(GameResult::BlackWin) => Some(UGIResult::P2win),
            Some(GameResult::Draw) => Some(UGIResult::Draw),
            None => None,
        }
    }

    fn isready(&mut self) -> () {
        println!("readyok");
    }

    fn position(&mut self, fen: &str) -> () {
        if self.debug.value {
            println!("info string set fen '{}'", fen);
        }

        self.pos.set_fen(fen);
    }

    fn moves(&mut self, movestr: &str) -> () {
        if self.debug.value {
            println!("info string apply move '{}'", movestr);
        }

        let moves = self.pos.legal_moves();
        for mv in moves {
            if mv.to_string() == movestr {
                self.pos.makemove(&mv);
                return;
            }
        }
    }

    fn go(&mut self, settings: &GoSettings) -> () {
        if self.debug.value {
            println!("info string go {}", settings);
        }

        match settings.kind {
            GoKind::Search => {
                match self.search.value.as_str() {
                    "random" => {
                        let result = random::root::root(&self.pos, &settings);

                        match (result.bestmove, result.ponder) {
                            (Some(best), Some(ponder)) => {
                                println!("bestmove {} ponder {}", best, ponder)
                            }
                            (Some(best), None) => println!("bestmove {}", best),
                            (None, Some(_)) => panic!("ponder but no bestmove"),
                            (None, None) => println!("bestmove 0000"),
                        }
                    }
                    "negamax" => {
                        let result = negamax::root::root(&self.pos, &settings);

                        match (result.bestmove, result.ponder) {
                            (Some(best), Some(ponder)) => {
                                println!("bestmove {} ponder {}", best, ponder)
                            }
                            (Some(best), None) => println!("bestmove {}", best),
                            (None, Some(_)) => panic!("ponder but no bestmove"),
                            (None, None) => println!("bestmove 0000"),
                        }
                    }
                    "alphabeta" => {
                        let result = alphabeta::root::root(&self.pos, &settings);

                        match (result.bestmove, result.ponder) {
                            (Some(best), Some(ponder)) => {
                                println!("bestmove {} ponder {}", best, ponder)
                            }
                            (Some(best), None) => println!("bestmove {}", best),
                            (None, Some(_)) => panic!("ponder but no bestmove"),
                            (None, None) => println!("bestmove 0000"),
                        }
                    }
                    _ => {}
                };
            }
            GoKind::Perft => {
                if let Some(d) = settings.depth {
                    perft(&self.pos, d);
                }
            }
            GoKind::SplitPerft => {
                if let Some(d) = settings.depth {
                    split(&self.pos, d);
                }
            }
        }
    }

    fn stop(&mut self) -> () {
        if self.debug.value {
            println!("info string stop");
        }
    }

    fn print(&self) -> () {
        print!("{}", self.pos);
    }

    fn print_options(&self) -> () {
        println!("{}", self.debug);
        println!("{}", self.search);
    }

    fn set_option(&mut self, name: &str, value: &str) -> () {
        if self.debug.value {
            println!("info string set option '{}' to '{}'", name, value);
        }

        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", name) => {
                for value in &self.search.options {
                    if value == name {
                        self.search.value = value.to_string();
                        break;
                    }
                }
            }
            (_, _) => {}
        }
    }

    // Queries
    fn query_p1turn(&self) -> () {
        match self.pos.turn {
            Side::White => println!("response true"),
            Side::Black => println!("response false"),
        }
    }

    fn query_gameover(&self) -> () {
        match self.pos.is_gameover() {
            true => println!("response true"),
            false => println!("response false"),
        }
    }

    fn query_result(&self) -> () {
        match self.pos.result() {
            Some(GameResult::WhiteWin) => println!("response p1win"),
            Some(GameResult::BlackWin) => println!("response p2win"),
            Some(GameResult::Draw) => println!("response draw"),
            None => println!("response none"),
        }
    }
}
