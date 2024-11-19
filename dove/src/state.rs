use pijersi::position::Position;
use ugi::options::options;

pub struct EngineState {
    pub pos: Position,
    // Options
    pub debug: options::Check,
    pub search: options::Combo,
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            debug: options::Check {
                name: "debug".to_string(),
                value: cfg!(debug_assertions),
            },
            search: options::Combo {
                name: "search".to_string(),
                value: "alphabeta".to_string(),
                options: vec![
                    "random".to_string(),
                    "negamax".to_string(),
                    "alphabeta".to_string(),
                ],
            },
        }
    }
}
