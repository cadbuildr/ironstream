// FILE: graphic3d_trans_mode_flags.rs
// occt: Graphic3d_TransModeFlags

/// Transform Persistence Mode defining whether to lock in object position, rotation and / or
/// zooming relative to camera position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransModeFlags(pub u32);

impl TransModeFlags {
    /// no persistence attributes (normal 3D object)
    pub const NONE: TransModeFlags = TransModeFlags(0x0000);

    /// object does not resize
    pub const ZOOM_PERS: TransModeFlags = TransModeFlags(0x0002);

    /// object does not rotate
    pub const ROTATE_PERS: TransModeFlags = TransModeFlags(0x0008);

    /// object behaves like trihedron - it is fixed at the corner of view and does not resize
    /// (but rotating)
    pub const TRIHEDRON_PERS: TransModeFlags = TransModeFlags(0x0020);

    /// object is defined in 2D screen coordinates (pixels) and does not resize, pan and rotate
    pub const 2D: TransModeFlags = TransModeFlags(0x0040);

    /// object is in front of the camera
    pub const CAMERA_PERS: TransModeFlags = TransModeFlags(0x0080);

    /// object is forced to be rendered with orthographic projection
    pub const ORTHO_PERS: TransModeFlags = TransModeFlags(0x0100);

    /// object does not resize with axial scale
    pub const AXIAL_SCALE_PERS: TransModeFlags = TransModeFlags(0x0200);

    /// object doesn't resize and rotate
    pub const ZOOM_ROTATE_PERS: TransModeFlags =
        TransModeFlags(0x0002 | 0x0008); // ZOOM_PERS | ROTATE_PERS

    /// object does not visually resize with either object or axial scale
    pub const AXIAL_ZOOM_PERS: TransModeFlags =
        TransModeFlags(0x0002 | 0x0200); // ZOOM_PERS | AXIAL_SCALE_PERS

    /// Check if a specific flag is set.
    pub fn contains(&self, other: TransModeFlags) -> bool {
        (self.0 & other.0) != 0
    }

    /// Check if all bits of another set are contained.
    pub fn contains_all(&self, other: TransModeFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Set a specific flag.
    pub fn insert(&mut self, other: TransModeFlags) {
        self.0 |= other.0;
    }

    /// Remove a specific flag.
    pub fn remove(&mut self, other: TransModeFlags) {
        self.0 &= !other.0;
    }

    /// Toggle a specific flag.
    pub fn toggle(&mut self, other: TransModeFlags) {
        self.0 ^= other.0;
    }

    /// Get the raw bit value.
    pub fn bits(&self) -> u32 {
        self.0
    }

    /// Create from raw bit value.
    pub fn from_bits(bits: u32) -> Self {
        TransModeFlags(bits)
    }
}

impl Default for TransModeFlags {
    fn default() -> Self {
        TransModeFlags::NONE
    }
}

impl std::ops::BitOr for TransModeFlags {
    type Output = TransModeFlags;

    fn bitor(self, rhs: TransModeFlags) -> TransModeFlags {
        TransModeFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for TransModeFlags {
    fn bitor_assign(&mut self, rhs: TransModeFlags) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for TransModeFlags {
    type Output = TransModeFlags;

    fn bitand(self, rhs: TransModeFlags) -> TransModeFlags {
        TransModeFlags(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for TransModeFlags {
    fn bitand_assign(&mut self, rhs: TransModeFlags) {
        self.0 &= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trans_mode_flags_values() {
        assert_eq!(TransModeFlags::NONE.bits(), 0x0000);
        assert_eq!(TransModeFlags::ZOOM_PERS.bits(), 0x0002);
        assert_eq!(TransModeFlags::ROTATE_PERS.bits(), 0x0008);
        assert_eq!(TransModeFlags::TRIHEDRON_PERS.bits(), 0x0020);
        assert_eq!(TransModeFlags::2D.bits(), 0x0040);
        assert_eq!(TransModeFlags::CAMERA_PERS.bits(), 0x0080);
        assert_eq!(TransModeFlags::ORTHO_PERS.bits(), 0x0100);
        assert_eq!(TransModeFlags::AXIAL_SCALE_PERS.bits(), 0x0200);
    }

    #[test]
    fn test_zoom_rotate_pers() {
        let combined = TransModeFlags::ZOOM_ROTATE_PERS;
        assert_eq!(combined.bits(), 0x000A); // 0x0002 | 0x0008
    }

    #[test]
    fn test_axial_zoom_pers() {
        let combined = TransModeFlags::AXIAL_ZOOM_PERS;
        assert_eq!(combined.bits(), 0x0202); // 0x0002 | 0x0200
    }

    #[test]
    fn test_contains() {
        let flags = TransModeFlags::ZOOM_PERS;
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        assert!(!flags.contains(TransModeFlags::ROTATE_PERS));
    }

    #[test]
    fn test_contains_all() {
        let flags = TransModeFlags::ZOOM_ROTATE_PERS;
        assert!(flags.contains_all(TransModeFlags::ZOOM_PERS));
        assert!(flags.contains_all(TransModeFlags::ROTATE_PERS));
        assert!(flags.contains_all(TransModeFlags::ZOOM_ROTATE_PERS));
        assert!(!flags.contains_all(TransModeFlags::TRIHEDRON_PERS));
    }

    #[test]
    fn test_insert() {
        let mut flags = TransModeFlags::NONE;
        flags.insert(TransModeFlags::ZOOM_PERS);
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        flags.insert(TransModeFlags::ROTATE_PERS);
        assert!(flags.contains(TransModeFlags::ROTATE_PERS));
    }

    #[test]
    fn test_remove() {
        let mut flags = TransModeFlags::ZOOM_ROTATE_PERS;
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        flags.remove(TransModeFlags::ZOOM_PERS);
        assert!(!flags.contains(TransModeFlags::ZOOM_PERS));
        assert!(flags.contains(TransModeFlags::ROTATE_PERS));
    }

    #[test]
    fn test_toggle() {
        let mut flags = TransModeFlags::NONE;
        flags.toggle(TransModeFlags::ZOOM_PERS);
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        flags.toggle(TransModeFlags::ZOOM_PERS);
        assert!(!flags.contains(TransModeFlags::ZOOM_PERS));
    }

    #[test]
    fn test_bitor() {
        let flags = TransModeFlags::ZOOM_PERS | TransModeFlags::ROTATE_PERS | TransModeFlags::CAMERA_PERS;
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        assert!(flags.contains(TransModeFlags::ROTATE_PERS));
        assert!(flags.contains(TransModeFlags::CAMERA_PERS));
    }

    #[test]
    fn test_bitand() {
        let flags1 = TransModeFlags::ZOOM_PERS | TransModeFlags::ROTATE_PERS;
        let flags2 = TransModeFlags::ROTATE_PERS | TransModeFlags::CAMERA_PERS;
        let result = flags1 & flags2;
        assert_eq!(result.bits(), TransModeFlags::ROTATE_PERS.bits());
    }

    #[test]
    fn test_default() {
        let flags = TransModeFlags::default();
        assert_eq!(flags, TransModeFlags::NONE);
    }

    #[test]
    fn test_from_bits() {
        let flags = TransModeFlags::from_bits(0x000A);
        assert!(flags.contains(TransModeFlags::ZOOM_PERS));
        assert!(flags.contains(TransModeFlags::ROTATE_PERS));
    }
}
