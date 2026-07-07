// FILE: mesh_vs_two_colors.rs
// occt: MeshVS_TwoColors

/// A structure containing two RGB colors, encoded as bitfields.
/// Used to represent paired colors for mesh visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoColors {
    // First color components (8 bits each)
    pub r1: u8,
    pub g1: u8,
    pub b1: u8,
    // Second color components (8 bits each)
    pub r2: u8,
    pub g2: u8,
    pub b2: u8,
}

impl TwoColors {
    /// Creates a new TwoColors structure with the specified components
    pub fn new(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> Self {
        TwoColors { r1, g1, b1, r2, g2, b2 }
    }

    /// Creates TwoColors from two color values (each as 24-bit RGB)
    pub fn from_rgb_values(color1: u32, color2: u32) -> Self {
        let r1 = ((color1 >> 16) & 0xFF) as u8;
        let g1 = ((color1 >> 8) & 0xFF) as u8;
        let b1 = (color1 & 0xFF) as u8;

        let r2 = ((color2 >> 16) & 0xFF) as u8;
        let g2 = ((color2 >> 8) & 0xFF) as u8;
        let b2 = (color2 & 0xFF) as u8;

        TwoColors { r1, g1, b1, r2, g2, b2 }
    }

    /// Returns the first color as a 24-bit RGB value
    pub fn color1(&self) -> u32 {
        ((self.r1 as u32) << 16) | ((self.g1 as u32) << 8) | (self.b1 as u32)
    }

    /// Returns the second color as a 24-bit RGB value
    pub fn color2(&self) -> u32 {
        ((self.r2 as u32) << 16) | ((self.g2 as u32) << 8) | (self.b2 as u32)
    }

    /// Hashes a byte value for the internal hash function
    fn hash_byte(hash: &mut u32, value: u8) {
        *hash = hash.wrapping_add(value as u32);
        *hash = hash.wrapping_add(*hash << 10);
        *hash ^= *hash >> 6;
    }

    /// Computes a hash value for this TwoColors structure
    pub fn hash(&self) -> u32 {
        let mut hash = 0u32;
        Self::hash_byte(&mut hash, self.r1);
        Self::hash_byte(&mut hash, self.g1);
        Self::hash_byte(&mut hash, self.b1);
        Self::hash_byte(&mut hash, self.r2);
        Self::hash_byte(&mut hash, self.g2);
        Self::hash_byte(&mut hash, self.b2);
        hash
    }
}

impl std::hash::Hash for TwoColors {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_colors_creation() {
        let colors = TwoColors::new(255, 0, 0, 0, 255, 0);
        assert_eq!(colors.r1, 255);
        assert_eq!(colors.g1, 0);
        assert_eq!(colors.b1, 0);
        assert_eq!(colors.r2, 0);
        assert_eq!(colors.g2, 255);
        assert_eq!(colors.b2, 0);
    }

    #[test]
    fn test_from_rgb_values() {
        let color1 = 0xFF0000; // Red
        let color2 = 0x00FF00; // Green
        let colors = TwoColors::from_rgb_values(color1, color2);

        assert_eq!(colors.r1, 255);
        assert_eq!(colors.g1, 0);
        assert_eq!(colors.b1, 0);
        assert_eq!(colors.r2, 0);
        assert_eq!(colors.g2, 255);
        assert_eq!(colors.b2, 0);
    }

    #[test]
    fn test_color_accessors() {
        let colors = TwoColors::from_rgb_values(0xFF0000, 0x00FF00);
        assert_eq!(colors.color1(), 0xFF0000);
        assert_eq!(colors.color2(), 0x00FF00);
    }

    #[test]
    fn test_equality() {
        let c1 = TwoColors::new(255, 0, 0, 0, 255, 0);
        let c2 = TwoColors::new(255, 0, 0, 0, 255, 0);
        let c3 = TwoColors::new(0, 255, 0, 255, 0, 0);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_hash() {
        let c1 = TwoColors::new(255, 0, 0, 0, 255, 0);
        let c2 = TwoColors::new(255, 0, 0, 0, 255, 0);
        let c3 = TwoColors::new(0, 255, 0, 255, 0, 0);

        // Equal objects should have the same hash
        assert_eq!(c1.hash(), c2.hash());

        // Different objects may have different hashes (not guaranteed but likely)
        // We just check that hash is computed without panicking
        let _ = c3.hash();
    }

    #[test]
    fn test_copy_clone() {
        let c1 = TwoColors::new(255, 0, 0, 0, 255, 0);
        let c2 = c1;
        let c3 = c1.clone();

        assert_eq!(c1, c2);
        assert_eq!(c1, c3);
    }

    #[test]
    fn test_black_and_white() {
        let bw = TwoColors::from_rgb_values(0x000000, 0xFFFFFF);
        assert_eq!(bw.r1, 0);
        assert_eq!(bw.g1, 0);
        assert_eq!(bw.b1, 0);
        assert_eq!(bw.r2, 255);
        assert_eq!(bw.g2, 255);
        assert_eq!(bw.b2, 255);
    }

    #[test]
    fn test_mixed_colors() {
        let mixed = TwoColors::from_rgb_values(0xABCDEF, 0x123456);
        assert_eq!(mixed.color1(), 0xABCDEF);
        assert_eq!(mixed.color2(), 0x123456);
    }
}
