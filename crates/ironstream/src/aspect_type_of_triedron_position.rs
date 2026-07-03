// FILE: aspect_type_of_triedron_position.rs
// occt: Aspect_TypeOfTriedronPosition

/// Defines the position of a trihedron (triedron) within a view.
///
/// This is represented as a bitmask to allow independent specification
/// of vertical and horizontal alignment. Positions can be combined
/// using bitwise OR operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AspectTypeOfTriedronPosition(u16);

impl AspectTypeOfTriedronPosition {
    /// At the center of the view.
    pub const CENTER: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0000);

    /// At the middle of the top side.
    pub const TOP: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0001);

    /// At the middle of the bottom side.
    pub const BOTTOM: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0002);

    /// At the middle of the left side.
    pub const LEFT: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0004);

    /// At the middle of the right side.
    pub const RIGHT: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0008);

    /// At the left lower corner (BOTTOM | LEFT).
    pub const LEFT_LOWER: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0006);

    /// At the left upper corner (TOP | LEFT).
    pub const LEFT_UPPER: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0005);

    /// At the right lower corner (BOTTOM | RIGHT).
    pub const RIGHT_LOWER: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x000a);

    /// At the right upper corner (TOP | RIGHT).
    pub const RIGHT_UPPER: AspectTypeOfTriedronPosition = AspectTypeOfTriedronPosition(0x0009);

    /// Creates a new position from a raw u16 bitmask value.
    pub fn new(value: u16) -> Self {
        AspectTypeOfTriedronPosition(value)
    }

    /// Returns the raw bitmask value.
    pub fn value(&self) -> u16 {
        self.0
    }

    /// Combines this position with another using bitwise OR.
    pub fn combined(&self, other: AspectTypeOfTriedronPosition) -> AspectTypeOfTriedronPosition {
        AspectTypeOfTriedronPosition(self.0 | other.0)
    }

    /// Checks if this position is at the center.
    pub fn is_center(&self) -> bool {
        self.0 == 0x0000
    }

    /// Checks if this position includes the top flag.
    pub fn has_top(&self) -> bool {
        (self.0 & 0x0001) != 0
    }

    /// Checks if this position includes the bottom flag.
    pub fn has_bottom(&self) -> bool {
        (self.0 & 0x0002) != 0
    }

    /// Checks if this position includes the left flag.
    pub fn has_left(&self) -> bool {
        (self.0 & 0x0004) != 0
    }

    /// Checks if this position includes the right flag.
    pub fn has_right(&self) -> bool {
        (self.0 & 0x0008) != 0
    }
}

impl Default for AspectTypeOfTriedronPosition {
    fn default() -> Self {
        AspectTypeOfTriedronPosition::CENTER
    }
}

impl From<u16> for AspectTypeOfTriedronPosition {
    fn from(value: u16) -> Self {
        AspectTypeOfTriedronPosition(value)
    }
}

impl From<AspectTypeOfTriedronPosition> for u16 {
    fn from(pos: AspectTypeOfTriedronPosition) -> Self {
        pos.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_values() {
        assert_eq!(AspectTypeOfTriedronPosition::CENTER.value(), 0x0000);
        assert_eq!(AspectTypeOfTriedronPosition::TOP.value(), 0x0001);
        assert_eq!(AspectTypeOfTriedronPosition::BOTTOM.value(), 0x0002);
        assert_eq!(AspectTypeOfTriedronPosition::LEFT.value(), 0x0004);
        assert_eq!(AspectTypeOfTriedronPosition::RIGHT.value(), 0x0008);
    }

    #[test]
    fn test_corner_positions() {
        assert_eq!(AspectTypeOfTriedronPosition::LEFT_LOWER.value(), 0x0006);
        assert_eq!(AspectTypeOfTriedronPosition::LEFT_UPPER.value(), 0x0005);
        assert_eq!(AspectTypeOfTriedronPosition::RIGHT_LOWER.value(), 0x000a);
        assert_eq!(AspectTypeOfTriedronPosition::RIGHT_UPPER.value(), 0x0009);
    }

    #[test]
    fn test_is_center() {
        assert!(AspectTypeOfTriedronPosition::CENTER.is_center());
        assert!(!AspectTypeOfTriedronPosition::TOP.is_center());
        assert!(!AspectTypeOfTriedronPosition::LEFT_UPPER.is_center());
    }

    #[test]
    fn test_has_flags() {
        let left_upper = AspectTypeOfTriedronPosition::LEFT_UPPER;
        assert!(left_upper.has_top());
        assert!(left_upper.has_left());
        assert!(!left_upper.has_bottom());
        assert!(!left_upper.has_right());
    }

    #[test]
    fn test_has_flags_right_lower() {
        let right_lower = AspectTypeOfTriedronPosition::RIGHT_LOWER;
        assert!(right_lower.has_bottom());
        assert!(right_lower.has_right());
        assert!(!right_lower.has_top());
        assert!(!right_lower.has_left());
    }

    #[test]
    fn test_combined_operation() {
        let top = AspectTypeOfTriedronPosition::TOP;
        let left = AspectTypeOfTriedronPosition::LEFT;
        let combined = top.combined(left);
        assert_eq!(combined.value(), 0x0005);
        assert_eq!(combined, AspectTypeOfTriedronPosition::LEFT_UPPER);
    }

    #[test]
    fn test_default() {
        assert_eq!(AspectTypeOfTriedronPosition::default(), AspectTypeOfTriedronPosition::CENTER);
    }

    #[test]
    fn test_from_u16() {
        let pos = AspectTypeOfTriedronPosition::from(0x0006);
        assert_eq!(pos, AspectTypeOfTriedronPosition::LEFT_LOWER);
    }

    #[test]
    fn test_to_u16() {
        let pos = AspectTypeOfTriedronPosition::LEFT_UPPER;
        let value: u16 = pos.into();
        assert_eq!(value, 0x0005);
    }

    #[test]
    fn test_equality() {
        let pos1 = AspectTypeOfTriedronPosition::new(0x0001);
        let pos2 = AspectTypeOfTriedronPosition::TOP;
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_clone_and_copy() {
        let original = AspectTypeOfTriedronPosition::RIGHT_UPPER;
        let cloned = original.clone();
        let copied = original;
        assert_eq!(cloned, original);
        assert_eq!(copied, original);
    }

    #[test]
    fn test_center_has_no_flags() {
        let center = AspectTypeOfTriedronPosition::CENTER;
        assert!(!center.has_top());
        assert!(!center.has_bottom());
        assert!(!center.has_left());
        assert!(!center.has_right());
    }
}
