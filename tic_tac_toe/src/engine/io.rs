use board::{TicTacToeCell, TicTacToeBoard};
use agents::Agent;

use std::io;

pub struct IOEngine<T : Agent> {
   opponent : T, 
   board: TicTacToeBoard,
   player_type: TicTacToeCell,
   opponent_type: TicTacToeCell,
}


impl<T : Agent> IOEngine<T> {
    pub fn new(player : TicTacToeCell, agent : T, board: TicTacToeBoard) -> Self {
        let opponent_type = match player {
            TicTacToeCell::X => TicTacToeCell::O,
            TicTacToeCell::O => TicTacToeCell::X,
            TicTacToeCell::Empty => panic!("Player can not be represented by an empty space"),
        };


        IOEngine {
            opponent: agent,
            board: board,
            player_type: player,
            opponent_type: opponent_type,
        }
    }


    pub fn player_turn(&mut self) {
        loop {
           println!("Please select your move.\nWhich row[1-3]:"); 
           let mut row = None;

           while row.is_none() {
                let mut input = String::new();     
                io::stdin()
                    .read_line(&mut input);
                let input = input.trim();
                match input.parse::<usize>() {
                    Ok(i) => {
                        if i > 0 && i < 4 {
                            row = Some(i - 1);
                        } else {
                            println!("Err: i({}) is not within the required range ([1-3])", i);
                        }
                    },
                    Err(..) => println!("Err: {} is not recognized as a valid integer.\nPlease try again:", input),
                }
           }
           let row = row.unwrap();

           println!("Which col[1-3]:");
           let mut col  = None;
        
           while col.is_none() {
                let mut input = String::new();     
                io::stdin()
                    .read_line(&mut input);
                let input = input.trim();
                match input.parse::<usize>() {
                    Ok(i) => {
                        if i > 0 && i < 4 {
                            col = Some(i - 1);
                        } else {
                            println!("Err: i({}) is not within the required range ([1-3])", i);
                        }
                    },
                    Err(..) => println!("Err: {} is not recognized as a valid integer.\nPlease try again:", input),
                }
 
           }
           let col = col.unwrap();

           if self.board[row * 3 + col] == TicTacToeCell::Empty {
                self.board[row * 3 + col] = self.player_type; 
               return;
           } else {
                println!("That cell[{},{}] is not empty", row,col);
           }
        }
    }

    pub fn opponent_turn(&mut self) {
        let pos = self.opponent.get_next_move(&self.board);
        assert!(self.board[pos] == TicTacToeCell::Empty, "Opponent selects non empty cell");
        println!("The opponent selects cell {}", pos);
        self.board[pos] = self.opponent_type;
    }

    pub fn print_state(&self) {
        println!("The board is as follows:");
        for i in 0..3 {
            print!("    ");
            for j in 0..3 {
                print!("{}", self.board[i * 3 + j]);        
                if j != 2 {
                    print!(" ");
                } else {
                    print!("\n");
                }
            }
        }
    }

    pub fn execute(&mut self, opponent_first : bool) {
        if opponent_first {
            self.opponent_turn();
        }
        self.print_state();
        let mut win_status = self.board.has_anyone_won();
        while let Some(TicTacToeCell::Empty) = win_status {
            self.player_turn();

            win_status = self.board.has_anyone_won();
            if let Some(state) = win_status {
                if state != TicTacToeCell::Empty {
                    break;
                }
            } else {
                // None result means draw
                break;
            }

            self.opponent_turn();

            self.print_state();
            win_status = self.board.has_anyone_won();
        }

        if let Some(win_status) = win_status {
            if win_status == self.player_type {
                println!("Well done! You won!");
            } else {
                println!("Poor performance! Opponent won!");
            }
        } else {
                println!("It was a draw!");
        }
   }
}
