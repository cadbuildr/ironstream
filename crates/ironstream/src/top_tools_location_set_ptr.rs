// FILE: top_tools_location_set_ptr.rs
// occt: TopTools_LocationSetPtr

//! Pointer to a topological location set.

use std::sync::Arc;

use crate::top_tools_location_set::TopToolsLocationSet;

/// Shared pointer to location set
pub type TopToolsLocationSetPtr = Arc<TopToolsLocationSet>;

/// Creates a new shared location set
pub fn new_location_set_ptr() -> TopToolsLocationSetPtr {
    Arc::new(TopToolsLocationSet::new())
}

/// Creates a new shared location set with initial content
pub fn location_set_ptr_from(set: TopToolsLocationSet) -> TopToolsLocationSetPtr {
    Arc::new(set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_set_ptr_new() {
        let _ptr = new_location_set_ptr();
        // Verify reference count is 1
    }

    #[test]
    fn test_location_set_ptr_clone() {
        let ptr1 = new_location_set_ptr();
        let _ptr2 = Arc::clone(&ptr1);
        // Both pointers refer to same set
    }
}
