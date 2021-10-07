pub type GameAction = (usize, usize); // index, value

#[derive(Debug)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

pub trait GameState: std::fmt::Debug + std::clone::Clone {
    fn new() -> Self;
    fn reward(&self) -> Option<f64>;
    fn legal_actions(&self) -> Vec<usize>;
    fn pretty_print(&self);
    fn next_state(&self, action: usize) -> Self;
}
