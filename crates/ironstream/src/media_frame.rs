// FILE: media_frame.rs
// occt: Media_Frame

//! Media frame data.
#[derive(Clone, Debug, Default)]
pub struct Frame {
    width: u32,
    height: u32,
    format: u32,
    pts: i64,
    data: Vec<u8>,
}

impl Frame {
    pub fn new() -> Self { Self::default() }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn format(&self) -> u32 { self.format }
    pub fn pts(&self) -> i64 { self.pts }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_format(&mut self, fmt: u32) { self.format = fmt; }
    pub fn set_pts(&mut self, p: i64) { self.pts = p; }
    pub fn set_data(&mut self, d: Vec<u8>) { self.data = d; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = Frame::new();
        assert_eq!(f.width(), 0);
    }
}
