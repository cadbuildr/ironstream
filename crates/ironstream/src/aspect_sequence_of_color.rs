// FILE: aspect_sequence_of_color.rs
// occt: Aspect_SequenceOfColor

//! Deprecated NCollection alias: Sequence<Color>

/// Color (stub).
#[derive(Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// Sequence of colors.
pub type AspectSequenceOfColor = Vec<Color>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: AspectSequenceOfColor = Vec::new();
        seq.push(Color { r: 1.0, g: 0.0, b: 0.0 });
        assert_eq!(seq.len(), 1);
    }
}
