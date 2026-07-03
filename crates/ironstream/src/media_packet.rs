// FILE: media_packet.rs
// occt: Media_Packet

//! Media packet data.
#[derive(Clone, Debug, Default)]
pub struct Packet {
    stream_index: i32,
    data: Vec<u8>,
    size: usize,
    pts: i64,
    dts: i64,
}

impl Packet {
    pub fn new() -> Self { Self::default() }
    pub fn stream_index(&self) -> i32 { self.stream_index }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn size(&self) -> usize { self.size }
    pub fn pts(&self) -> i64 { self.pts }
    pub fn dts(&self) -> i64 { self.dts }
    pub fn set_stream_index(&mut self, idx: i32) { self.stream_index = idx; }
    pub fn set_data(&mut self, d: Vec<u8>) { self.size = d.len(); self.data = d; }
    pub fn set_pts(&mut self, p: i64) { self.pts = p; }
    pub fn set_dts(&mut self, d: i64) { self.dts = d; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Packet::new();
        assert_eq!(p.stream_index(), 0);
    }
}
