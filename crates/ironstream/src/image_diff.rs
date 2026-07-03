// FILE: image_diff.rs
// occt: Image_Diff, Image_PixMap, Image_Format, Image_AlienPixMap

/// Pixel format for image buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Gray8,
    Rgb8,
    Rgba8,
    Rgb32f,
    Rgba32f,
    Bgr8,
    Bgra8,
}

impl ImageFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ImageFormat::Gray8 => 1,
            ImageFormat::Rgb8 | ImageFormat::Bgr8 => 3,
            ImageFormat::Rgba8 | ImageFormat::Bgra8 => 4,
            ImageFormat::Rgb32f => 12,
            ImageFormat::Rgba32f => 16,
        }
    }

    pub fn nb_channels(&self) -> usize {
        match self {
            ImageFormat::Gray8 => 1,
            ImageFormat::Rgb8 | ImageFormat::Bgr8 | ImageFormat::Rgb32f => 3,
            ImageFormat::Rgba8 | ImageFormat::Bgra8 | ImageFormat::Rgba32f => 4,
        }
    }

    pub fn has_alpha(&self) -> bool {
        matches!(self, ImageFormat::Rgba8 | ImageFormat::Bgra8 | ImageFormat::Rgba32f)
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, ImageFormat::Rgb32f | ImageFormat::Rgba32f)
    }
}

// occt: Image_PixMap
#[derive(Clone, Debug)]
pub struct ImagePixMap {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
    pub top_down: bool,
}

impl ImagePixMap {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        let stride = width as usize * format.bytes_per_pixel();
        let data = vec![0u8; stride * height as usize];
        Self { width, height, format, data, top_down: true }
    }

    pub fn is_empty(&self) -> bool { self.width == 0 || self.height == 0 }

    pub fn stride(&self) -> usize { self.width as usize * self.format.bytes_per_pixel() }

    pub fn size_bytes(&self) -> usize { self.data.len() }

    pub fn pixel_offset(&self, x: u32, y: u32) -> usize {
        let row = if self.top_down { y } else { self.height.saturating_sub(1) - y };
        (row as usize * self.stride()) + x as usize * self.format.bytes_per_pixel()
    }

    pub fn pixel_rgba8(&self, x: u32, y: u32) -> [u8; 4] {
        if x >= self.width || y >= self.height { return [0; 4]; }
        let off = self.pixel_offset(x, y);
        match self.format {
            ImageFormat::Rgba8 | ImageFormat::Bgra8 => {
                [self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]]
            }
            ImageFormat::Rgb8 | ImageFormat::Bgr8 => {
                [self.data[off], self.data[off+1], self.data[off+2], 255]
            }
            ImageFormat::Gray8 => {
                [self.data[off]; 4]
            }
            _ => [0; 4],
        }
    }

    pub fn set_pixel_rgba8(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= self.width || y >= self.height { return; }
        let off = self.pixel_offset(x, y);
        match self.format {
            ImageFormat::Rgba8 => {
                self.data[off..off+4].copy_from_slice(&rgba);
            }
            ImageFormat::Rgb8 => {
                self.data[off..off+3].copy_from_slice(&rgba[..3]);
            }
            ImageFormat::Gray8 => {
                let g = (0.299 * rgba[0] as f32 + 0.587 * rgba[1] as f32 + 0.114 * rgba[2] as f32) as u8;
                self.data[off] = g;
            }
            _ => {}
        }
    }

    pub fn fill(&mut self, rgba: [u8; 4]) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel_rgba8(x, y, rgba);
            }
        }
    }

    pub fn nb_pixels(&self) -> u64 { self.width as u64 * self.height as u64 }
}

/// Result of an image comparison.
#[derive(Clone, Debug)]
pub struct ImageDiffResult {
    pub nb_different_pixels: u64,
    pub max_channel_diff: u8,
    pub rms_error: f64,
    pub is_same: bool,
    pub tolerance: u8,
}

impl ImageDiffResult {
    pub fn same() -> Self {
        Self { nb_different_pixels: 0, max_channel_diff: 0, rms_error: 0.0, is_same: true, tolerance: 0 }
    }

    pub fn different_ratio(&self, total: u64) -> f64 {
        if total == 0 { 0.0 } else { self.nb_different_pixels as f64 / total as f64 }
    }
}

// occt: Image_Diff
#[derive(Clone, Debug)]
pub struct ImageDiff {
    pub tolerance: u8,
    pub scale_factor: u32,    // downscale for faster comparison
}

impl ImageDiff {
    pub fn new() -> Self { Self { tolerance: 0, scale_factor: 1 } }

    pub fn with_tolerance(mut self, t: u8) -> Self { self.tolerance = t; self }
    pub fn with_scale(mut self, s: u32) -> Self { self.scale_factor = s.max(1); self }

