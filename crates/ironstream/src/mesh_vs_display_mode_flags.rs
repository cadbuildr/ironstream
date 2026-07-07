// FILE: mesh_vs_display_mode_flags.rs
// occt: MeshVS_DisplayModeFlags

/// Display mode flags for MeshVS visualization.
/// These flags define different presentation modes for mesh visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayModeFlags(pub i32);

impl DisplayModeFlags {
    /// Wire frame display mode
    pub const WIREFRAME: Self = DisplayModeFlags(0x0001);

    /// Shading display mode
    pub const SHADING: Self = DisplayModeFlags(0x0002);

    /// Shrink display mode
    pub const SHRINK: Self = DisplayModeFlags(0x0003);

    /// Mask to pick out OpenCascade display mode
    pub const OCC_MASK: Self = DisplayModeFlags(0x0003);

    /// Vector data presentation
    pub const VECTOR_DATA_PRS: Self = DisplayModeFlags(0x0004);

    /// Nodal color data presentation
    pub const NODAL_COLOR_DATA_PRS: Self = DisplayModeFlags(0x0008);

    /// Elemental color data presentation
    pub const ELEMENTAL_COLOR_DATA_PRS: Self = DisplayModeFlags(0x0010);

    /// Text data presentation
    pub const TEXT_DATA_PRS: Self = DisplayModeFlags(0x0020);

    /// Entities with data
    pub const ENTITIES_WITH_DATA: Self = DisplayModeFlags(0x0040);

    /// Deformed presentation wire frame
    pub const DEFORMED_PRS_WIREFRAME: Self = DisplayModeFlags(0x0080);

    /// Deformed presentation shading
    pub const DEFORMED_PRS_SHADING: Self = DisplayModeFlags(0x0100);

    /// Deformed presentation shrink
    pub const DEFORMED_PRS_SHRINK: Self = DisplayModeFlags(0x0180);

    /// Mask for deformed presentations
    pub const DEFORMED_MASK: Self = DisplayModeFlags(0x0180);

    /// Selection presentation
    pub const SELECTION_PRS: Self = DisplayModeFlags(0x0200);

    /// Highlight presentation
    pub const HILIGHT_PRS: Self = DisplayModeFlags(0x0400);

    /// User-defined presentation
    pub const USER: Self = DisplayModeFlags(0x0800);

    /// Creates a new DisplayModeFlags from a raw i32 value
    pub fn new(value: i32) -> Self {
        DisplayModeFlags(value)
    }

    /// Returns the raw i32 value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Checks if this flag contains all bits of the given flag.
    /// Note that per OCCT `MeshVS_DisplayModeFlags.hxx`, `MeshVS_DMF_Shrink`
    /// (0x0003) shares its bits with `WireFrame | Shading`, so containment of
    /// SHRINK is equivalent to containment of both WIREFRAME and SHADING.
    pub fn contains(&self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Sets the given flag
    pub fn set(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    /// Clears the given flag
    pub fn clear(&mut self, flag: Self) {
        self.0 &= !flag.0;
    }
}

impl std::ops::BitOr for DisplayModeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        DisplayModeFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for DisplayModeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        DisplayModeFlags(self.0 & rhs.0)
    }
}

impl From<i32> for DisplayModeFlags {
    fn from(value: i32) -> Self {
        DisplayModeFlags(value)
    }
}

impl From<DisplayModeFlags> for i32 {
    fn from(flags: DisplayModeFlags) -> Self {
        flags.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_values() {
        assert_eq!(DisplayModeFlags::WIREFRAME.value(), 0x0001);
        assert_eq!(DisplayModeFlags::SHADING.value(), 0x0002);
        assert_eq!(DisplayModeFlags::SHRINK.value(), 0x0003);
        assert_eq!(DisplayModeFlags::VECTOR_DATA_PRS.value(), 0x0004);
    }

    #[test]
    fn test_contains() {
        let flags = DisplayModeFlags::WIREFRAME | DisplayModeFlags::SHADING;
        assert!(flags.contains(DisplayModeFlags::WIREFRAME));
        assert!(flags.contains(DisplayModeFlags::SHADING));
        // In OCCT, MeshVS_DMF_Shrink = 0x0003 = WireFrame | Shading, so the
        // combination is bit-identical to SHRINK and contains it.
        assert!(flags.contains(DisplayModeFlags::SHRINK));
        // A flag outside the OCC mask is not contained.
        assert!(!flags.contains(DisplayModeFlags::VECTOR_DATA_PRS));
    }

    #[test]
    fn test_set_clear() {
        let mut flags = DisplayModeFlags::WIREFRAME;
        flags.set(DisplayModeFlags::SHADING);
        assert!(flags.contains(DisplayModeFlags::WIREFRAME));
        assert!(flags.contains(DisplayModeFlags::SHADING));

        flags.clear(DisplayModeFlags::WIREFRAME);
        assert!(!flags.contains(DisplayModeFlags::WIREFRAME));
        assert!(flags.contains(DisplayModeFlags::SHADING));
    }

    #[test]
    fn test_bitwse_operations() {
        let f1 = DisplayModeFlags::WIREFRAME;
        let f2 = DisplayModeFlags::SHADING;
        let combined = f1 | f2;
        assert!(combined.contains(DisplayModeFlags::WIREFRAME));
        assert!(combined.contains(DisplayModeFlags::SHADING));

        let masked = combined & DisplayModeFlags::WIREFRAME;
        assert_eq!(masked.value(), DisplayModeFlags::WIREFRAME.value());
    }

    #[test]
    fn test_from_i32() {
        let flags: DisplayModeFlags = 0x0001i32.into();
        assert_eq!(flags.value(), 0x0001);
    }

    #[test]
    fn test_occ_mask() {
        let flags = DisplayModeFlags::WIREFRAME | DisplayModeFlags::SHADING;
        let masked = flags & DisplayModeFlags::OCC_MASK;
        assert_eq!(masked.value(), DisplayModeFlags::OCC_MASK.value());
    }
}
