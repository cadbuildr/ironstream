// FILE: graphic3d_display_priority.rs
// occt: Graphic3d_DisplayPriority

//! Structure priority - range (do not change this range!).
//! Values are between 0 and 10, with 5 used by default.
//! A structure of priority 10 is displayed the last and appears over the others (considering depth test).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum DisplayPriority {
    /// Invalid priority
    Invalid = -1,
    /// Bottom (0)
    Bottom = 0,
    /// Almost bottom (1)
    AlmostBottom = 1,
    /// Below 2 (2)
    Below2 = 2,
    /// Below 1 (3)
    Below1 = 3,
    /// Below (4)
    Below = 4,
    /// Normal (5) - default
    Normal = 5,
    /// Above (6)
    Above = 6,
    /// Above 1 (7)
    Above1 = 7,
    /// Above 2 (8)
    Above2 = 8,
    /// Highlight (9)
    Highlight = 9,
    /// Topmost (10)
    Topmost = 10,
}

pub const DISPLAY_PRIORITY_NB: usize = 12; // Invalid(-1) through Topmost(10) = 12 values

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_priority_values() {
        assert_eq!(DisplayPriority::Invalid as i32, -1);
        assert_eq!(DisplayPriority::Bottom as i32, 0);
        assert_eq!(DisplayPriority::AlmostBottom as i32, 1);
        assert_eq!(DisplayPriority::Below2 as i32, 2);
        assert_eq!(DisplayPriority::Below1 as i32, 3);
        assert_eq!(DisplayPriority::Below as i32, 4);
        assert_eq!(DisplayPriority::Normal as i32, 5);
        assert_eq!(DisplayPriority::Above as i32, 6);
        assert_eq!(DisplayPriority::Above1 as i32, 7);
        assert_eq!(DisplayPriority::Above2 as i32, 8);
        assert_eq!(DisplayPriority::Highlight as i32, 9);
        assert_eq!(DisplayPriority::Topmost as i32, 10);
    }

    #[test]
    fn test_display_priority_ordering() {
        // Test that priorities can be compared
        assert!(DisplayPriority::Bottom < DisplayPriority::Normal);
        assert!(DisplayPriority::Normal < DisplayPriority::Topmost);
        assert!(DisplayPriority::Invalid < DisplayPriority::Bottom);
    }

    #[test]
    fn test_display_priority_nb() {
        // DISPLAY_PRIORITY_NB represents total count (from Invalid=-1 to Topmost=10)
        assert_eq!(DISPLAY_PRIORITY_NB, 12);
    }
}
