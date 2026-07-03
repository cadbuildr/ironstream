// FILE: image_compressed_pix_map.rs
// occt: Image_CompressedPixMap

//! Compressed pixmap data definition.
#[derive(Clone, Debug)]
pub struct CompressedPixMap {
    base_format: u32,
    compressed_format: u32,
    width: u32,
    height: u32,
    mip_maps: Vec<u32>,
}

impl CompressedPixMap {
    pub fn new() -> Self {
        Self {
            base_format: 0,
            compressed_format: 0,
            width: 0,
            height: 0,
            mip_maps: Vec::new(),
        }
    }

    pub fn base_format(&self) -> u32 { self.base_format }
    pub fn set_base_format(&mut self, format: u32) { self.base_format = format; }
    pub fn compressed_format(&self) -> u32 { self.compressed_format }
    pub fn set_compressed_format(&mut self, format: u32) { self.compressed_format = format; }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn mip_maps(&self) -> &[u32] { &self.mip_maps }
    pub fn set_mip_maps(&mut self, maps: Vec<u32>) { self.mip_maps = maps; }
}

impl Default for CompressedPixMap {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = CompressedPixMap::new();
        assert_eq!(p.width(), 0);
    }

    #[test]
    fn test_setters() {
        let mut p = CompressedPixMap::new();
        p.set_dimensions(512, 256);
        assert_eq!(p.width(), 512);
    }
}
