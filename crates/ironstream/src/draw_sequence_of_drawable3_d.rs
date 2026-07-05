// FILE: draw_sequence_of_drawable3_d.rs
// occt: Draw_SequenceOfDrawable3D

//! Deprecated: Use standard Rust collections directly.
//! This is a sequence of 3D drawables for backward compatibility.

use std::collections::VecDeque;

/// Deprecated: Use VecDeque<DrawDrawable3D> directly instead.
pub type DrawSequenceOfDrawable3D = VecDeque<u32>;

/// Create a new sequence
pub fn new() -> DrawSequenceOfDrawable3D {
    VecDeque::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq: DrawSequenceOfDrawable3D = new();
        assert_eq!(seq.len(), 0);
    }
}
