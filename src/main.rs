mod canvas;

use canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(7, 6);
    canvas.set_pixel(2, 1, true);
    canvas.set_pixel(4, 1, true);
    canvas.set_pixel(1, 3, true);
    canvas.set_pixel(5, 3, true);
    canvas.set_pixel(2, 4, true);
    canvas.set_pixel(3, 4, true);
    canvas.set_pixel(4, 4, true);
    canvas.print_pbm();
}
