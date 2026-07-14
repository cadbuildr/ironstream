// FILE: image_proc.rs
// occt-ref: Image_ColorBgr, Image_ColorRgb, Image_ColorBgra, Image_ColorRgba32
//       Image_SupportedFormats, Image_PixMapData

/// Packed RGB color (byte components).
// occt-ref: Image_ColorRgb
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ImageColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn luminance(&self) -> f32 {
        0.299 * self.r as f32 / 255.0
        + 0.587 * self.g as f32 / 255.0
        + 0.114 * self.b as f32 / 255.0
    }
}

/// Packed BGR color (OpenGL-native byte order).
// occt-ref: Image_ColorBgr
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageColorBgr {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl ImageColorBgr {
    pub fn new(b: u8, g: u8, r: u8) -> Self { Self { b, g, r } }

    pub fn from_rgb(c: ImageColorRgb) -> Self { Self { b: c.b, g: c.g, r: c.r } }
    pub fn to_rgb(&self) -> ImageColorRgb { ImageColorRgb { r: self.r, g: self.g, b: self.b } }
}

/// Packed RGBA color (byte components).
/// occt-note: Image_ColorRgba32 / Quantity_ColorRGBA byte encoding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for ImageColorRgba {
    fn default() -> Self { Self { r: 0, g: 0, b: 0, a: 255 } }
}

impl ImageColorRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }

    pub fn opaque(r: u8, g: u8, b: u8) -> Self { Self::new(r, g, b, 255) }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | self.a as u32
    }

    pub fn is_opaque(&self) -> bool { self.a == 255 }

    pub fn to_rgb(&self) -> ImageColorRgb { ImageColorRgb { r: self.r, g: self.g, b: self.b } }

    pub fn alpha_f32(&self) -> f32 { self.a as f32 / 255.0 }
}

/// Packed BGRA color (GPU-native byte order).
// occt-ref: Image_ColorBgra
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageColorBgra {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl ImageColorBgra {
    pub fn new(b: u8, g: u8, r: u8, a: u8) -> Self { Self { b, g, r, a } }

    pub fn from_rgba(c: ImageColorRgba) -> Self { Self { b: c.b, g: c.g, r: c.r, a: c.a } }
    pub fn to_rgba(&self) -> ImageColorRgba { ImageColorRgba { r: self.r, g: self.g, b: self.b, a: self.a } }
}

/// Advertises which pixel formats are supported for load/save.
/// occt: Image_SupportedFormats
#[derive(Clone, Debug, Default)]
pub struct ImageSupportedFormats {
    pub formats: Vec<String>,
}

impl ImageSupportedFormats {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, fmt: &str) { self.formats.push(fmt.to_string()); }

    pub fn supports(&self, fmt: &str) -> bool {
        self.formats.iter().any(|f| f.eq_ignore_ascii_case(fmt))
    }

    pub fn nb_formats(&self) -> usize { self.formats.len() }
}

/// Raw pixel buffer with width, height, and bytes-per-pixel metadata.
// occt-ref: Image_PixMapData
#[derive(Clone, Debug)]
pub struct ImagePixMapData {
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: usize,
    pub data: Vec<u8>,
}

impl ImagePixMapData {
    pub fn new(width: u32, height: u32, bytes_per_pixel: usize) -> Self {
        let size = (width as usize) * (height as usize) * bytes_per_pixel;
        Self { width, height, bytes_per_pixel, data: vec![0u8; size] }
    }

    pub fn row_bytes(&self) -> usize { self.width as usize * self.bytes_per_pixel }

    pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        Some((y as usize * self.width as usize + x as usize) * self.bytes_per_pixel)
    }

    pub fn set_rgba(&mut self, x: u32, y: u32, c: ImageColorRgba) -> bool {
        if self.bytes_per_pixel < 4 { return false; }
        if let Some(off) = self.pixel_offset(x, y) {
            self.data[off] = c.r;
            self.data[off+1] = c.g;
            self.data[off+2] = c.b;
            self.data[off+3] = c.a;
            true
        } else { false }
    }

    pub fn get_rgba(&self, x: u32, y: u32) -> Option<ImageColorRgba> {
        if self.bytes_per_pixel < 4 { return None; }
        let off = self.pixel_offset(x, y)?;
        Some(ImageColorRgba::new(
            self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]))
    }

    pub fn size_bytes(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hex_roundtrip() {
        let c = ImageColorRgb::from_hex(0xFF8000);
        assert_eq!(c.r, 0xFF);
        assert_eq!(c.g, 0x80);
        assert_eq!(c.b, 0x00);
        assert_eq!(c.to_hex(), 0xFF8000);
    }

    #[test]
    fn rgba_hex_roundtrip() {
        let c = ImageColorRgba::from_hex(0x1234_5678);
        assert_eq!(c.r, 0x12);
        assert_eq!(c.a, 0x78);
        assert!(!c.is_opaque());
        assert_eq!(c.to_hex(), 0x1234_5678);
    }

    #[test]
    fn bgr_from_rgb() {
        let rgb = ImageColorRgb::new(10, 20, 30);
        let bgr = ImageColorBgr::from_rgb(rgb);
        assert_eq!(bgr.r, 10);
        assert_eq!(bgr.b, 30);
        assert_eq!(bgr.to_rgb(), rgb);
    }

    #[test]
    fn supported_formats() {
        let mut fmt = ImageSupportedFormats::new();
        fmt.add("PNG");
        fmt.add("JPEG");
        assert!(fmt.supports("png"));
        assert!(fmt.supports("JPEG"));
        assert!(!fmt.supports("BMP"));
    }

    #[test]
    fn pixmap_data_set_get() {
        let mut pm = ImagePixMapData::new(4, 4, 4);
        let c = ImageColorRgba::opaque(255, 128, 0);
        assert!(pm.set_rgba(1, 2, c));
        let g = pm.get_rgba(1, 2).unwrap();
        assert_eq!(g, c);
        assert!(pm.get_rgba(10, 10).is_none());
    }
}
