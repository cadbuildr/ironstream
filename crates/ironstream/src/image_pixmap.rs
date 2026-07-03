// FILE: image_pixmap.rs
// occt: Image_PixMap, Image_AlienPixMap, Image_SupportedFormats

// occt: Image_Format
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Gray,
    GrayF,
    Rgb,
    RgbF,
    Rgba,
    RgbaF,
    Bgr,
    BgrA,
}

impl ImageFormat {
    pub fn channels(&self) -> usize {
        match self {
            ImageFormat::Gray | ImageFormat::GrayF => 1,
            ImageFormat::Rgb | ImageFormat::RgbF | ImageFormat::Bgr => 3,
            ImageFormat::Rgba | ImageFormat::RgbaF | ImageFormat::BgrA => 4,
        }
    }

    pub fn bytes_per_channel(&self) -> usize {
        match self {
            ImageFormat::GrayF | ImageFormat::RgbF | ImageFormat::RgbaF => 4,
            _ => 1,
        }
    }

    pub fn bytes_per_pixel(&self) -> usize {
        self.channels() * self.bytes_per_channel()
    }
}

// occt: Image_PixMap
/// In-memory pixel map with configurable format.
#[derive(Clone, Debug)]
pub struct PixMap {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

impl PixMap {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        let size = (width as usize) * (height as usize) * format.bytes_per_pixel();
        Self { width, height, format, data: vec![0u8; size] }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    pub fn row_size(&self) -> usize {
        self.width as usize * self.format.bytes_per_pixel()
    }

    pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        Some((y as usize * self.width as usize + x as usize) * self.format.bytes_per_pixel())
    }

    pub fn set_pixel_rgba(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> bool {
        if self.format != ImageFormat::Rgba { return false; }
        if let Some(off) = self.pixel_offset(x, y) {
            self.data[off] = r;
            self.data[off + 1] = g;
            self.data[off + 2] = b;
            self.data[off + 3] = a;
            true
        } else {
            false
        }
    }

    pub fn get_pixel_rgba(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if self.format != ImageFormat::Rgba { return None; }
        let off = self.pixel_offset(x, y)?;
        Some([self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]])
    }

    pub fn fill(&mut self, value: u8) {
        self.data.iter_mut().for_each(|b| *b = value);
    }
}

// occt: Image_AlienPixMap
/// Pixel map that can be loaded from / saved to image files (stub).
#[derive(Clone, Debug)]
pub struct AlienPixMap {
    pub base: PixMap,
    pub file_path: Option<String>,
}

impl AlienPixMap {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        Self { base: PixMap::new(width, height, format), file_path: None }
    }

    pub fn load(&mut self, path: &str) -> bool {
        self.file_path = Some(path.to_string());
        false // stub: actual file I/O not implemented
    }

    pub fn save(&self, _path: &str) -> bool {
        false // stub
    }

    pub fn width(&self) -> u32 { self.base.width }
    pub fn height(&self) -> u32 { self.base.height }
}

// occt: Image_SupportedFormats
/// Reports which image file formats are supported.
#[derive(Clone, Debug, Default)]
pub struct SupportedFormats {
    pub formats: Vec<String>,
}

impl SupportedFormats {
    pub fn new() -> Self {
        Self { formats: vec!["PNG".into(), "BMP".into(), "JPEG".into(), "TGA".into()] }
    }

    pub fn has(&self, fmt: &str) -> bool {
        self.formats.iter().any(|f| f.eq_ignore_ascii_case(fmt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixmap_rgba_round_trip() {
        let mut pm = PixMap::new(4, 4, ImageFormat::Rgba);
        assert!(pm.set_pixel_rgba(1, 2, 255, 128, 0, 255));
        let px = pm.get_pixel_rgba(1, 2).unwrap();
        assert_eq!(px, [255, 128, 0, 255]);
    }

    #[test]
    fn pixmap_size_bytes() {
        let pm = PixMap::new(8, 8, ImageFormat::Rgba);
        assert_eq!(pm.size_bytes(), 8 * 8 * 4);
    }

    #[test]
    fn image_format_channels() {
        assert_eq!(ImageFormat::Gray.channels(), 1);
        assert_eq!(ImageFormat::Rgb.channels(), 3);
        assert_eq!(ImageFormat::Rgba.channels(), 4);
    }

    #[test]
    fn alien_pixmap_load_stub() {
        let mut ap = AlienPixMap::new(0, 0, ImageFormat::Rgb);
        assert!(!ap.load("test.png"));
        assert_eq!(ap.file_path.as_deref(), Some("test.png"));
    }

    #[test]
    fn supported_formats_has() {
        let sf = SupportedFormats::new();
        assert!(sf.has("PNG"));
        assert!(!sf.has("WEBP"));
    }
}
