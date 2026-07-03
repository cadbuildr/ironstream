// FILE: std_select_type_of_selection_image.rs
// occt: StdSelect_TypeOfSelectionImage

/// Type of output selection image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdSelectTypeOfSelectionImage {
    /// Normalized depth (grayscale)
    NormalizedDepth = 0,
    /// Normalized depth, inverted
    NormalizedDepthInverted = 1,
    /// Unnormalized depth (grayscale)
    UnnormalizedDepth = 2,
    /// Color of detected object
    ColoredDetectedObject = 3,
    /// Random color for each entity
    ColoredEntity = 4,
    /// Random color for each entity type
    ColoredEntityType = 5,
    /// Random color for each owner
    ColoredOwner = 6,
    /// Color of selection mode
    ColoredSelectionMode = 7,
    /// Normal direction values
    SurfaceNormal = 8,
}

impl StdSelectTypeOfSelectionImage {
    /// Convert from integer to enum variant
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(StdSelectTypeOfSelectionImage::NormalizedDepth),
            1 => Some(StdSelectTypeOfSelectionImage::NormalizedDepthInverted),
            2 => Some(StdSelectTypeOfSelectionImage::UnnormalizedDepth),
            3 => Some(StdSelectTypeOfSelectionImage::ColoredDetectedObject),
            4 => Some(StdSelectTypeOfSelectionImage::ColoredEntity),
            5 => Some(StdSelectTypeOfSelectionImage::ColoredEntityType),
            6 => Some(StdSelectTypeOfSelectionImage::ColoredOwner),
            7 => Some(StdSelectTypeOfSelectionImage::ColoredSelectionMode),
            8 => Some(StdSelectTypeOfSelectionImage::SurfaceNormal),
            _ => None,
        }
    }

    /// Convert to integer value
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(StdSelectTypeOfSelectionImage::NormalizedDepth as i32, 0);
        assert_eq!(
            StdSelectTypeOfSelectionImage::NormalizedDepthInverted as i32,
            1
        );
        assert_eq!(StdSelectTypeOfSelectionImage::UnnormalizedDepth as i32, 2);
        assert_eq!(
            StdSelectTypeOfSelectionImage::ColoredDetectedObject as i32,
            3
        );
        assert_eq!(StdSelectTypeOfSelectionImage::ColoredEntity as i32, 4);
        assert_eq!(StdSelectTypeOfSelectionImage::ColoredEntityType as i32, 5);
        assert_eq!(StdSelectTypeOfSelectionImage::ColoredOwner as i32, 6);
        assert_eq!(
            StdSelectTypeOfSelectionImage::ColoredSelectionMode as i32,
            7
        );
        assert_eq!(StdSelectTypeOfSelectionImage::SurfaceNormal as i32, 8);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            StdSelectTypeOfSelectionImage::from_i32(0),
            Some(StdSelectTypeOfSelectionImage::NormalizedDepth)
        );
        assert_eq!(
            StdSelectTypeOfSelectionImage::from_i32(1),
            Some(StdSelectTypeOfSelectionImage::NormalizedDepthInverted)
        );
        assert_eq!(
            StdSelectTypeOfSelectionImage::from_i32(8),
            Some(StdSelectTypeOfSelectionImage::SurfaceNormal)
        );
        assert_eq!(StdSelectTypeOfSelectionImage::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(
            StdSelectTypeOfSelectionImage::NormalizedDepth.to_i32(),
            0
        );
        assert_eq!(
            StdSelectTypeOfSelectionImage::ColoredEntity.to_i32(),
            4
        );
        assert_eq!(StdSelectTypeOfSelectionImage::SurfaceNormal.to_i32(), 8);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for i in 0..=8 {
            if let Some(variant) = StdSelectTypeOfSelectionImage::from_i32(i) {
                assert_eq!(variant.to_i32(), i);
            }
        }
    }
}
