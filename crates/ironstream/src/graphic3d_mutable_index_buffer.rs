// FILE: graphic3d_mutable_index_buffer.rs
// occt: Graphic3d_MutableIndexBuffer

/// Range of invalidated buffer data (byte offsets).
#[derive(Debug, Clone, Copy)]
pub struct BufferRange {
    offset: usize,
    size: usize,
}

impl BufferRange {
    /// Creates a new buffer range.
    pub fn new(offset: usize, size: usize) -> Self {
        BufferRange { offset, size }
    }

    /// Clears the range (sets size to 0).
    pub fn clear(&mut self) {
        self.size = 0;
    }

    /// Unites this range with another range.
    pub fn unite(&mut self, other: BufferRange) {
        if other.size == 0 {
            return;
        }
        if self.size == 0 {
            *self = other;
            return;
        }

        let self_end = self.offset + self.size;
        let other_end = other.offset + other.size;

        let new_offset = self.offset.min(other.offset);
        let new_end = self_end.max(other_end);

        self.offset = new_offset;
        self.size = new_end - new_offset;
    }

    /// Returns the offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the size.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Mutable index buffer for graphics rendering.
pub struct Graphic3dMutableIndexBuffer {
    data: Vec<u8>,
    invalidated_range: BufferRange,
    stride: usize,
}

impl Graphic3dMutableIndexBuffer {
    /// Creates a new mutable index buffer with specified stride.
    pub fn new(stride: usize) -> Self {
        Graphic3dMutableIndexBuffer {
            data: Vec::new(),
            invalidated_range: BufferRange::new(0, 0),
            stride,
        }
    }

    /// Returns true if buffer data can be invalidated.
    pub fn is_mutable(&self) -> bool {
        true
    }

    /// Returns the invalidated range.
    pub fn invalidated_range(&self) -> BufferRange {
        self.invalidated_range
    }

    /// Resets the invalidated range.
    pub fn validate(&mut self) {
        self.invalidated_range.clear();
    }

    /// Invalidates the entire buffer.
    pub fn invalidate(&mut self) {
        self.invalidated_range = BufferRange::new(0, self.data.len());
    }

    /// Invalidates a range of indices.
    pub fn invalidate_range(&mut self, index_lower: usize, index_upper: usize) {
        assert!(index_lower <= index_upper, "Invalid index range");
        let byte_lower = index_lower * self.stride;
        let byte_upper = (index_upper + 1) * self.stride;
        let range = BufferRange::new(byte_lower, byte_upper - byte_lower);
        self.invalidated_range.unite(range);
    }

    /// Returns stride value.
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// Returns buffer size in bytes.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_range_creation() {
        let range = BufferRange::new(0, 100);
        assert_eq!(range.offset(), 0);
        assert_eq!(range.size(), 100);
    }

    #[test]
    fn test_buffer_range_clear() {
        let mut range = BufferRange::new(10, 50);
        range.clear();
        assert_eq!(range.size(), 0);
    }

    #[test]
    fn test_buffer_range_unite() {
        let mut range1 = BufferRange::new(0, 50);
        let range2 = BufferRange::new(100, 50);
        range1.unite(range2);
        assert_eq!(range1.offset(), 0);
        assert_eq!(range1.size(), 150);
    }

    #[test]
    fn test_buffer_range_unite_overlap() {
        let mut range1 = BufferRange::new(0, 100);
        let range2 = BufferRange::new(50, 100);
        range1.unite(range2);
        assert_eq!(range1.offset(), 0);
        assert_eq!(range1.size(), 150);
    }

    #[test]
    fn test_mutable_index_buffer_creation() {
        let buf = Graphic3dMutableIndexBuffer::new(4);
        assert!(buf.is_mutable());
        assert_eq!(buf.stride(), 4);
        assert_eq!(buf.size(), 0);
    }

    #[test]
    fn test_mutable_index_buffer_invalidate() {
        let mut buf = Graphic3dMutableIndexBuffer::new(2);
        buf.data.resize(100, 0);
        buf.invalidate();
        assert_eq!(buf.invalidated_range().size(), 100);
    }

    #[test]
    fn test_mutable_index_buffer_invalidate_range() {
        let mut buf = Graphic3dMutableIndexBuffer::new(4);
        buf.invalidate_range(0, 9);
        let range = buf.invalidated_range();
        assert_eq!(range.offset(), 0);
        assert_eq!(range.size(), 40);
    }

    #[test]
    fn test_mutable_index_buffer_validate() {
        let mut buf = Graphic3dMutableIndexBuffer::new(2);
        buf.data.resize(100, 0);
        buf.invalidate();
        assert!(buf.invalidated_range().size() > 0);
        buf.validate();
        assert_eq!(buf.invalidated_range().size(), 0);
    }
}
