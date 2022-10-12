use crate::canvas::{Canvas, Color};

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

pub struct Board {
    width: i32,
    height: i32,
    cells: Vec<State>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Board {
            width: width,
            height: height,
            cells: vec![State::Dead; (width * height) as usize],
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> State {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            State::Dead
        } else {
            self.cells[(y * self.width + x) as usize]
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, state: State) {
        self.cells[(y * self.width + x) as usize] = state;
    }

    pub fn print_board(&self) {
        for j in 0..self.height {
            for i in 0..self.width {
                if self.get_cell(i, j) == State::Dead {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }

    pub fn count_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                if self.get_cell(i, j) == State::Alive {
                    count = count + 1;
                }
            }
        }
        if self.get_cell(x, y) == State::Alive {
            count = count - 1;
        }
        count
    }

    pub fn next_generation(&self) -> Board {
        let mut board = Board::new(self.width, self.height);
        for j in 0..self.height {
            for i in 0..self.width {
                let alive_neighbours = self.count_neighbors(i, j);
                if alive_neighbours == 3 && self.get_cell(i, j) == State::Dead {
                    board.set_cell(i, j, State::Alive);
                } else if (alive_neighbours == 2 || alive_neighbours == 3)
                    && self.get_cell(i, j) == State::Alive
                {
                    board.set_cell(i, j, State::Alive);
                } else {
                    board.set_cell(i, j, State::Dead);
                }
            }
        }
        board
    }

    pub fn render(&self) -> Canvas {
        let mut c = Canvas::new(self.width * 10, self.height * 10);

        for j in 0..self.height {
            for i in 0..self.width {
                if self.get_cell(i, j) == State::Alive {
                    c.rect(i * 10, j * 10, 10, 10, Color::Black);
                }
            }
        }

        c
    }
}
