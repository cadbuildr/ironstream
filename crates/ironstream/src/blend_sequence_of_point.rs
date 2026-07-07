// FILE: blend_sequence_of_point.rs
// occt: Blend_SequenceOfPoint

//! Deprecated type alias for backward compatibility.
//! @deprecated Since OCCT 8.0.0. Use a sequence collection directly instead.

/// A sequence of Blend_Point objects.
/// This is a deprecated type alias maintained for backward compatibility.
/// In modern OCCT, use a generic sequence type directly.
#[derive(Clone, Debug, Default)]
pub struct BlendSequenceOfPoint {
    items: Vec<i32>, // Placeholder: represents sequence of points
}

impl BlendSequenceOfPoint {
    /// Creates a new empty sequence.
    pub fn new() -> Self {
        BlendSequenceOfPoint {
            items: Vec::new(),
        }
    }

    /// Appends an element to the sequence.
    pub fn append(&mut self, _item: i32) {
        // In a real implementation, this would append a Blend_Point
        self.items.push(0);
    }

    /// Returns the number of elements in the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seq = BlendSequenceOfPoint::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = BlendSequenceOfPoint::new();
        seq.append(1);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_multiple_appends() {
        let mut seq = BlendSequenceOfPoint::new();
        seq.append(1);
        seq.append(2);
        seq.append(3);
        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_default() {
        let seq = BlendSequenceOfPoint::default();
        assert!(seq.is_empty());
    }
}
