mod board;
mod agents;
mod engine;

use board::{TicTacToeBoard, TicTacToeCell};
use engine::IOEngine;
use agents::optimal_agent::OptimalAgent;


fn main() {
    let board = TicTacToeBoard::new();
    let opponent = OptimalAgent::new(TicTacToeCell::X);
    let mut engine = IOEngine::new(TicTacToeCell::O, opponent, board);

    engine.execute(false);
}
