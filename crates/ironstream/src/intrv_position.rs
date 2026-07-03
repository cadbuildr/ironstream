// FILE: intrv_position.rs
// occt: Intrv_Position

/// Describes the position of one interval relative to another.
/// Enumerates 13 position types reflecting overlap and adjacency relationships.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntrvPosition {
    /// This interval ends before the other starts
    Before = 0,
    /// This interval just touches the start of the other
    JustBefore = 1,
    /// This interval overlaps the start of the other
    OverlappingAtStart = 2,
    /// This interval encloses the other with matching ends
    JustEnclosingAtEnd = 3,
    /// This interval completely contains the other
    Enclosing = 4,
    /// This interval just touches within the other at start
    JustOverlappingAtStart = 5,
    /// This interval has same bounds as the other
    Similar = 6,
    /// This interval encloses the other with matching start
    JustEnclosingAtStart = 7,
    /// This interval is completely inside the other
    Inside = 8,
    /// This interval just touches within the other at end
    JustOverlappingAtEnd = 9,
    /// This interval overlaps the end of the other
    OverlappingAtEnd = 10,
    /// This interval just touches the end of the other
    JustAfter = 11,
    /// This interval starts after the other ends
    After = 12,
}

impl Default for IntrvPosition {
    fn default() -> Self {
        IntrvPosition::Before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_variants() {
        assert_eq!(IntrvPosition::Before as i32, 0);
        assert_eq!(IntrvPosition::Enclosing as i32, 4);
        assert_eq!(IntrvPosition::After as i32, 12);
    }

    #[test]
    fn test_position_debug() {
        assert_eq!(format!("{:?}", IntrvPosition::Similar), "Similar");
    }

    #[test]
    fn test_position_default() {
        assert_eq!(IntrvPosition::default(), IntrvPosition::Before);
    }
}
