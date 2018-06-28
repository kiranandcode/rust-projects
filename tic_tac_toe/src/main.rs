mod board;
mod agents;
use board::{TicTacToeBoard, TicTacToeCell};


fn main() {

    let mut board = TicTacToeBoard::new();
    board[1] = TicTacToeCell::X;
    board[3] = TicTacToeCell::X;
    board[8] = TicTacToeCell::X;

    board[2] = TicTacToeCell::O;
    board[6] = TicTacToeCell::O;
    board[4] = TicTacToeCell::O;

    for (index,cell) in board.iter().enumerate() {
        println!("{}: {}", index, cell);
    }

    for (index,cell) in board.iter().enumerate() {
        println!("{}: {}", index, cell);
    }
 
    println!("Iterating over row 0");
    for (index,cell) in board.row_iter(0).enumerate() {
        println!("{}: {}", index, cell);
    }
 
    println!("Iterating over col 1");
    for (index,cell) in board.col_iter(1).enumerate() {
        println!("{}: {}", index, cell);
    }
 

    println!("{}", board);
    println!("Hello, world!");
}
