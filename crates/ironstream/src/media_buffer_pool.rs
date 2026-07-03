// FILE: media_buffer_pool.rs
// occt: Media_BufferPool

//! Media buffer pool.
#[derive(Clone, Debug, Default)]
pub struct BufferPool {
    buffers: Vec<Vec<u8>>,
}

impl BufferPool {
    pub fn new() -> Self { Self { buffers: Vec::new() } }
    pub fn allocate(&mut self, size: usize) -> usize {
        self.buffers.push(vec![0u8; size]);
        self.buffers.len() - 1
    }
    pub fn buffer(&self, id: usize) -> Option<&[u8]> { self.buffers.get(id).map(|b| b.as_slice()) }
    pub fn count(&self) -> usize { self.buffers.len() }
    pub fn clear(&mut self) { self.buffers.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut pool = BufferPool::new();
        let id = pool.allocate(100);
        assert_eq!(id, 0);
    }
}
