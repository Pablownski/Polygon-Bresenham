mod framebuffer;
mod line;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::draw_line;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    draw_line(&mut framebuffer, 100, 100, 700, 500);
    draw_line(&mut framebuffer, 700, 100, 100, 500);
    draw_line(&mut framebuffer, 400, 50, 400, 550);
    draw_line(&mut framebuffer, 50, 300, 750, 300);
    framebuffer.point(0, 0);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
