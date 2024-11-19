use go::GoSettings;
use result::UGIResult;

pub mod go;
pub mod listen;
mod moves;
pub mod options;
mod position;
pub mod result;
mod setoption;

pub trait UGI {
    fn init(&mut self) -> ();

    fn shutdown(&mut self) -> ();

    fn name(&self) -> &str;

    fn author(&self) -> &str;

    fn uginewgame(&mut self) -> ();

    fn result(&mut self) -> Option<UGIResult>;

    fn isready(&mut self) -> ();

    fn position(&mut self, fen: &str) -> ();

    fn moves(&mut self, movestr: &str) -> ();

    fn go(&mut self, settings: &GoSettings) -> ();

    fn stop(&mut self) -> ();

    fn print(&self) -> ();

    fn print_options(&self) -> ();

    fn set_option(&mut self, name: &str, value: &str) -> ();

    // Queries
    fn query_p1turn(&self) -> ();

    fn query_gameover(&self) -> ();

    fn query_result(&self) -> ();
}
