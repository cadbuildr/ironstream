// FILE: graphic3d_texture_set_bits.rs
// occt: Graphic3d_TextureSetBits
// occt: Graphic3d_TextureUnit

/// Texture unit enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextureUnit {
    // Regular texture units (0-15)
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
    Unit8,
    Unit9,
    Unit10,
    Unit11,
    Unit12,
    Unit13,
    Unit14,
    Unit15,

    // Reserved units for special purposes (negative indices)
    DepthPeelingDepth,    // -6
    DepthPeelingFrontColor, // -5
    ShadowMap,            // -4
    PbrEnvironmentLUT,    // -3
    PbrIblDiffuseSH,      // -2
    PbrIblSpecular,       // -1
}

impl TextureUnit {
    /// Returns the numeric index of the texture unit.
    pub fn as_index(&self) -> i32 {
        match self {
            TextureUnit::Unit0 => 0,
            TextureUnit::Unit1 => 1,
            TextureUnit::Unit2 => 2,
            TextureUnit::Unit3 => 3,
            TextureUnit::Unit4 => 4,
            TextureUnit::Unit5 => 5,
            TextureUnit::Unit6 => 6,
            TextureUnit::Unit7 => 7,
            TextureUnit::Unit8 => 8,
            TextureUnit::Unit9 => 9,
            TextureUnit::Unit10 => 10,
            TextureUnit::Unit11 => 11,
            TextureUnit::Unit12 => 12,
            TextureUnit::Unit13 => 13,
            TextureUnit::Unit14 => 14,
            TextureUnit::Unit15 => 15,
            TextureUnit::DepthPeelingDepth => -6,
            TextureUnit::DepthPeelingFrontColor => -5,
            TextureUnit::ShadowMap => -4,
            TextureUnit::PbrEnvironmentLUT => -3,
            TextureUnit::PbrIblDiffuseSH => -2,
            TextureUnit::PbrIblSpecular => -1,
        }
    }

    /// Aliases for semantic texture units
    pub const BASECOLOR: TextureUnit = TextureUnit::Unit0;
    pub const EMISSIVE: TextureUnit = TextureUnit::Unit1;
    pub const OCCLUSION: TextureUnit = TextureUnit::Unit2;
    pub const NORMAL: TextureUnit = TextureUnit::Unit3;
    pub const METALLIC_ROUGHNESS: TextureUnit = TextureUnit::Unit4;
    pub const ENV_MAP: TextureUnit = TextureUnit::Unit0;
    pub const POINT_SPRITE: TextureUnit = TextureUnit::Unit1;
}

/// Standard texture units combination bits.
/// These are bit flags for the texture units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureSetBits(pub u32);

impl TextureSetBits {
    /// None - no texture units.
    pub const NONE: TextureSetBits = TextureSetBits(0);

    /// BaseColor texture unit bit.
    pub const BASE_COLOR: TextureSetBits = TextureSetBits(1 << 0); // Unit0

    /// Emissive texture unit bit.
    pub const EMISSIVE: TextureSetBits = TextureSetBits(1 << 1); // Unit1

    /// Occlusion texture unit bit.
    pub const OCCLUSION: TextureSetBits = TextureSetBits(1 << 2); // Unit2

    /// Normal texture unit bit.
    pub const NORMAL: TextureSetBits = TextureSetBits(1 << 3); // Unit3

    /// MetallicRoughness texture unit bit.
    pub const METALLIC_ROUGHNESS: TextureSetBits = TextureSetBits(1 << 4); // Unit4

    /// Check if a specific bit is set.
    pub fn contains(&self, other: TextureSetBits) -> bool {
        (self.0 & other.0) != 0
    }

    /// Check if all bits of another set are contained.
    pub fn contains_all(&self, other: TextureSetBits) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Set a specific bit.
    pub fn insert(&mut self, other: TextureSetBits) {
        self.0 |= other.0;
    }

    /// Remove a specific bit.
    pub fn remove(&mut self, other: TextureSetBits) {
        self.0 &= !other.0;
    }

    /// Toggle a specific bit.
    pub fn toggle(&mut self, other: TextureSetBits) {
        self.0 ^= other.0;
    }

    /// Get the raw bit value.
    pub fn bits(&self) -> u32 {
        self.0
    }

    /// Create from raw bit value.
    pub fn from_bits(bits: u32) -> Self {
        TextureSetBits(bits)
    }
}

impl Default for TextureSetBits {
    fn default() -> Self {
        TextureSetBits::NONE
    }
}

impl std::ops::BitOr for TextureSetBits {
    type Output = TextureSetBits;

