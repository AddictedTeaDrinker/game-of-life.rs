mod canvas;

use canvas::{Canvas, Color};

fn main() {
    let mut canvas = Canvas::new(7, 6);
    canvas.set_pixel(2, 1, Color::Black);
    canvas.set_pixel(4, 1, Color::Black);
    canvas.set_pixel(1, 3, Color::Black);
    canvas.set_pixel(5, 3, Color::Black);
    canvas.set_pixel(2, 4, Color::Black);
    canvas.set_pixel(3, 4, Color::Black);
    canvas.set_pixel(4, 4, Color::Black);
    canvas.print_pbm();
}
