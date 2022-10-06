#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            pixels: vec![Color::White; (width * height) as usize],
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        for i in x..x + width {
            for j in y..y + height {
                self.set_pixel(i, j, color);
            }
        }
    }

    pub fn print_pbm(&self) {
        println!("P1\n{} {}", self.width, self.height);
        for pixel in self.pixels.iter() {
            if pixel == &Color::Black {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!("");
    }
}
