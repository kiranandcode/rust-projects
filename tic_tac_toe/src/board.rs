use std::collections::hash_map::HashMap;
use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Debug,Hash,Copy,Clone,PartialEq)]
pub enum TicTacToeCell {
   X,
   O,
   Empty
}

#[derive(Debug,Hash,Copy,Clone)]
pub struct TicTacToeBoard {
    c1 : TicTacToeCell,
    c2 : TicTacToeCell,
    c3 : TicTacToeCell,

    c4 : TicTacToeCell,
    c5 : TicTacToeCell,
    c6 : TicTacToeCell,

    c7 : TicTacToeCell,
    c8 : TicTacToeCell,
    c9 : TicTacToeCell,
}

impl TicTacToeBoard {
    pub fn new() -> Self {
        TicTacToeBoard {
            c1: TicTacToeCell::Empty,
            c2: TicTacToeCell::Empty,
            c3: TicTacToeCell::Empty,

            c4: TicTacToeCell::Empty,
            c5: TicTacToeCell::Empty,
            c6: TicTacToeCell::Empty,

            c7: TicTacToeCell::Empty,
            c8: TicTacToeCell::Empty,
            c9: TicTacToeCell::Empty,
        }
    }

    pub fn has_anyone_won(&self) -> TicTacToeCell {
        // test rows
        if      self[0] == self[1] &&  self[1] == self[2] && self[0] != TicTacToeCell::Empty {
            self[0]
        } 
        else if self[3] == self[4] &&  self[4] == self[5] && self[3] != TicTacToeCell::Empty  {
            self[3]
        } 
        else if self[6] == self[7] &&  self[7] == self[8] && self[6] != TicTacToeCell::Empty  {
            self[6]
        }

        // test columns
        else if self[0] == self[3] &&  self[3] == self[6] && self[0] != TicTacToeCell::Empty {
            self[0]
        } 
        else if self[1] == self[4] &&  self[4] == self[7] && self[1] != TicTacToeCell::Empty  {
            self[1]

        } 
        else if self[2] == self[5] &&  self[5] == self[8] && self[2] != TicTacToeCell::Empty  {
            self[2]
        }

        // test diagonals

        else if self[0] == self[4] &&  self[4] == self[8] && self[0] != TicTacToeCell::Empty {
            self[0]
        } 
        else if self[2] == self[4] &&  self[4] == self[6] && self[2] != TicTacToeCell::Empty {
            self[2]
        }  else {
            TicTacToeCell::Empty
        }
 

    }
}


impl Index<usize> for TicTacToeBoard {
    type Output = TicTacToeCell;

    fn index(&self, index : usize) -> &Self::Output {
        assert!(index >= 0 && index < 9);
        match index {
            0 => &self.c1,
            1 => &self.c2,
            2 => &self.c3,
            3 => &self.c4,
            4 => &self.c5,
            5 => &self.c6,
            6 => &self.c7,
            7 => &self.c8,
            8 => &self.c9,
            _ => panic!("Cell index out of bounds")
 
        }
    }

}


impl IndexMut<usize> for TicTacToeBoard {

    fn index_mut(&mut self, index : usize) -> &mut TicTacToeCell {
        match index {
            0 => &mut self.c1,
            1 => &mut self.c2,
            2 => &mut self.c3,
            3 => &mut self.c4,
            4 => &mut self.c5,
            5 => &mut self.c6,
            6 => &mut self.c7,
            7 => &mut self.c8,
            8 => &mut self.c9,
            _ => panic!("Cell index out of bounds")
 
        }
 
    }
}

impl fmt::Display for TicTacToeCell {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match *self {
                TicTacToeCell::X =>  write!(f, "{}", "X"),
                TicTacToeCell::O =>  write!(f, "{}", "O"),
                TicTacToeCell::Empty =>  write!(f, "{}", "_"),
           }
     }
}


impl fmt::Display for TicTacToeBoard {

     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "[");
            for i in 0..9 {
                 write!(f, "{}", self[i]);
                 if i < 8 {
                     if i == 2 || i == 5 {
                         write!(f, "; ");
                     } else  {
                         write!(f, ", ");
                     }
                 }
            }
         write!(f, "]")
     }

}


