// FILE: mesh_vs_selection_mode_flags.rs
// occt: MeshVS_SelectionModeFlags

/// Selection mode flags for mesh visualization.
/// Defines which entities can be selected in a mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionModeFlags(pub i32);

impl SelectionModeFlags {
    /// Mesh selection mode (no specific entity)
    pub const MESH: Self = SelectionModeFlags(0x0000);

    /// Node selection mode
    pub const NODE: Self = SelectionModeFlags(0x0001);

    /// 0D element selection mode
    pub const ELEMENT_0D: Self = SelectionModeFlags(0x0002);

    /// Link (edge) selection mode
    pub const LINK: Self = SelectionModeFlags(0x0004);

    /// Face selection mode
    pub const FACE: Self = SelectionModeFlags(0x0008);

    /// Volume selection mode
    pub const VOLUME: Self = SelectionModeFlags(0x0010);

    /// All element types (0D, Link, Face, Volume)
    pub const ELEMENT: Self = SelectionModeFlags(0x001E); // 0x0002 | 0x0004 | 0x0008 | 0x0010

    /// All entity types (Element | Node)
    pub const ALL: Self = SelectionModeFlags(0x001F); // 0x001E | 0x0001

    /// Group selection mode
    pub const GROUP: Self = SelectionModeFlags(0x0100);

    /// Creates a new SelectionModeFlags from a raw i32 value
    pub fn new(value: i32) -> Self {
        SelectionModeFlags(value)
    }

    /// Returns the raw i32 value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Checks if this flag contains the given flag
    pub fn contains(&self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
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

impl std::ops::BitOr for SelectionModeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        SelectionModeFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for SelectionModeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        SelectionModeFlags(self.0 & rhs.0)
    }
}

impl From<i32> for SelectionModeFlags {
    fn from(value: i32) -> Self {
        SelectionModeFlags(value)
    }
}

impl From<SelectionModeFlags> for i32 {
    fn from(flags: SelectionModeFlags) -> Self {
        flags.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_values() {
        assert_eq!(SelectionModeFlags::MESH.value(), 0x0000);
        assert_eq!(SelectionModeFlags::NODE.value(), 0x0001);
        assert_eq!(SelectionModeFlags::ELEMENT_0D.value(), 0x0002);
        assert_eq!(SelectionModeFlags::LINK.value(), 0x0004);
        assert_eq!(SelectionModeFlags::FACE.value(), 0x0008);
        assert_eq!(SelectionModeFlags::VOLUME.value(), 0x0010);
        assert_eq!(SelectionModeFlags::GROUP.value(), 0x0100);
    }

    #[test]
    fn test_element_composition() {
        let element = SelectionModeFlags::ELEMENT_0D | SelectionModeFlags::LINK
            | SelectionModeFlags::FACE | SelectionModeFlags::VOLUME;
        assert_eq!(element.value(), SelectionModeFlags::ELEMENT.value());
        assert!(element.contains(SelectionModeFlags::ELEMENT_0D));
        assert!(element.contains(SelectionModeFlags::LINK));
        assert!(element.contains(SelectionModeFlags::FACE));
        assert!(element.contains(SelectionModeFlags::VOLUME));
    }

    #[test]
    fn test_all_composition() {
        let all = SelectionModeFlags::ELEMENT | SelectionModeFlags::NODE;
        assert_eq!(all.value(), SelectionModeFlags::ALL.value());
        assert!(all.contains(SelectionModeFlags::NODE));
        assert!(all.contains(SelectionModeFlags::ELEMENT_0D));
        assert!(all.contains(SelectionModeFlags::LINK));
        assert!(all.contains(SelectionModeFlags::FACE));
        assert!(all.contains(SelectionModeFlags::VOLUME));
    }

    #[test]
    fn test_contains() {
        let flags = SelectionModeFlags::NODE | SelectionModeFlags::FACE | SelectionModeFlags::GROUP;
        assert!(flags.contains(SelectionModeFlags::NODE));
        assert!(flags.contains(SelectionModeFlags::FACE));
        assert!(flags.contains(SelectionModeFlags::GROUP));
        assert!(!flags.contains(SelectionModeFlags::LINK));
    }

    #[test]
    fn test_set_clear() {
        let mut flags = SelectionModeFlags::MESH;
        flags.set(SelectionModeFlags::NODE);
        assert!(flags.contains(SelectionModeFlags::NODE));

        flags.set(SelectionModeFlags::FACE);
        assert!(flags.contains(SelectionModeFlags::NODE));
        assert!(flags.contains(SelectionModeFlags::FACE));

        flags.clear(SelectionModeFlags::NODE);
        assert!(!flags.contains(SelectionModeFlags::NODE));
        assert!(flags.contains(SelectionModeFlags::FACE));
    }

    #[test]
    fn test_bitwise_operations() {
        let f1 = SelectionModeFlags::NODE;
        let f2 = SelectionModeFlags::FACE;
        let combined = f1 | f2;
        assert!(combined.contains(SelectionModeFlags::NODE));
        assert!(combined.contains(SelectionModeFlags::FACE));

        let masked = combined & SelectionModeFlags::NODE;
        assert_eq!(masked.value(), SelectionModeFlags::NODE.value());
    }

    #[test]
    fn test_from_i32() {
        let flags: SelectionModeFlags = 0x0001i32.into();
        assert_eq!(flags.value(), 0x0001);
        assert!(flags.contains(SelectionModeFlags::NODE));
    }
}
