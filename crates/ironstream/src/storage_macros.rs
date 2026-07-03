// FILE: storage_macros.rs
// occt: Storage_Macros

//! Macros for Storage schema handling (C++ preprocessor definitions ported to documentation)
//! In OCCT, these are preprocessor macros for schema method declaration and selection handling.
//! In Rust, we provide the patterns as documentation since Rust doesn't have direct equivalents.

/// Declares standard schema methods for a struct
/// Maps to Storage_DECLARE_SCHEMA_METHODS in OCCT
pub const DECLARE_SCHEMA_METHODS: &str = "Schema method declarations";

/// Begin read selection mapping
pub const BEGIN_READ_SELECTION: &str = "Start read selection block";

/// Map a read selection for a class/callback pair
pub const READ_SELECTION: &str = "Map class to callback";

/// End read selection block
pub const END_READ_SELECTION: &str = "End read selection";

/// Begin schema types listing
pub const BEGIN_SCHEMA_TYPES: &str = "Start schema types";

/// Add a type to schema
pub const SCHEMA_TYPES: &str = "Add type to schema";

/// End schema types listing
pub const END_SCHEMA_TYPES: &str = "End schema types";

/// Begin add types block
pub const BEGIN_ADD_TYPES: &str = "Start add types block";

/// Add a type/callback mapping
pub const ADD_TYPES: &str = "Add type/callback mapping";

/// End add types block
pub const END_ADD_TYPES: &str = "End add types block";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_exist() {
        assert!(!DECLARE_SCHEMA_METHODS.is_empty());
        assert!(!BEGIN_READ_SELECTION.is_empty());
        assert!(!READ_SELECTION.is_empty());
        assert!(!END_READ_SELECTION.is_empty());
    }
}
