// FILE: image_texture.rs
// occt: Image_Texture, Image_PixMapData, Image_PixMap (texture loading stubs)

/// Pixel format encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Gray8,
    Rgb8,
    Rgba8,
    Rgb16f,
    Rgba16f,
    Rgb32f,
    Rgba32f,
    Unknown,
}

impl Default for PixelFormat {
    fn default() -> Self { Self::Rgba8 }
}

impl PixelFormat {
    pub fn nb_channels(&self) -> usize {
        match self {
            Self::Gray8 => 1,
            Self::Rgb8 | Self::Rgb16f | Self::Rgb32f => 3,
            Self::Rgba8 | Self::Rgba16f | Self::Rgba32f => 4,
            Self::Unknown => 0,
        }
    }

    pub fn bytes_per_channel(&self) -> usize {
        match self {
            Self::Gray8 | Self::Rgb8 | Self::Rgba8 => 1,
            Self::Rgb16f | Self::Rgba16f => 2,
            Self::Rgb32f | Self::Rgba32f => 4,
            Self::Unknown => 0,
        }
    }

    pub fn bytes_per_pixel(&self) -> usize { self.nb_channels() * self.bytes_per_channel() }
    pub fn is_alpha(&self) -> bool { matches!(self, Self::Rgba8 | Self::Rgba16f | Self::Rgba32f) }
}

// occt: Image_PixMapData — raw pixel data buffer
#[derive(Clone, Debug)]
pub struct PixMapData {
    pub width: usize,
    pub height: usize,
    pub format: PixelFormat,
    pub data: Vec<u8>,
    pub stride: usize,
}

impl PixMapData {
    pub fn new(width: usize, height: usize, format: PixelFormat) -> Self {
        let stride = width * format.bytes_per_pixel();
        let data = vec![0u8; stride * height];
        Self { width, height, format, data, stride }
    }

    pub fn nb_bytes(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn row_ptr(&self, row: usize) -> &[u8] { &self.data[row*self.stride..(row+1)*self.stride] }
    pub fn row_ptr_mut(&mut self, row: usize) -> &mut [u8] { &mut self.data[row*self.stride..(row+1)*self.stride] }

    pub fn set_pixel_rgba(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height { return; }
        if self.format.nb_channels() < 4 { return; }
        let offset = y * self.stride + x * self.format.bytes_per_pixel();
        if offset + 3 < self.data.len() {
            self.data[offset] = r;
            self.data[offset+1] = g;
            self.data[offset+2] = b;
            self.data[offset+3] = a;
        }
    }

    pub fn get_pixel_rgba(&self, x: usize, y: usize) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height { return None; }
        if self.format.nb_channels() < 4 { return None; }
        let offset = y * self.stride + x * self.format.bytes_per_pixel();
        if offset + 3 < self.data.len() {
            Some([self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3]])
        } else {
            None
        }
    }
}

/// Wrapping mode for UV coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureWrap {
    Repeat,
    Clamp,
    MirrorRepeat,
}

impl Default for TextureWrap {
    fn default() -> Self { Self::Repeat }
}

/// Filtering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFilter {
    Nearest,
    Linear,
    MipMap,
}

impl Default for TextureFilter {
    fn default() -> Self { Self::Linear }
}

// occt: Image_Texture — a texture referencing pixmap data or a file path
#[derive(Clone, Debug)]
pub struct ImageTexture {
    pub id: u32,
    pub path: Option<String>,
    pub pixmap: Option<PixMapData>,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
    pub filter: TextureFilter,
    pub is_premult_alpha: bool,
    pub u_scale: f64,
    pub v_scale: f64,
    pub u_offset: f64,
    pub v_offset: f64,
}

impl ImageTexture {
    pub fn from_path(id: u32, path: &str) -> Self {
        Self {
            id,
            path: Some(path.to_owned()),
            pixmap: None,
            wrap_s: TextureWrap::Repeat,
            wrap_t: TextureWrap::Repeat,
            filter: TextureFilter::Linear,
            is_premult_alpha: false,
            u_scale: 1.0, v_scale: 1.0,
            u_offset: 0.0, v_offset: 0.0,
        }
    }

