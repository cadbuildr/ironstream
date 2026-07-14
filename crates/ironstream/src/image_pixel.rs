// FILE: image_pixel.rs
// occt-ref: Image_PixMap, Image_Diff, Image_AlienPixMap

/// Pixel format enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Gray,
    GrayF,
    GrayAlpha,
    RGB,
    BGR,
    RGBA,
    BGRA,
    RGBf,
    RGBAf,
}

impl Default for PixelFormat {
    fn default() -> Self { Self::RGBA }
}

impl PixelFormat {
    pub fn nb_components(&self) -> usize {
        match self {
            Self::Gray | Self::GrayF => 1,
            Self::GrayAlpha => 2,
            Self::RGB | Self::BGR | Self::RGBf => 3,
            Self::RGBA | Self::BGRA | Self::RGBAf => 4,
        }
    }

    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::Gray => 1,
            Self::GrayAlpha => 2,
            Self::RGB | Self::BGR => 3,
            Self::RGBA | Self::BGRA => 4,
            Self::GrayF => 4,
            Self::RGBf => 12,
            Self::RGBAf => 16,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::GrayF | Self::RGBf | Self::RGBAf)
    }
}

// occt-ref: Image_PixMap // — pixel map container
#[derive(Clone, Debug)]
pub struct ImagePixMap {
    pub width: usize,
    pub height: usize,
    pub format: PixelFormat,
    pub data: Vec<u8>,
    pub top_down: bool,
}

impl ImagePixMap {
    pub fn new(width: usize, height: usize, format: PixelFormat) -> Self {
        let bpp = format.bytes_per_pixel();
        Self {
            width,
            height,
            format,
            data: vec![0u8; width * height * bpp],
            top_down: true,
        }
    }

    pub fn is_empty(&self) -> bool { self.width == 0 || self.height == 0 }
    pub fn size_bytes(&self) -> usize { self.data.len() }
    pub fn bytes_per_pixel(&self) -> usize { self.format.bytes_per_pixel() }
    pub fn row_bytes(&self) -> usize { self.width * self.bytes_per_pixel() }

    pub fn pixel_offset(&self, col: usize, row: usize) -> Option<usize> {
        if col >= self.width || row >= self.height { return None; }
        let r = if self.top_down { row } else { self.height - 1 - row };
        Some((r * self.width + col) * self.bytes_per_pixel())
    }

    pub fn set_pixel_rgba(&mut self, col: usize, row: usize, r: u8, g: u8, b: u8, a: u8) -> bool {
        match (self.format, self.pixel_offset(col, row)) {
            (PixelFormat::RGBA, Some(off)) => {
                self.data[off] = r; self.data[off+1] = g;
                self.data[off+2] = b; self.data[off+3] = a;
                true
            }
            (PixelFormat::RGB, Some(off)) => {
                self.data[off] = r; self.data[off+1] = g; self.data[off+2] = b;
                true
            }
            (PixelFormat::Gray, Some(off)) => {
                self.data[off] = ((r as u32 + g as u32 + b as u32) / 3) as u8;
                true
            }
            _ => false,
        }
    }

    pub fn get_pixel_rgba(&self, col: usize, row: usize) -> Option<[u8; 4]> {
        let off = self.pixel_offset(col, row)?;
        match self.format {
            PixelFormat::RGBA => Some([self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]]),
            PixelFormat::RGB => Some([self.data[off], self.data[off+1], self.data[off+2], 255]),
            PixelFormat::Gray => Some([self.data[off]; 4]),
            _ => None,
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        let w = self.width;
        let h = self.height;
        for row in 0..h {
            for col in 0..w {
                self.set_pixel_rgba(col, row, r, g, b, a);
            }
        }
    }

    pub fn clear(&mut self) { self.data.iter_mut().for_each(|b| *b = 0); }

    pub fn flip_vertical(&mut self) {
        let row_bytes = self.row_bytes();
        let h = self.height;
        for r in 0..h/2 {
            let r2 = h - 1 - r;
            for c in 0..row_bytes {
                self.data.swap(r * row_bytes + c, r2 * row_bytes + c);
            }
        }
    }
}

/// Comparison result for two pixel maps.
#[derive(Clone, Debug, Default)]
pub struct DiffResult {
    pub nb_different_pixels: usize,
    pub max_channel_diff: u8,
    pub rms_error: f64,
}

impl DiffResult {
    pub fn is_equal(&self) -> bool { self.nb_different_pixels == 0 }
}

