// FILE: media_codec_context.rs
// occt: Media_CodecContext

//! Media codec context.
#[derive(Clone, Debug, Default)]
pub struct CodecContext {
    codec_id: u32,
    width: u32,
    height: u32,
    bit_rate: u64,
}

impl CodecContext {
    pub fn new() -> Self { Self::default() }
    pub fn codec_id(&self) -> u32 { self.codec_id }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn bit_rate(&self) -> u64 { self.bit_rate }
    pub fn set_codec_id(&mut self, id: u32) { self.codec_id = id; }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_bit_rate(&mut self, rate: u64) { self.bit_rate = rate; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ctx = CodecContext::new();
        assert_eq!(ctx.codec_id(), 0);
    }
}
