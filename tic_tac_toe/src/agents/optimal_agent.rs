use std::iter::Iterator;
use board::{TicTacToeCell, TicTacToeBoard};

struct OptimalAgent(TicTacToeCell); 

fn get_majority<T : Iterator<Item=TicTacToeCell>>(iter: T) -> Option<(TicTacToeCell, usize)> {
           let mut x = 0; 
           let mut o = 0;

           for cell in iter {
                match cell {
                    X => x += 1,
                    O => o += 1,
                    _ => (),
                }
           }
           if x > o {
                Some((TicTacToeCell::X, x))
           } else if o < x {
                Some((TicTacToeCell::O, o))
           } else {
                None
           }
}

fn get_diagonal_majority(slice : &[usize], board: &TicTacToeBoard) -> Option<(TicTacToeCell,usize)> {
    let mut x = 0; 
    let mut o = 0;

    for i in slice {
         match board[i] {
            X => x += 1,
            O => o += 1,
            _ => (),
        }
    }
    if x > o {
        Some((TicTacToeCell::X, x))
    } else if o < x {
        Some((TicTacToeCell::O, o))
    } else {
        None
    }
}

fn get_left_diagonal_majority(board : &TicTacToeBoard) {
    get_diagonal_majority(&[0,4,8], board)
}

fn get_right_diagonal_majority(board : &TicTacToeBoard) {
    get_diagonal_majority(&[2,4,6], board)
}




impl OptimalAgent {
    pub fn new(player : TicTacToeCell) -> Self {
        assert!(player != TicTacToeCell::Empty, "Agent can not use an empty space as its piece");
        OptimalAgent(player)
    }


    
    fn place_in_row(i, board) -> usize {
        0
    }

    fn place_in_col(i, board) -> usize {
        0
    }
}

impl super::Agent for OptimalAgent {
    fn get_next_move(&mut self, board: &TicTacToeBoard) -> usize {
        if board.has_anyone_won() != TicTacToeCell::Empty {
            panic!("Get next move called on Agents::OptimalAgent after game has been won");
        }
        let opponent_piece = if self.0 == TicTacToeCell::X { TicTacToeCell::O } else { TicTacToeCell::X };
            
        for i in 0..3 {
            if let Some((player, count)) = get_majority(board.row_iter(i)) {
                if count >= 2 {
                    return self.place_in_row(i, board);
                }
            }
        }

        for i in 0..3 {
            if let Some((player, count)) = get_majority(board.col_iter(i)) {
                if count >= 2 {
                    return self.place_in_col(i, board);
                }
            }
        }

        


            
        // first, if any opponent paths are near done, block them


        0

    }
}

