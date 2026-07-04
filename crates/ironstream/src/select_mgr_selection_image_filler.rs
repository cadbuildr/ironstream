// FILE: select_mgr_selection_image_filler.rs
// occt: SelectMgr_SelectionImageFiller

use std::collections::HashSet;

/// Pseudo-random number generator (simple implementation)
pub struct SimpleBullardGenerator {
    seed: u64,
}

impl SimpleBullardGenerator {
    /// Creates a new generator with default seed
    pub fn new() -> Self {
        SimpleBullardGenerator { seed: 12345 }
    }

    /// Returns next random integer
    pub fn next_int(&mut self) -> i32 {
        // Linear congruential generator
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.seed / 65536) % 32768) as i32
    }
}

impl Default for SimpleBullardGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Color representation in sRGB space
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuantityColor {
    r: u8,
    g: u8,
    b: u8,
}

impl QuantityColor {
    /// Creates a new color from RGB components (0.0 to 1.0)
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        QuantityColor {
            r: (r * 255.0).max(0.0).min(255.0) as u8,
            g: (g * 255.0).max(0.0).min(255.0) as u8,
            b: (b * 255.0).max(0.0).min(255.0) as u8,
        }
    }

    /// Returns the components as normalized floats
    pub fn components(&self) -> (f64, f64, f64) {
        (
            f64::from(self.r) / 255.0,
            f64::from(self.g) / 255.0,
            f64::from(self.b) / 255.0,
        )
    }
}

/// Selection image filler type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionImageType {
    /// Color-based selection
    ColorBased,
    /// Entity-based selection
    EntityBased,
    /// Ownership-based selection
    OwnershipBased,
}

/// Abstract base class for filling pixels in a selection image.
pub struct SelectMgrSelectionImageFiller {
    /// The image being filled
    image_width: usize,
    image_height: usize,
    /// Random color generator
    bullard_generator: SimpleBullardGenerator,
    /// Set of unique colors used
    unique_colors: HashSet<QuantityColor>,
}

impl SelectMgrSelectionImageFiller {
    /// Main constructor
    pub fn new(width: usize, height: usize) -> Self {
        SelectMgrSelectionImageFiller {
            image_width: width,
            image_height: height,
            bullard_generator: SimpleBullardGenerator::new(),
            unique_colors: HashSet::new(),
        }
    }

    /// Fill pixel at specified position (abstract, to be overridden)
    pub fn fill(&mut self, _col: usize, _row: usize, _picked: i32) {
        // Override in subclass
    }

    /// Flush results into final image
    pub fn flush(&mut self) {
        // Override in subclass
    }

    /// Find a new unique random color
    pub fn random_pastel_color(&mut self) -> QuantityColor {
        loop {
            let color = self.next_random_pastel_color();
            if self.unique_colors.insert(color.clone()) {
                return color;
            }
        }
    }

    /// Generate next random pastel color
    pub fn next_random_pastel_color(&mut self) -> QuantityColor {
        let r = f64::from((self.bullard_generator.next_int() % 256) as u8) / 255.0;
        let g = f64::from((self.bullard_generator.next_int() % 256) as u8) / 255.0;
        let b = f64::from((self.bullard_generator.next_int() % 256) as u8) / 255.0;
        QuantityColor::new(r, g, b)
    }

    /// Get image dimensions
    pub fn image_dimensions(&self) -> (usize, usize) {
        (self.image_width, self.image_height)
    }

    /// Get unique colors count
    pub fn unique_colors_count(&self) -> usize {
        self.unique_colors.len()
    }

    /// Clear unique colors set
    pub fn clear_colors(&mut self) {
        self.unique_colors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullard_generator() {
        let mut gen = SimpleBullardGenerator::new();
        let val1 = gen.next_int();
        let val2 = gen.next_int();
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_color_creation() {
        let color = QuantityColor::new(1.0, 0.5, 0.0);
        let (r, g, b) = color.components();
        assert!((r - 1.0).abs() < 0.01);
        assert!((g - 0.5).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_color_clamping() {
        let color = QuantityColor::new(2.0, -0.5, 0.5);
        let (r, g, b) = color.components();
        assert!(r <= 1.0);
        assert!(g >= 0.0);
    }

    #[test]
    fn test_filler_creation() {
        let filler = SelectMgrSelectionImageFiller::new(800, 600);
        assert_eq!(filler.image_dimensions(), (800, 600));
    }

    #[test]
    fn test_random_pastel_color() {
        let mut filler = SelectMgrSelectionImageFiller::new(100, 100);
        let color1 = filler.random_pastel_color();
        let color2 = filler.random_pastel_color();
        assert_eq!(filler.unique_colors_count(), 2);
        assert_ne!(color1, color2);
    }

    #[test]
    fn test_unique_colors() {
        let mut filler = SelectMgrSelectionImageFiller::new(100, 100);
        for _ in 0..10 {
            let _color = filler.random_pastel_color();
        }
        assert_eq!(filler.unique_colors_count(), 10);
    }

    #[test]
    fn test_clear_colors() {
        let mut filler = SelectMgrSelectionImageFiller::new(100, 100);
        let _color = filler.random_pastel_color();
        assert_eq!(filler.unique_colors_count(), 1);
        filler.clear_colors();
        assert_eq!(filler.unique_colors_count(), 0);
    }
}
