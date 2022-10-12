mod board;
mod canvas;

use board::{Board, State};
use canvas::{Canvas, Color};

fn main() -> Result<(), std::io::Error> {
    let mut board = Board::new(10, 10);
    board.set_cell(1, 0, State::Alive);
    board.set_cell(2, 1, State::Alive);
    board.set_cell(0, 2, State::Alive);
    board.set_cell(1, 2, State::Alive);
    board.set_cell(2, 2, State::Alive);

    let mut canvas = board.render();
    canvas.save_pbm("glider/000.pbm")?;
    for i in 1..40 {
        board = board.next_generation();
        canvas = board.render();
        canvas.save_pbm(&format!("glider/{:03}.pbm", i))?;
    }

    Ok(())
}
