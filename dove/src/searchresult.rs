use pijersi::mv::Mv;

pub struct SearchResult {
    pub bestmove: Option<Mv>,
    pub ponder: Option<Mv>,
    pub nodes: u64,
}
