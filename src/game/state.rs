

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Loss,
    RequestedRestart,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Over(GameResult),
}