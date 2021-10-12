extern crate raylib;
use raylib::prelude::*;
use raylib::ffi::DrawPixel;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("a square")
        .build();
     
    while !rl.window_should_close() {
       let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        //d.DrawPixel(0, 0, Color::RED);
        let red = raylib::ffi::Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        for n in 0..1000{
            for i in 0..1000 {
                d.draw_pixel(n, i, red);
            }
        }
    }
}