// occt-ref: Image_Diff // — pixel-by-pixel image comparison
#[derive(Clone, Debug)]
pub struct ImageDiff {
    pub tolerance: u8,
}

impl Default for ImageDiff {
    fn default() -> Self { Self { tolerance: 0 } }
}

impl ImageDiff {
    pub fn new(tolerance: u8) -> Self { Self { tolerance } }

    pub fn compare(&self, img1: &ImagePixMap, img2: &ImagePixMap) -> Option<DiffResult> {
        if img1.width != img2.width || img1.height != img2.height || img1.format != img2.format {
            return None;
        }
        let mut diff = DiffResult::default();
        let mut sq_sum = 0.0f64;
        let bpp = img1.bytes_per_pixel();
        let n = img1.data.len();
        for i in 0..n {
            let d = (img1.data[i] as i32 - img2.data[i] as i32).unsigned_abs() as u8;
            sq_sum += (d as f64).powi(2);
            if d > self.tolerance {
                if d > diff.max_channel_diff { diff.max_channel_diff = d; }
                if i % bpp == 0 { diff.nb_different_pixels += 1; }
            }
        }
        diff.rms_error = (sq_sum / n as f64).sqrt();
        Some(diff)
    }
}

// occt-ref: Image_AlienPixMap // — pixel map that can load from file (stub)
#[derive(Clone, Debug, Default)]
pub struct ImageAlienPixMap {
    pub pixmap: Option<ImagePixMap>,
    pub file_path: String,
    pub is_loaded: bool,
}

impl ImageAlienPixMap {
    pub fn new() -> Self { Self::default() }

    pub fn load_stub(&mut self, path: &str, width: usize, height: usize, format: PixelFormat) {
        self.file_path = path.to_owned();
        self.pixmap = Some(ImagePixMap::new(width, height, format));
        self.is_loaded = true;
    }

    pub fn is_loaded(&self) -> bool { self.is_loaded }
    pub fn width(&self) -> usize { self.pixmap.as_ref().map(|p| p.width).unwrap_or(0) }
    pub fn height(&self) -> usize { self.pixmap.as_ref().map(|p| p.height).unwrap_or(0) }
    pub fn pixmap(&self) -> Option<&ImagePixMap> { self.pixmap.as_ref() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_format_bpp() {
        assert_eq!(PixelFormat::RGBA.bytes_per_pixel(), 4);
        assert_eq!(PixelFormat::Gray.bytes_per_pixel(), 1);
        assert_eq!(PixelFormat::RGB.bytes_per_pixel(), 3);
        assert_eq!(PixelFormat::RGBf.bytes_per_pixel(), 12);
    }

    #[test]
    fn pixmap_set_get_rgba() {
        let mut p = ImagePixMap::new(4, 4, PixelFormat::RGBA);
        p.set_pixel_rgba(1, 2, 255, 128, 0, 200);
        let px = p.get_pixel_rgba(1, 2).unwrap();
        assert_eq!(px, [255, 128, 0, 200]);
    }

    #[test]
    fn pixmap_fill_and_clear() {
        let mut p = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        p.fill(10, 20, 30, 255);
        assert_eq!(p.get_pixel_rgba(0, 0).unwrap()[0], 10);
        p.clear();
        assert_eq!(p.get_pixel_rgba(0, 0).unwrap()[0], 0);
    }

    #[test]
    fn pixmap_flip_vertical() {
        let mut p = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        p.set_pixel_rgba(0, 0, 1, 0, 0, 255);
        p.set_pixel_rgba(0, 1, 2, 0, 0, 255);
        p.flip_vertical();
        assert_eq!(p.get_pixel_rgba(0, 0).unwrap()[0], 2);
        assert_eq!(p.get_pixel_rgba(0, 1).unwrap()[0], 1);
    }

    #[test]
    fn image_diff_identical() {
        let p1 = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        let p2 = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        let d = ImageDiff::new(0).compare(&p1, &p2).unwrap();
        assert!(d.is_equal());
    }

    #[test]
    fn image_diff_count_pixels() {
        let mut p1 = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        let p2 = ImagePixMap::new(2, 2, PixelFormat::RGBA);
        p1.set_pixel_rgba(0, 0, 50, 0, 0, 255);
        let d = ImageDiff::new(0).compare(&p1, &p2).unwrap();
        assert!(d.nb_different_pixels > 0);
        assert_eq!(d.max_channel_diff, 255); // alpha channel at p2 is 0 vs 255
    }
}
