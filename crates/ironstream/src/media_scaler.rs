// FILE: media_scaler.rs
// occt: Media_Scaler

//! Media frame scaler.
#[derive(Clone, Debug, Default)]
pub struct Scaler {
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
}

impl Scaler {
    pub fn new() -> Self { Self::default() }
    pub fn src_width(&self) -> u32 { self.src_width }
    pub fn src_height(&self) -> u32 { self.src_height }
    pub fn dst_width(&self) -> u32 { self.dst_width }
    pub fn dst_height(&self) -> u32 { self.dst_height }
    pub fn set_src_dimensions(&mut self, w: u32, h: u32) { self.src_width = w; self.src_height = h; }
    pub fn set_dst_dimensions(&mut self, w: u32, h: u32) { self.dst_width = w; self.dst_height = h; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Scaler::new();
        assert_eq!(s.src_width(), 0);
    }
}
