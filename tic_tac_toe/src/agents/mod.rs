pub mod optimal_agent;
use board::TicTacToeBoard;


pub trait Agent {
    fn get_next_move(&mut self, board: &TicTacToeBoard) -> usize; 
}
