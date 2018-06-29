use std::iter::Iterator;
use board::{TicTacToeCell, TicTacToeBoard};

pub struct OptimalAgent(TicTacToeCell); 


#[cfg(test)]
mod Tests {

    use super::*;
    use std::slice::Iter;

    #[test]
    fn get_majority_empty_row() {
        let slice = &[TicTacToeCell::Empty, TicTacToeCell::Empty, TicTacToeCell::Empty];
        let iter = slice.iter();

        assert_eq!(
            None, 
            get_majority(iter)
            );
    }

    #[test]
    fn get_majority_no_contention() {
        let slice = &[TicTacToeCell::X, TicTacToeCell::Empty, TicTacToeCell::Empty];
        let iter = slice.iter();

        assert_eq!(
            Some((TicTacToeCell::X, 1)), 
            get_majority(iter)
            );
    }

    #[test]
    fn get_majority_equal_pairing() {
        let slice = &[TicTacToeCell::X, TicTacToeCell::O, TicTacToeCell::Empty];
        let iter = slice.iter();

        assert_eq!(
            None, 
            get_majority(iter)
            );
    }

    #[test]
    fn get_majority_filled_row() {
        let slice = &[TicTacToeCell::X, TicTacToeCell::O, TicTacToeCell::O];
        let iter = slice.iter();

        assert_eq!(
            None, 
            get_majority(iter)
            );
    }



}

fn get_majority<'a, T : Iterator<Item=&'a TicTacToeCell>>(iter: T) -> Option<(TicTacToeCell, usize)> {
    let mut x = 0; 
    let mut o = 0;

    for cell in iter {
        match cell {
            TicTacToeCell::X => x += 1,
            TicTacToeCell::O => o += 1,
            TicTacToeCell::Empty => (),
        }
    }

    if x + o == 3 {
        None
    } else if x > o {
        Some((TicTacToeCell::X, x))
    } else if o < x {
        Some((TicTacToeCell::O, o))
    } else {
        None
    }
}

fn get_diagonal_majority(slice : &[usize], board: &TicTacToeBoard) -> Option<(TicTacToeCell,u8)> {
    let mut x = 0; 
    let mut o = 0;

    for i in slice {
        match board[*i] {
            TicTacToeCell::X => x += 1,
            TicTacToeCell::O => o += 1,
            TicTacToeCell::Empty => (),
        }
    }

    if x + o == 3 {
        None
    } else if x > o {
        Some((TicTacToeCell::X, x))
    } else if o < x {
        Some((TicTacToeCell::O, o))
    } else {
        None
    }
}

fn get_left_diagonal_majority(board : &TicTacToeBoard) -> Option<(TicTacToeCell,u8)> {
    get_diagonal_majority(&[0,4,8], board)
}

fn get_right_diagonal_majority(board : &TicTacToeBoard) -> Option<(TicTacToeCell,u8)> {
    get_diagonal_majority(&[2,4,6], board)
}




impl OptimalAgent {
    pub fn new(player : TicTacToeCell) -> Self {
        assert!(player != TicTacToeCell::Empty, "Agent can not use an empty space as its piece");
        OptimalAgent(player)
    }



    fn place_in_row(&self, row : usize, board : &TicTacToeBoard) -> usize {
        let mut col = 0;
        for (ind, val) in board.row_iter(row).enumerate() {
            if let TicTacToeCell::Empty = val {
                col = ind;
                break;
            }
        }
        return row * 3 +  col;
    }

    fn place_in_col(&self, col: usize, board: &TicTacToeBoard) -> usize {
        let mut row = 0;
        for (ind, val) in board.col_iter(col).enumerate() {
            if let TicTacToeCell::Empty = val {
                row = ind;
                break;
            }
        }
        return row * 3 +  col;
    }

    fn place_in_diagonal(&self, i: u8, board: &TicTacToeBoard) -> usize {
        match i {
            0 => {
                for i in &[0,4,8]  {
                    if let TicTacToeCell::Empty = board[*i] {
                        return *i;
                    }
                }
                panic!("Sanity failure in agents::optimal_agent::place_in_diagonal");
            }
            1 => {
                for i in &[2,4,6]  {
                    if let TicTacToeCell::Empty = board[*i] {
                        return *i;
                    }
                }

                panic!("Sanity failure in agents::optimal_agent::place_in_diagonal");
            }
            _ => panic!("Invalid input to agents::optimal_agent::place_in_diagonal"),
        }
    }

    fn place_central(&self, board: &TicTacToeBoard) -> usize {
        if let TicTacToeCell::Empty = board[4] {
            4
        } else {
            for i in 0..9 {
                if let TicTacToeCell::Empty = board[i] {
                    return i;
                }
            }
            panic!("Sanity failure in agents::optimal_agent::place_in_diagonal");
        }
    }



}



impl super::Agent for OptimalAgent {
    fn get_next_move(&mut self, board: &TicTacToeBoard) -> usize {
        if board.has_anyone_won() != Some(TicTacToeCell::Empty) {
            panic!("Get next move called on Agents::OptimalAgent after game has been completed");
        }
        let opponent_piece = if self.0 == TicTacToeCell::X { TicTacToeCell::O } else { TicTacToeCell::X };

        // represents     row or col or diag, num, how much
        // for spec - 0 is row, 1 is col, 2 is diag
        // for diag, 0 is \, 1 is /
        let mut max : Option<(i8,usize)> = None;

        for i in 0..3 {
            if let Some((player, count)) = get_majority(board.row_iter(i)) {
                if count >= 2 {
                    return self.place_in_row(i, board);
                }

                if max.is_none() {
                    max = Some((0, i));
                }
            }
        }

        for i in 0..3 {
            if let Some((player, count)) = get_majority(board.col_iter(i)) {
                if count >= 2 {
                    return self.place_in_col(i, board);
                }
                if max.is_none() {
                    max = Some((1, i));
                }

            }
        }

        if let Some((player, count)) = get_left_diagonal_majority(board) {
            if count >= 2 {
                return self.place_in_diagonal(0, board);
            }
            if max.is_none() {
                max = Some((2, 0));
            }
        }

        if let Some((player, count)) = get_left_diagonal_majority(board) {
            if count >= 2 {
                return self.place_in_diagonal(1, board);
            }
            if max.is_none() {
                max = Some((2, 1));
            }
        }



        if let Some((spec, ind)) = max {
            // first, if any opponent paths are near done, block them
            match spec {
                0 => self.place_in_row(ind, board),
                1 => self.place_in_col(ind, board),
                2 => self.place_in_diagonal(ind as u8, board),
                _ => panic!("This path should never be called"),
            }
        } else {
            self.place_central(board)
        }
    }
}

