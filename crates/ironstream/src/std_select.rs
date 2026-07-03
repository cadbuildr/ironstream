// FILE: std_select.rs
// occt: StdSelect

//! Root namespace for standard selection package.
//! Contains utilities and constants for object selection.

pub const MAX_SELECTION_DEPTH: i32 = 1000;

pub struct StdSelect;

impl StdSelect {
    pub fn description() -> &'static str {
        "Standard selection tools and utilities"
    }

    pub fn max_depth() -> i32 {
        MAX_SELECTION_DEPTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_select_namespace() {
        assert!(!StdSelect::description().is_empty());
        assert_eq!(StdSelect::max_depth(), MAX_SELECTION_DEPTH);
    }
}
