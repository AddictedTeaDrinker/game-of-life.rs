mod board;
mod canvas;

use board::{Board, State};
use canvas::{Canvas, Color};

fn main() {
    let mut board = Board::new(10, 10);
    board.set_cell(1, 0, State::Alive);
    board.set_cell(2, 1, State::Alive);
    board.set_cell(0, 2, State::Alive);
    board.set_cell(1, 2, State::Alive);
    board.set_cell(2, 2, State::Alive);
    board.print_board();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        board = board.next_generation();
        println!("");
        board.print_board();
    }
}
