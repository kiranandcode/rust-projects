use board::{TicTacToeCell, TicTacToeBoard};

struct OptimalAgent(TicTacToeCell); 

impl OptimalAgent {
    pub fn new(player : TicTacToeCell) -> Self {
        assert!(player != TicTacToeCell::Empty, "Agent can not use an empty space as its piece");
        OptimalAgent(player)
    }
}

impl super::Agent for OptimalAgent {
    fn get_next_move(&mut self, board: &TicTacToeBoard) -> usize {
        if board.has_anyone_won() != TicTacToeCell::Empty {
            panic!("Get next move called on Agents::OptimalAgent after game has been won");
        }
        let opponent_piece = if self.0 == TicTacToeCell::X { TicTacToeCell::O } else { TicTacToeCell::X };
            

        // first, if any opponent paths are near done, block them


        0

    }
}