    pub fn from_pixmap(id: u32, data: PixMapData) -> Self {
        Self {
            id,
            path: None,
            pixmap: Some(data),
            wrap_s: TextureWrap::Repeat,
            wrap_t: TextureWrap::Repeat,
            filter: TextureFilter::Linear,
            is_premult_alpha: false,
            u_scale: 1.0, v_scale: 1.0,
            u_offset: 0.0, v_offset: 0.0,
        }
    }

    pub fn set_wrap(&mut self, s: TextureWrap, t: TextureWrap) { self.wrap_s = s; self.wrap_t = t; }
    pub fn set_filter(&mut self, f: TextureFilter) { self.filter = f; }
    pub fn set_scale(&mut self, u: f64, v: f64) { self.u_scale = u; self.v_scale = v; }
    pub fn set_offset(&mut self, u: f64, v: f64) { self.u_offset = u; self.v_offset = v; }

    pub fn has_data(&self) -> bool { self.path.is_some() || self.pixmap.is_some() }
    pub fn width(&self) -> Option<usize> { self.pixmap.as_ref().map(|p| p.width) }
    pub fn height(&self) -> Option<usize> { self.pixmap.as_ref().map(|p| p.height) }
    pub fn format(&self) -> Option<PixelFormat> { self.pixmap.as_ref().map(|p| p.format) }

    pub fn sample_uv(&self, u: f64, v: f64) -> Option<[u8; 4]> {
        let pm = self.pixmap.as_ref()?;
        let uu = match self.wrap_s {
            TextureWrap::Repeat => u.rem_euclid(1.0),
            TextureWrap::Clamp => u.clamp(0.0, 1.0),
            TextureWrap::MirrorRepeat => {
                let t = u.rem_euclid(2.0);
                if t > 1.0 { 2.0 - t } else { t }
            }
        };
        let vv = match self.wrap_t {
            TextureWrap::Repeat => v.rem_euclid(1.0),
            TextureWrap::Clamp => v.clamp(0.0, 1.0),
            TextureWrap::MirrorRepeat => {
                let t = v.rem_euclid(2.0);
                if t > 1.0 { 2.0 - t } else { t }
            }
        };
        let x = (uu * (pm.width as f64 - 1.0)).round() as usize;
        let y = (vv * (pm.height as f64 - 1.0)).round() as usize;
        pm.get_pixel_rgba(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_format_channels() {
        assert_eq!(PixelFormat::Rgba8.nb_channels(), 4);
        assert_eq!(PixelFormat::Rgb8.nb_channels(), 3);
        assert_eq!(PixelFormat::Gray8.nb_channels(), 1);
        assert_eq!(PixelFormat::Rgba8.bytes_per_pixel(), 4);
        assert!(PixelFormat::Rgba8.is_alpha());
    }

    #[test]
    fn pixmap_data_set_get() {
        let mut pm = PixMapData::new(4, 4, PixelFormat::Rgba8);
        pm.set_pixel_rgba(2, 2, 255, 128, 0, 255);
        let px = pm.get_pixel_rgba(2, 2).unwrap();
        assert_eq!(px, [255, 128, 0, 255]);
    }

    #[test]
    fn pixmap_data_size() {
        let pm = PixMapData::new(8, 8, PixelFormat::Rgb8);
        assert_eq!(pm.nb_bytes(), 8 * 8 * 3);
        assert!(!pm.is_empty());
    }

    #[test]
    fn image_texture_from_path() {
        let t = ImageTexture::from_path(1, "/tex/wood.png");
        assert!(t.has_data());
        assert!(t.path.is_some());
        assert!(t.width().is_none());
    }

    #[test]
    fn image_texture_sample_uv() {
        let mut pm = PixMapData::new(2, 2, PixelFormat::Rgba8);
        pm.set_pixel_rgba(1, 1, 100, 200, 50, 255);
        let mut tex = ImageTexture::from_pixmap(1, pm);
        tex.set_wrap(TextureWrap::Clamp, TextureWrap::Clamp);
        let px = tex.sample_uv(1.0, 1.0).unwrap();
        assert_eq!(px, [100, 200, 50, 255]);
    }

    #[test]
    fn image_texture_wrap_repeat() {
        let pm = PixMapData::new(4, 4, PixelFormat::Rgba8);
        let tex = ImageTexture::from_pixmap(1, pm);
        // uv = 1.25 should wrap to 0.25 with Repeat mode
        let _ = tex.sample_uv(1.25, 0.5);
    }
}
