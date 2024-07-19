pub fn draw_line_bresenham(framebuffer: &mut crate::framebuffer::Framebuffer, x0: isize, y0: isize, x1: isize, y1: isize, color: crate::framebuffer::Color) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let mut x = x0;
    let mut y = y0;
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x >= 0 && y >= 0 && (x as usize) < framebuffer.width && (y as usize) < framebuffer.height {
            framebuffer.set_pixel(x as usize, y as usize, color);
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

