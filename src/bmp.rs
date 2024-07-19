use std::fs::File;
use std::io::Write;

pub fn save_bmp(filename: &str, width: usize, height: usize, pixels: &[crate::framebuffer::Color]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let file_size = 14u32 + 40 + (width * height * 3) as u32;

    // BMP Header
    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&[0u8; 4])?;
    file.write_all(&(14u32 + 40).to_le_bytes())?;

    // DIB Header
    file.write_all(&[40u8, 0, 0, 0])?;
    file.write_all(&(width as u32).to_le_bytes())?;
    file.write_all(&(height as u32).to_le_bytes())?;
    file.write_all(&[1u8, 0])?;
    file.write_all(&[24u8, 0])?;
    file.write_all(&[0u8; 24])?;

    // Pixel Data
    for y in (0..height) {
        for x in 0..width {
            let color = pixels[y * width + x];
            file.write_all(&[color.b, color.g, color.r])?;
        }
    }

    Ok(())
}
