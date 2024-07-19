pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0, 0, 0); width * height];
        Self { width, height, pixels }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = Color::new(0, 0, 0);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Color) {
        crate::line_impl::draw_line_bresenham(self, x0, y0, x1, y1, color);
    }

    pub fn draw_polygon(&mut self, points: &[(isize, isize)], color: Color) {
        if points.len() < 2 {
            return;
        }
        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];
            self.draw_line(x0, y0, x1, y1, color);
        }
    }

    pub fn draw_thick_polygon(&mut self, points: &[(isize, isize)], color: Color, thickness: usize) {
        for i in 0..thickness {
            let offset = i as isize;
            let offset_points: Vec<(isize, isize)> = points.iter()
                .map(|&(x, y)| (x + offset, y + offset))
                .collect();
            self.draw_polygon(&offset_points, color);

            let offset_points: Vec<(isize, isize)> = points.iter()
                .map(|&(x, y)| (x - offset, y - offset))
                .collect();
            self.draw_polygon(&offset_points, color);
        }
    }

    pub fn fill_polygon(&mut self, points: &[(isize, isize)], color: Color) {
        if points.len() < 3 {
            return;
        }

        let min_y = points.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..points.len() {
                let (x0, y0) = points[i];
                let (x1, y1) = points[(i + 1) % points.len()];

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                    intersections.push(x);
                }
            }

            intersections.sort();
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x_start = intersections[i];
                    let x_end = intersections[i + 1];
                    for x in x_start..=x_end {
                        self.set_pixel(x as usize, y as usize, color);
                    }
                }
            }
        }
    }

    pub fn save_as_bmp(&self, filename: &str) -> std::io::Result<()> {
        crate::bmp::save_bmp(filename, self.width, self.height, &self.pixels)
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

