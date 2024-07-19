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
    let blue = Color::new(0, 0, 255);
    let red = Color::new(255, 0, 0);

    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345),
        (233, 330), (230, 360), (250, 380), (220, 385),
        (205, 410), (193, 383)
    ];

    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    let polygon3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    framebuffer.draw_thick_polygon(&polygon1, white, 3); // Grosor de 3 píxeles
    framebuffer.fill_polygon(&polygon1, yellow);

    framebuffer.draw_thick_polygon(&polygon2, white, 3); // Grosor de 3 píxeles
    framebuffer.fill_polygon(&polygon2, blue);

    framebuffer.draw_thick_polygon(&polygon3, white, 3); // Grosor de 3 píxeles
    framebuffer.fill_polygon(&polygon3, red);

    framebuffer.save_as_bmp("output.bmp").expect("Failed to save BMP file");
}

