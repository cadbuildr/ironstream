// FILE: quantity_array1_of_color.rs
// occt: Quantity_Array1OfColor

//! Deprecated: Quantity_Array1OfColor is a type alias for NCollection_Array1.

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

/// Array1 of colors
#[derive(Debug, Clone)]
pub struct Array1 {
    colors: Vec<Color>,
    lower: usize,
}

impl Array1 {
    pub fn new(lower: usize, upper: usize) -> Self {
        let len = upper.saturating_sub(lower) + 1;
        Self {
            colors: vec![Color::default(); len],
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.lower + self.colors.len().saturating_sub(1)
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn value(&self, index: usize) -> Option<Color> {
        if index >= self.lower {
            self.colors.get(index - self.lower).copied()
        } else {
            None
        }
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut Color> {
        if index >= self.lower {
            self.colors.get_mut(index - self.lower)
        } else {
            None
        }
    }
}

pub type QuantityArray1OfColor = Array1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_create() {
        let c = Color::new(0.5, 0.5, 0.5);
        assert_eq!(c.red(), 0.5);
    }

    #[test]
    fn test_array_create() {
        let arr = Array1::new(0, 9);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_value() {
        let arr = Array1::new(1, 5);
        assert!(arr.value(1).is_some());
    }
}