    fn bitor(self, rhs: TextureSetBits) -> TextureSetBits {
        TextureSetBits(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for TextureSetBits {
    fn bitor_assign(&mut self, rhs: TextureSetBits) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for TextureSetBits {
    type Output = TextureSetBits;

    fn bitand(self, rhs: TextureSetBits) -> TextureSetBits {
        TextureSetBits(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for TextureSetBits {
    fn bitand_assign(&mut self, rhs: TextureSetBits) {
        self.0 &= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_unit_indices() {
        assert_eq!(TextureUnit::Unit0.as_index(), 0);
        assert_eq!(TextureUnit::Unit1.as_index(), 1);
        assert_eq!(TextureUnit::Unit5.as_index(), 5);
        assert_eq!(TextureUnit::Unit15.as_index(), 15);
        assert_eq!(TextureUnit::DepthPeelingDepth.as_index(), -6);
        assert_eq!(TextureUnit::DepthPeelingFrontColor.as_index(), -5);
        assert_eq!(TextureUnit::ShadowMap.as_index(), -4);
        assert_eq!(TextureUnit::PbrEnvironmentLUT.as_index(), -3);
        assert_eq!(TextureUnit::PbrIblDiffuseSH.as_index(), -2);
        assert_eq!(TextureUnit::PbrIblSpecular.as_index(), -1);
    }

    #[test]
    fn test_texture_unit_aliases() {
        assert_eq!(TextureUnit::BASECOLOR.as_index(), 0);
        assert_eq!(TextureUnit::EMISSIVE.as_index(), 1);
        assert_eq!(TextureUnit::OCCLUSION.as_index(), 2);
        assert_eq!(TextureUnit::NORMAL.as_index(), 3);
        assert_eq!(TextureUnit::METALLIC_ROUGHNESS.as_index(), 4);
        assert_eq!(TextureUnit::ENV_MAP.as_index(), 0);
        assert_eq!(TextureUnit::POINT_SPRITE.as_index(), 1);
    }

    #[test]
    fn test_texture_set_bits_none() {
        let bits = TextureSetBits::NONE;
        assert_eq!(bits.bits(), 0);
        assert!(!bits.contains(TextureSetBits::BASE_COLOR));
    }

    #[test]
    fn test_texture_set_bits_individual() {
        assert_eq!(TextureSetBits::BASE_COLOR.bits(), 1);
        assert_eq!(TextureSetBits::EMISSIVE.bits(), 2);
        assert_eq!(TextureSetBits::OCCLUSION.bits(), 4);
        assert_eq!(TextureSetBits::NORMAL.bits(), 8);
        assert_eq!(TextureSetBits::METALLIC_ROUGHNESS.bits(), 16);
    }

    #[test]
    fn test_contains() {
        let bits = TextureSetBits::BASE_COLOR;
        assert!(bits.contains(TextureSetBits::BASE_COLOR));
        assert!(!bits.contains(TextureSetBits::EMISSIVE));
    }

    #[test]
    fn test_contains_all() {
        let bits = TextureSetBits::BASE_COLOR | TextureSetBits::EMISSIVE;
        assert!(bits.contains_all(TextureSetBits::BASE_COLOR));
        assert!(bits.contains_all(TextureSetBits::EMISSIVE));
        assert!(bits.contains_all(TextureSetBits::BASE_COLOR | TextureSetBits::EMISSIVE));
        assert!(!bits.contains_all(TextureSetBits::NORMAL));
    }

    #[test]
    fn test_insert() {
        let mut bits = TextureSetBits::NONE;
        bits.insert(TextureSetBits::BASE_COLOR);
        assert_eq!(bits.bits(), 1);
        bits.insert(TextureSetBits::EMISSIVE);
        assert_eq!(bits.bits(), 3);
    }

    #[test]
    fn test_remove() {
        let mut bits = TextureSetBits::BASE_COLOR | TextureSetBits::EMISSIVE;
        assert_eq!(bits.bits(), 3);
        bits.remove(TextureSetBits::BASE_COLOR);
        assert_eq!(bits.bits(), 2);
    }

    #[test]
    fn test_toggle() {
        let mut bits = TextureSetBits::NONE;
        bits.toggle(TextureSetBits::BASE_COLOR);
        assert_eq!(bits.bits(), 1);
        bits.toggle(TextureSetBits::BASE_COLOR);
        assert_eq!(bits.bits(), 0);
    }

    #[test]
    fn test_bitor() {
        let bits = TextureSetBits::BASE_COLOR | TextureSetBits::EMISSIVE | TextureSetBits::NORMAL;
        assert_eq!(bits.bits(), 11); // 0001 | 0010 | 1000 = 1011
    }

    #[test]
    fn test_bitand() {
        let bits1 = TextureSetBits::BASE_COLOR | TextureSetBits::EMISSIVE;
        let bits2 = TextureSetBits::EMISSIVE | TextureSetBits::NORMAL;
        let result = bits1 & bits2;
        assert_eq!(result.bits(), 2); // Only EMISSIVE remains
    }

    #[test]
    fn test_default() {
        let bits = TextureSetBits::default();
        assert_eq!(bits.bits(), 0);
    }

    #[test]
    fn test_complex_operations() {
        let mut bits = TextureSetBits::NONE;
        bits |= TextureSetBits::BASE_COLOR;
        bits |= TextureSetBits::NORMAL;
        bits |= TextureSetBits::METALLIC_ROUGHNESS;

        assert!(bits.contains(TextureSetBits::BASE_COLOR));
        assert!(bits.contains(TextureSetBits::NORMAL));
        assert!(bits.contains(TextureSetBits::METALLIC_ROUGHNESS));
        assert!(!bits.contains(TextureSetBits::EMISSIVE));
    }
}
