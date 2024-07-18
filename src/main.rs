extern crate nalgebra_glm as glm;
use std::fs::File;
use std::io::{BufWriter, Write};

mod framebuffer;
mod bmp;
mod line_impl;

use framebuffer::{Framebuffer, Color};

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    let white = Color::new(255, 255, 255);
    let yellow = Color::new(255, 255, 0);

    let polygon_points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), 
        (233, 330), (230, 360), (250, 380), (220, 385), 
        (205, 410), (193, 383)
    ];

    framebuffer.draw_thick_polygon(&polygon_points, white, 3); // grosor de 3 píxeles
    framebuffer.fill_polygon(&polygon_points, yellow);

    framebuffer.save_as_bmp("output.bmp").expect("Failed to save BMP file");
}
