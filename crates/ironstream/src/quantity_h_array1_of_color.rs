// FILE: quantity_h_array1_of_color.rs
// occt: Quantity_HArray1OfColor

//! Deprecated: Quantity_HArray1OfColor is a handle wrapper for color array.

/// RGB color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn red(&self) -> f32 {
        self.r
    }

    pub fn green(&self) -> f32 {
        self.g
    }

    pub fn blue(&self) -> f32 {
        self.b
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

/// Handle array of colors
#[derive(Debug, Clone)]
pub struct HArray1 {
    colors: Vec<Color>,
}

impl HArray1 {
    pub fn new(size: usize) -> Self {
        Self {
            colors: vec![Color::default(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    pub fn value(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut Color> {
        self.colors.get_mut(index)
    }
}

pub type QuantityHArray1OfColor = HArray1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = HArray1::new(10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_value() {
        let arr = HArray1::new(5);
        assert!(arr.value(0).is_some());
        assert!(arr.value(5).is_none());
    }

    #[test]
    fn test_default_color() {
        let c = Color::default();
        assert_eq!(c.red(), 0.0);
    }
}
