//! 216-color web-safe ordered dithering (Bayer 4x4 matrix).
//!
//! Provides an authentic 1996 Netscape 3.x / Mosaic web-safe color quantization
//! when the optional retro dither mode is enabled. Quantizes 24-bit RGB images
//! to the classic 6x6x6 color cube (values 0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF).

/// Standard 4x4 Bayer threshold matrix normalized from 0 to 15.
const BAYER_4X4: [[i32; 4]; 4] = [
    [0, 8, 2, 10],
    [12, 4, 14, 6],
    [3, 11, 1, 9],
    [15, 7, 13, 5],
];

/// Quantizes an 8-bit color component to the nearest 6-level web-safe step (0, 51, 102, 153, 204, 255).
#[inline]
fn quantize_channel(val: i32) -> u8 {
    let clamped = val.clamp(0, 255);
    // Nearest multiple of 51
    let step = (clamped + 25) / 51;
    (step * 51) as u8
}

/// Applies Bayer 4x4 ordered dithering to an RGBA byte slice in-place.
pub fn dither_rgba(width: usize, height: usize, rgba: &mut [u8]) {
    for y in 0..height {
        let row_offset = y * width * 4;
        let my = y % 4;
        for x in 0..width {
            let offset = row_offset + x * 4;
            let mx = x % 4;

            // Bayer matrix offset centered around 0 in range [-8, 7]
            // Scaling factor tuned so dither spreads nicely over the 51-unit step
            let threshold = BAYER_4X4[my][mx] - 8;
            let dither_offset = threshold * 3; // ~[-24, 21]

            let r = rgba[offset] as i32 + dither_offset;
            let g = rgba[offset + 1] as i32 + dither_offset;
            let b = rgba[offset + 2] as i32 + dither_offset;

            rgba[offset] = quantize_channel(r);
            rgba[offset + 1] = quantize_channel(g);
            rgba[offset + 2] = quantize_channel(b);
            // Alpha left unchanged
        }
    }
}

/// Applies Bayer 4x4 web-safe dithering to an egui::ColorImage in-place.
pub fn dither_color_image(image: &mut egui::ColorImage) {
    let [width, height] = image.size;
    for y in 0..height {
        let my = y % 4;
        let row_offset = y * width;
        for x in 0..width {
            let mx = x % 4;
            let threshold = BAYER_4X4[my][mx] - 8;
            let dither_offset = threshold * 3;
            let p = &mut image.pixels[row_offset + x];
            let r = quantize_channel(p.r() as i32 + dither_offset);
            let g = quantize_channel(p.g() as i32 + dither_offset);
            let b = quantize_channel(p.b() as i32 + dither_offset);
            *p = egui::Color32::from_rgba_premultiplied(r, g, b, p.a());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantize_channel() {
        assert_eq!(quantize_channel(0), 0);
        assert_eq!(quantize_channel(25), 0);
        assert_eq!(quantize_channel(26), 51);
        assert_eq!(quantize_channel(51), 51);
        assert_eq!(quantize_channel(255), 255);
    }

    #[test]
    fn test_dither_rgba() {
        let mut pixels = vec![128, 128, 128, 255, 128, 128, 128, 255, 128, 128, 128, 255, 128, 128, 128, 255];
        dither_rgba(2, 2, &mut pixels);
        for i in 0..4 {
            let r = pixels[i * 4];
            // Must be one of 0, 51, 102, 153, 204, 255
            assert!(r % 51 == 0, "Color component {r} not in web-safe palette");
            assert_eq!(pixels[i * 4 + 3], 255, "Alpha must be preserved");
        }
    }
}
