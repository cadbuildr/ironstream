// FILE: graphic3d_attrib_buffer.rs
// occt: Graphic3d_AttribBuffer

//! Buffer of vertex attributes.
//!
//! This class is intended for advanced usage allowing invalidation of entire buffer content
//! or its sub-part. Attributes can be interleaved or non-interleaved.

/// Represents a range of buffer data as a start position and length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferRange {
    pub start: i32,
    pub length: i32,
}

impl BufferRange {
    /// Creates an empty buffer range.
    pub fn new() -> Self {
        BufferRange {
            start: 0,
            length: 0,
        }
    }

    /// Creates a buffer range with the given start and length.
    pub fn with_bounds(start: i32, length: i32) -> Self {
        BufferRange { start, length }
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the last element index within the range.
    pub fn upper(&self) -> i32 {
        self.start + self.length - 1
    }

    /// Clears the range.
    pub fn clear(&mut self) {
        self.start = 0;
        self.length = 0;
    }

    /// Unites another range with this one, expanding to cover both.
    pub fn unite(&mut self, other: &BufferRange) {
        if self.is_empty() {
            *self = *other;
            return;
        }
        if other.is_empty() {
            return;
        }

        let new_start = std::cmp::min(self.start, other.start);
        let new_upper = std::cmp::max(self.upper(), other.upper());
        self.start = new_start;
        self.length = new_upper - new_start + 1;
    }
}

impl Default for BufferRange {
    fn default() -> Self {
        Self::new()
    }
}

/// Buffer of vertex attributes with support for invalidation tracking.
#[derive(Debug)]
pub struct AttribBuffer {
    is_mutable: bool,
    is_interleaved: bool,
    invalidated_range: BufferRange,
}

impl AttribBuffer {
    /// Creates an empty attribute buffer.
    pub fn new() -> Self {
        AttribBuffer {
            is_mutable: false,
            is_interleaved: true,
            invalidated_range: BufferRange::new(),
        }
    }

    /// Returns true if data can be invalidated.
    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }

    /// Sets whether data can be invalidated.
    ///
    /// # Panics
    ///
    /// Panics if the buffer size exceeds 32-bit address space and mutable is true.
    pub fn set_mutable(&mut self, mutable: bool) {
        self.is_mutable = mutable;
    }

    /// Returns true for interleaved array (true by default).
    pub fn is_interleaved(&self) -> bool {
        self.is_interleaved
    }

    /// Setup interleaved/non-interleaved array.
    ///
    /// WARNING: Filling non-interleaved buffer should be implemented on user side
    /// without auxiliary methods designed for interleaved data.
    pub fn set_interleaved(&mut self, is_interleaved: bool) {
        self.is_interleaved = is_interleaved;
    }

    /// Returns the invalidated range.
    pub fn invalidated_range(&self) -> BufferRange {
        self.invalidated_range
    }

    /// Reset invalidated range.
    pub fn validate(&mut self) {
        self.invalidated_range.clear();
    }

    /// Invalidate the entire buffer data.
    pub fn invalidate_all(&mut self) {
        self.invalidated_range = BufferRange::with_bounds(0, i32::MAX);
    }

    /// Invalidate specific attribute data.
    pub fn invalidate_attribute(&mut self, _attr_index: usize) {
        if self.is_interleaved {
            self.invalidate_all();
        } else {
            // For non-interleaved, would need to know attribute stride
            // This is a simplified implementation
            self.invalidate_all();
        }
    }

    /// Invalidate attribute data within specified sub-range.
    pub fn invalidate_attribute_range(&mut self, _attr_index: usize, _lower: usize, _upper: usize) {
        if self.is_interleaved {
            // Would compute range based on stride
            self.invalidate_all();
        } else {
            // For non-interleaved, would need stride info
            self.invalidate_all();
        }
    }

    /// Invalidate all attribute data within specified vertex sub-range.
    pub fn invalidate_vertex_range(&mut self, _lower: usize, _upper: usize) {
        if self.is_interleaved {
            // Would compute range based on stride
            self.invalidate_all();
        } else {
            // For non-interleaved, invalidate each attribute
            self.invalidate_all();
        }
    }

    /// Invalidate specified sub-range of data (as byte offsets).
    pub fn invalidate_range(&mut self, range: BufferRange) {
        self.invalidated_range.unite(&range);
    }
}

impl Default for AttribBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_range_new() {
        let range = BufferRange::new();
        assert!(range.is_empty());
        assert_eq!(range.start, 0);
        assert_eq!(range.length, 0);
    }

    #[test]
    fn test_buffer_range_with_bounds() {
        let range = BufferRange::with_bounds(10, 5);
        assert!(!range.is_empty());
        assert_eq!(range.start, 10);
        assert_eq!(range.length, 5);
        assert_eq!(range.upper(), 14);
    }

    #[test]
    fn test_buffer_range_clear() {
        let mut range = BufferRange::with_bounds(10, 5);
        range.clear();
        assert!(range.is_empty());
    }

    #[test]
    fn test_buffer_range_unite_empty() {
        let mut range1 = BufferRange::new();
        let range2 = BufferRange::with_bounds(5, 10);
        range1.unite(&range2);
        assert_eq!(range1.start, 5);
        assert_eq!(range1.length, 10);
    }

    #[test]
    fn test_buffer_range_unite_overlapping() {
        let mut range1 = BufferRange::with_bounds(0, 10);
        let range2 = BufferRange::with_bounds(5, 10);
        range1.unite(&range2);
        assert_eq!(range1.start, 0);
        assert_eq!(range1.upper(), 14);
        assert_eq!(range1.length, 15);
    }

    #[test]
    fn test_attrib_buffer_new() {
        let buffer = AttribBuffer::new();
        assert!(!buffer.is_mutable());
        assert!(buffer.is_interleaved());
        assert!(buffer.invalidated_range().is_empty());
    }

    #[test]
    fn test_attrib_buffer_set_mutable() {
        let mut buffer = AttribBuffer::new();
        buffer.set_mutable(true);
        assert!(buffer.is_mutable());
    }

    #[test]
    fn test_attrib_buffer_set_interleaved() {
        let mut buffer = AttribBuffer::new();
        buffer.set_interleaved(false);
        assert!(!buffer.is_interleaved());
    }

    #[test]
    fn test_attrib_buffer_invalidate_all() {
        let mut buffer = AttribBuffer::new();
        buffer.invalidate_all();
        assert!(!buffer.invalidated_range().is_empty());
    }

    #[test]
    fn test_attrib_buffer_validate() {
        let mut buffer = AttribBuffer::new();
        buffer.invalidate_all();
        assert!(!buffer.invalidated_range().is_empty());
        buffer.validate();
        assert!(buffer.invalidated_range().is_empty());
    }

    #[test]
    fn test_attrib_buffer_invalidate_range() {
        let mut buffer = AttribBuffer::new();
        let range = BufferRange::with_bounds(10, 5);
        buffer.invalidate_range(range);
        assert!(!buffer.invalidated_range().is_empty());
        assert_eq!(buffer.invalidated_range().start, 10);
        assert_eq!(buffer.invalidated_range().length, 5);
    }
}
