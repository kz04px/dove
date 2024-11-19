use std::{fmt, iter::Peekable, str::SplitAsciiWhitespace};

#[derive(Default)]
pub enum GoKind {
    #[default]
    Search,
    Perft,
    SplitPerft,
}

#[derive(Default)]
pub struct GoSettings {
    pub kind: GoKind,
    pub p1time: Option<i32>,
    pub p2time: Option<i32>,
    pub p1inc: Option<i32>,
    pub p2inc: Option<i32>,
    pub depth: Option<i32>,
    pub nodes: Option<u64>,
    pub movetime: Option<i32>,
    pub movestogo: Option<i32>,
}

impl GoSettings {
    #[must_use]
    pub fn from_time(p1time: i32, p2time: i32, p1inc: i32, p2inc: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: Some(p1time),
            p2time: Some(p2time),
            p1inc: Some(p1inc),
            p2inc: Some(p2inc),
            depth: None,
            nodes: None,
            movetime: None,
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_depth(d: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: Some(d),
            nodes: None,
            movetime: None,
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_movetime(t: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: None,
            nodes: None,
            movetime: Some(t),
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_nodes(n: u64) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: None,
            nodes: Some(n),
            movetime: None,
            movestogo: None,
        }
    }
}

impl fmt::Display for GoSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            GoKind::Search => write!(f, "search")?,
            GoKind::Perft => write!(f, "perft")?,
            GoKind::SplitPerft => write!(f, "split")?,
        }

        if let Some(n) = self.p1time {
            write!(f, " p1time {}", n)?;
        } else {
            write!(f, " p1time None")?;
        }

        if let Some(n) = self.p2time {
            write!(f, " p2time {}", n)?;
        } else {
            write!(f, " p2time None")?;
        }

        if let Some(n) = self.p1inc {
            write!(f, " p1inc {}", n)?;
        } else {
            write!(f, " p1inc None")?;
        }

        if let Some(n) = self.p2inc {
            write!(f, " p2inc {}", n)?;
        } else {
            write!(f, " p2inc None")?;
        }

        if let Some(n) = self.depth {
            write!(f, " depth {}", n)?;
        } else {
            write!(f, " depth None")?;
        }

        if let Some(n) = self.nodes {
            write!(f, " nodes {}", n)?;
        } else {
            write!(f, " nodes None")?;
        }

        if let Some(n) = self.movetime {
            write!(f, " movetime {}", n)?;
        } else {
            write!(f, " movetime None")?;
        }

        if let Some(n) = self.movestogo {
            write!(f, " movestogo {}", n)?;
        } else {
            write!(f, " movestogo None")?;
        }

        Ok(())
    }
}

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&GoSettings) -> ()) {
    let mut settings = GoSettings::default();

    while stream.peek().is_some() {
        match (stream.next(), stream.peek()) {
            (Some("search"), _) => {
                settings.kind = GoKind::Search;
            }
            (Some("perft"), _) => {
                settings.kind = GoKind::Perft;
            }
            (Some("split"), _) => {
                settings.kind = GoKind::SplitPerft;
            }
            (Some("p1time"), Some(t)) => {
                settings.p1time = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("p2time"), Some(t)) => {
                settings.p2time = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("p1inc"), Some(t)) => {
                settings.p1inc = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("p2inc"), Some(t)) => {
                settings.p2inc = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("depth"), Some(d)) => {
                settings.depth = Some(d.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("nodes"), Some(n)) => {
                settings.nodes = Some(n.parse::<u64>().unwrap());
                stream.next();
            }
            (Some("movetime"), Some(t)) => {
                settings.movetime = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            (Some("movestogo"), Some(t)) => {
                settings.movestogo = Some(t.parse::<i32>().unwrap());
                stream.next();
            }
            _ => {}
        }
    }

    func(&settings);
}
