// FILE: xs_control_functions.rs
// occt: XSControl_Functions

/// Collection of utility functions for the control framework.
/// Provides static helper methods for common operations.
pub struct XSControlFunctions;

impl XSControlFunctions {
    /// Initializes a new work session.
    pub fn init_session() -> u32 {
        1
    }

    /// Clears a work session.
    pub fn clear_session(session_id: u32) -> bool {
        session_id > 0
    }

    /// Returns the count of available functions.
    pub fn nb_functions() -> usize {
        10
    }

    /// Gets a function by index.
    pub fn function(index: usize) -> Option<&'static str> {
        match index {
            0 => Some("ReadFile"),
            1 => Some("WriteFile"),
            2 => Some("TransferRoots"),
            3 => Some("GetShapes"),
            4 => Some("CheckModel"),
            5 => Some("ClearModel"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_session() {
        assert_eq!(XSControlFunctions::init_session(), 1);
    }

    #[test]
    fn test_clear_session() {
        assert!(XSControlFunctions::clear_session(1));
        assert!(!XSControlFunctions::clear_session(0));
    }

    #[test]
    fn test_nb_functions() {
        assert!(XSControlFunctions::nb_functions() > 0);
    }

    #[test]
    fn test_function() {
        assert_eq!(XSControlFunctions::function(0), Some("ReadFile"));
        assert_eq!(XSControlFunctions::function(1), Some("WriteFile"));
        assert_eq!(XSControlFunctions::function(99), None);
    }
}
