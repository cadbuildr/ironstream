// FILE: image_algo.rs
// occt: Image_PixMap, Image_AlienPixMap, Image_PixMapTypedData

/// Pixel format.
/// occt: Image_Format
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Gray8,
    RGB8,
    RGBA8,
    Gray16,
    RGB32F,
    RGBA32F,
    BGR8,
    BGRA8,
}

impl Default for ImageFormat {
    fn default() -> Self { Self::RGBA8 }
}

impl ImageFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::Gray8 => 1,
            Self::Gray16 => 2,
            Self::RGB8 | Self::BGR8 => 3,
            Self::RGBA8 | Self::BGRA8 => 4,
            Self::RGB32F => 12,
            Self::RGBA32F => 16,
        }
    }

    pub fn nb_channels(&self) -> usize {
        match self {
            Self::Gray8 | Self::Gray16 => 1,
            Self::RGB8 | Self::BGR8 | Self::RGB32F => 3,
            Self::RGBA8 | Self::BGRA8 | Self::RGBA32F => 4,
        }
    }
}

/// 2D pixel map (image buffer in RAM).
/// occt: Image_PixMap
#[derive(Clone, Debug)]
pub struct ImagePixMap {
    pub width: usize,
    pub height: usize,
    pub format: ImageFormat,
    pub row_padding: usize,  // extra bytes per row (alignment)
    data: Vec<u8>,
}

impl ImagePixMap {
    pub fn new(width: usize, height: usize, format: ImageFormat) -> Self {
        let row_bytes = width * format.bytes_per_pixel();
        let total = row_bytes * height;
        Self { width, height, format, row_padding: 0, data: vec![0u8; total] }
    }

    pub fn is_empty(&self) -> bool { self.width == 0 || self.height == 0 }
    pub fn size_bytes(&self) -> usize { self.data.len() }
    pub fn bytes_per_row(&self) -> usize { self.width * self.format.bytes_per_pixel() + self.row_padding }

    pub fn pixel_offset(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        Some(y * self.bytes_per_row() + x * self.format.bytes_per_pixel())
    }

    pub fn get_pixel_rgba(&self, x: usize, y: usize) -> Option<[u8; 4]> {
        let off = self.pixel_offset(x, y)?;
        match self.format {
            ImageFormat::RGBA8 | ImageFormat::BGRA8 => {
                if off + 3 < self.data.len() {
                    Some([self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]])
                } else { None }
            }
            ImageFormat::RGB8 | ImageFormat::BGR8 => {
                if off + 2 < self.data.len() {
                    Some([self.data[off], self.data[off+1], self.data[off+2], 255])
                } else { None }
            }
            ImageFormat::Gray8 => {
                if off < self.data.len() { Some([self.data[off]; 4]) } else { None }
            }
            _ => None,
        }
    }

    pub fn set_pixel_rgba(&mut self, x: usize, y: usize, rgba: [u8; 4]) {
        let Some(off) = self.pixel_offset(x, y) else { return; };
        match self.format {
            ImageFormat::RGBA8 | ImageFormat::BGRA8 => {
                if off + 3 < self.data.len() {
                    self.data[off] = rgba[0]; self.data[off+1] = rgba[1];
                    self.data[off+2] = rgba[2]; self.data[off+3] = rgba[3];
                }
            }
            ImageFormat::RGB8 | ImageFormat::BGR8 => {
                if off + 2 < self.data.len() {
                    self.data[off] = rgba[0]; self.data[off+1] = rgba[1]; self.data[off+2] = rgba[2];
                }
            }
            ImageFormat::Gray8 => {
                if off < self.data.len() {
                    self.data[off] = ((rgba[0] as u32 + rgba[1] as u32 + rgba[2] as u32) / 3) as u8;
                }
            }
            _ => {}
        }
    }

    pub fn fill(&mut self, rgba: [u8; 4]) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel_rgba(x, y, rgba);
            }
        }
    }

    pub fn flip_vertical(&mut self) {
        let row = self.bytes_per_row();
        for y in 0..self.height/2 {
            let y2 = self.height - 1 - y;
            let off1 = y * row;
            let off2 = y2 * row;
            for k in 0..row {
                self.data.swap(off1 + k, off2 + k);
            }
        }
    }
}

/// Alien pixmap: a pixmap that can be read from a file path (stub).
/// occt: Image_AlienPixMap
#[derive(Clone, Debug)]
pub struct ImageAlienPixMap {
    pub file_path: String,
    pub pixmap: Option<ImagePixMap>,
    pub is_loaded: bool,
}

impl Default for ImageAlienPixMap {
    fn default() -> Self { Self { file_path: String::new(), pixmap: None, is_loaded: false } }
}

impl ImageAlienPixMap {
    pub fn new() -> Self { Self::default() }

    /// Stub load: create a blank 1×1 RGBA pixmap if path ends with known image extension.
    pub fn load(&mut self, path: &str) -> bool {
        self.file_path = path.to_string();
        let lower = path.to_lowercase();
        let valid = lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg")
            || lower.ends_with(".bmp") || lower.ends_with(".tiff");
        if valid {
            self.pixmap = Some(ImagePixMap::new(1, 1, ImageFormat::RGBA8));
            self.is_loaded = true;
        } else {
            self.is_loaded = false;
        }
        self.is_loaded
    }

    pub fn is_loaded(&self) -> bool { self.is_loaded }
    pub fn width(&self) -> usize { self.pixmap.as_ref().map(|p| p.width).unwrap_or(0) }
    pub fn height(&self) -> usize { self.pixmap.as_ref().map(|p| p.height).unwrap_or(0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixmap_new() {
        let p = ImagePixMap::new(100, 50, ImageFormat::RGBA8);
        assert!(!p.is_empty());
        assert_eq!(p.size_bytes(), 100 * 50 * 4);
    }

    #[test]
    fn pixmap_pixel_set_get() {
        let mut p = ImagePixMap::new(10, 10, ImageFormat::RGBA8);
        p.set_pixel_rgba(3, 4, [255, 128, 0, 200]);
        let px = p.get_pixel_rgba(3, 4).unwrap();
        assert_eq!(px[0], 255);
        assert_eq!(px[1], 128);
        assert_eq!(px[3], 200);
    }

    #[test]
    fn pixmap_out_of_bounds() {
        let p = ImagePixMap::new(10, 10, ImageFormat::RGBA8);
        assert!(p.get_pixel_rgba(10, 0).is_none());
        assert!(p.get_pixel_rgba(0, 10).is_none());
    }

    #[test]
    fn image_format_bytes() {
        assert_eq!(ImageFormat::Gray8.bytes_per_pixel(), 1);
        assert_eq!(ImageFormat::RGB8.bytes_per_pixel(), 3);
        assert_eq!(ImageFormat::RGBA8.bytes_per_pixel(), 4);
        assert_eq!(ImageFormat::RGBA32F.bytes_per_pixel(), 16);
    }

    #[test]
    fn alien_pixmap_load() {
        let mut p = ImageAlienPixMap::new();
        assert!(p.load("photo.png"));
        assert!(p.is_loaded());
        assert!(!p.load("model.stl"));
        assert!(!p.is_loaded());
    }
}
