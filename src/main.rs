mod canvas;

use canvas::{Canvas, Color};

fn main() {
    let mut canvas = Canvas::new(800, 600);
    canvas.rect(150, 100, 300, 500, Color::Black);
    canvas.print_pbm();
}
