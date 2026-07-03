// FILE: geom2d_hatch_hatching.rs
// occt: Geom2dHatch_Hatching

/// A hatching pattern.
#[derive(Debug, Clone)]
pub struct Geom2dHatchHatching {
    index: i32,
    orientation: i32,
}

impl Geom2dHatchHatching {
    pub fn new(index: i32, orientation: i32) -> Self {
        Geom2dHatchHatching { index, orientation }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hatching_creation() {
        let hatch = Geom2dHatchHatching::new(1, 0);
        assert_eq!(hatch.index(), 1);
        assert_eq!(hatch.orientation(), 0);
    }
}
