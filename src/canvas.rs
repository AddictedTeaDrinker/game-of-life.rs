pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<bool>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            pixels: vec![false; (width * height) as usize],
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> bool {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: bool) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn print_pbm(&self) {
        println!("P1\n{} {}", self.width, self.height);
        for pixel in self.pixels.iter() {
            print!("{} ", *pixel as usize);
        }
        println!("");
    }
}
