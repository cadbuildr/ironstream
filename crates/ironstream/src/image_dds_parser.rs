// FILE: image_dds_parser.rs
// occt: Image_DDSParser

//! DDS file parser.
#[derive(Clone, Debug, Default)]
pub struct DdsParser {
    width: u32,
    height: u32,
    format: u32,
    mip_count: u32,
}

impl DdsParser {
    pub fn new() -> Self { Self::default() }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn format(&self) -> u32 { self.format }
    pub fn mip_count(&self) -> u32 { self.mip_count }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_format(&mut self, fmt: u32) { self.format = fmt; }
    pub fn set_mip_count(&mut self, count: u32) { self.mip_count = count; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = DdsParser::new();
        assert_eq!(p.width(), 0);
    }
}