    pub fn compare(&self, a: &ImagePixMap, b: &ImagePixMap) -> ImageDiffResult {
        if a.width != b.width || a.height != b.height {
            return ImageDiffResult {
                nb_different_pixels: a.nb_pixels().max(b.nb_pixels()),
                max_channel_diff: 255,
                rms_error: 255.0,
                is_same: false,
                tolerance: self.tolerance,
            };
        }
        let mut diff_count = 0u64;
        let mut max_diff = 0u8;
        let mut sq_sum = 0.0f64;
        let total = a.nb_pixels();

        for y in 0..a.height {
            for x in 0..a.width {
                let pa = a.pixel_rgba8(x, y);
                let pb = b.pixel_rgba8(x, y);
                let mut max_ch = 0u8;
                for c in 0..3 {
                    let d = (pa[c] as i16 - pb[c] as i16).unsigned_abs() as u8;
                    max_ch = max_ch.max(d);
                    sq_sum += (d as f64) * (d as f64);
                }
                if max_ch > self.tolerance {
                    diff_count += 1;
                    max_diff = max_diff.max(max_ch);
                }
            }
        }

        let rms = (sq_sum / (total.max(1) as f64 * 3.0)).sqrt();
        let is_same = diff_count == 0;

        ImageDiffResult {
            nb_different_pixels: diff_count,
            max_channel_diff: max_diff,
            rms_error: rms,
            is_same,
            tolerance: self.tolerance,
        }
    }

    pub fn highlight_diff(&self, a: &ImagePixMap, b: &ImagePixMap) -> ImagePixMap {
        let w = a.width.min(b.width);
        let h = a.height.min(b.height);
        let mut out = ImagePixMap::new(w, h, ImageFormat::Rgba8);
        for y in 0..h {
            for x in 0..w {
                let pa = a.pixel_rgba8(x, y);
                let pb = b.pixel_rgba8(x, y);
                let diff: bool = (0..3).any(|c| (pa[c] as i16 - pb[c] as i16).unsigned_abs() as u8 > self.tolerance);
                let px = if diff { [255, 0, 0, 255] } else { [pa[0]/2, pa[1]/2, pa[2]/2, 255] };
                out.set_pixel_rgba8(x, y, px);
            }
        }
        out
    }
}

impl Default for ImageDiff {
    fn default() -> Self { Self::new() }
}

// occt: Image_AlienPixMap (supports loading/saving metadata)
#[derive(Clone, Debug)]
pub struct AlienPixMap {
    pub pixmap: ImagePixMap,
    pub file_path: String,
    pub dpi: f32,
    pub gamma: f32,
}

impl AlienPixMap {
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        Self {
            pixmap: ImagePixMap::new(width, height, format),
            file_path: String::new(),
            dpi: 96.0,
            gamma: 2.2,
        }
    }

    pub fn width(&self) -> u32 { self.pixmap.width }
    pub fn height(&self) -> u32 { self.pixmap.height }
    pub fn format(&self) -> ImageFormat { self.pixmap.format }
    pub fn is_empty(&self) -> bool { self.pixmap.is_empty() }

    pub fn set_file_path(&mut self, path: impl Into<String>) { self.file_path = path.into(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_format_info() {
        assert_eq!(ImageFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(ImageFormat::Rgb32f.bytes_per_pixel(), 12);
        assert!(ImageFormat::Rgba8.has_alpha());
        assert!(!ImageFormat::Rgb8.has_alpha());
        assert!(ImageFormat::Rgba32f.is_floating_point());
    }

    #[test]
    fn pixmap_fill_pixel() {
        let mut img = ImagePixMap::new(4, 4, ImageFormat::Rgba8);
        img.fill([255, 0, 0, 255]);
        let px = img.pixel_rgba8(2, 2);
        assert_eq!(px, [255, 0, 0, 255]);
    }

    #[test]
    fn image_diff_identical() {
        let mut a = ImagePixMap::new(4, 4, ImageFormat::Rgba8);
        a.fill([100, 150, 200, 255]);
        let b = a.clone();
        let diff = ImageDiff::new().compare(&a, &b);
        assert!(diff.is_same);
        assert_eq!(diff.nb_different_pixels, 0);
    }

    #[test]
    fn image_diff_different() {
        let mut a = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
        a.fill([0, 0, 0, 255]);
        let mut b = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
        b.fill([255, 255, 255, 255]);
        let diff = ImageDiff::new().compare(&a, &b);
        assert!(!diff.is_same);
        assert_eq!(diff.nb_different_pixels, 4);
    }

    #[test]
    fn image_diff_with_tolerance() {
        let mut a = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
        a.fill([100, 100, 100, 255]);
        let mut b = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
        b.fill([105, 100, 100, 255]);
        let diff = ImageDiff::new().with_tolerance(10).compare(&a, &b);
        assert!(diff.is_same);
    }

    #[test]
    fn pixmap_size_mismatch() {
        let a = ImagePixMap::new(4, 4, ImageFormat::Rgb8);
        let b = ImagePixMap::new(8, 8, ImageFormat::Rgb8);
        let diff = ImageDiff::new().compare(&a, &b);
        assert!(!diff.is_same);
    }
}
