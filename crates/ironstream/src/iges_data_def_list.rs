// FILE: iges_data_def_list.rs
// occt: IGESData_DefList

//! Enumeration for the definition state of IGES entity list fields.
//!
//! Some fields of an IGES entity may be:
//! - Undefined (not present at all)
//! - Defined as a single item
//! - Defined as a list of multiple items
//!
//! A typical example is a level number which can be absent, single, or multiple.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefList {
    /// The list is empty (there is not even a single item)
    None,
    /// The list contains a single item
    One,
    /// The list contains several items
    Several,
    /// The list contains one item, but this item is incorrect
    ErrorOne,
    /// The list contains several items, but at least one is incorrect
    ErrorSeveral,
}

impl DefList {
    /// Check if this is an error state
    pub fn is_error(self) -> bool {
        matches!(self, DefList::ErrorOne | DefList::ErrorSeveral)
    }

    /// Check if this contains exactly one item (including error)
    pub fn has_one(self) -> bool {
        matches!(self, DefList::One | DefList::ErrorOne)
    }

    /// Check if this contains multiple items (including error)
    pub fn has_several(self) -> bool {
        matches!(self, DefList::Several | DefList::ErrorSeveral)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def_list_variants() {
        assert_eq!(DefList::None, DefList::None);
        assert_eq!(DefList::One, DefList::One);
        assert_eq!(DefList::Several, DefList::Several);
        assert_eq!(DefList::ErrorOne, DefList::ErrorOne);
        assert_eq!(DefList::ErrorSeveral, DefList::ErrorSeveral);
    }

    #[test]
    fn test_is_error() {
        assert!(!DefList::None.is_error());
        assert!(!DefList::One.is_error());
        assert!(!DefList::Several.is_error());
        assert!(DefList::ErrorOne.is_error());
        assert!(DefList::ErrorSeveral.is_error());
    }

    #[test]
    fn test_has_one() {
        assert!(!DefList::None.has_one());
        assert!(DefList::One.has_one());
        assert!(!DefList::Several.has_one());
        assert!(DefList::ErrorOne.has_one());
        assert!(!DefList::ErrorSeveral.has_one());
    }

    #[test]
    fn test_has_several() {
        assert!(!DefList::None.has_several());
        assert!(!DefList::One.has_several());
        assert!(DefList::Several.has_several());
        assert!(!DefList::ErrorOne.has_several());
        assert!(DefList::ErrorSeveral.has_several());
    }
}
