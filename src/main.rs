extern crate raylib;
use raylib::prelude::*;
use raylib::ffi::DrawPixel;
mod scene;
use scene::Circle;
use std::num;


fn main() {
    let WIDTH: i32 = 1000;
    let HEIGHT: i32 = 1000;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("a circle")
        .build();
    //    
    let circ = Circle {
        center_x: 0.5, 
        center_y: 0.5, 
        radius: 250,
    };
    let mut circ_x: f32 = circ.center_x  * WIDTH as f32;
    let mut circ_y: f32 = circ.center_y  * HEIGHT as f32;
    let mut circ_x: i32 = circ_x as i32;
    let mut circ_y: i32 = circ_y as i32;
    //
    let mut drew: i32 = 0;
    while !rl.window_should_close() {
       let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if(drew == 0) {
            for y in 0..WIDTH{
                for x in 0..HEIGHT {
                    let mut distance = getDistance(circ_x, x, circ_y, y);
                    if(distance < circ.radius) {
                        d.draw_pixel(x, y, Color::GREEN);
                    }
                }
            }
        }
    }
    fn getDistance(cx: i32, px: i32, cy: i32, py: i32) -> i32 {
        //println!("pixel x: {}\npixel y: {}", px, py);
        let mut base_x: i32 = (px - cx);
        let mut base_y: i32 = (py - cy);
        let mut base_x: i32 = sqr(base_x);
        let mut base_y: i32 = sqr(base_y);
        // return the square root of the sum
        let mut base_of_root = base_x + base_y;
        return (base_of_root as f64).sqrt() as i32;
    }
    fn sqr(n: i32) -> i32 {
        return n * n;
    }
}