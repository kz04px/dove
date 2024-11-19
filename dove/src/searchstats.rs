use pijersi::mv::Mv;

#[derive(Default)]
pub struct SearchStats {
    pub nodes: u64,
    pub bestmove: Option<Mv>,
}
