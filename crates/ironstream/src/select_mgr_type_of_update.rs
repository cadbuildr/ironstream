// FILE: select_mgr_type_of_update.rs
// occt: SelectMgr_TypeOfUpdate

/// Enumeration for types of selection updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfUpdate {
    /// Selection primitive is updated independently.
    Incremental,
    /// Selection requires a complete rebuild.
    Full,
}

impl TypeOfUpdate {
    pub fn is_incremental(&self) -> bool {
        *self == TypeOfUpdate::Incremental
    }

    pub fn is_full(&self) -> bool {
        *self == TypeOfUpdate::Full
    }

    pub fn as_str(&self) -> &str {
        match self {
            TypeOfUpdate::Incremental => "Incremental",
            TypeOfUpdate::Full => "Full",
        }
    }
}

impl Default for TypeOfUpdate {
    fn default() -> Self {
        TypeOfUpdate::Incremental
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_comparison() {
        assert_eq!(TypeOfUpdate::Incremental, TypeOfUpdate::Incremental);
        assert_ne!(TypeOfUpdate::Incremental, TypeOfUpdate::Full);
    }

    #[test]
    fn type_checks() {
        assert!(TypeOfUpdate::Incremental.is_incremental());
        assert!(TypeOfUpdate::Full.is_full());
    }

    #[test]
    fn as_str() {
        assert_eq!(TypeOfUpdate::Incremental.as_str(), "Incremental");
        assert_eq!(TypeOfUpdate::Full.as_str(), "Full");
    }

    #[test]
    fn default() {
        assert_eq!(TypeOfUpdate::default(), TypeOfUpdate::Incremental);
    }
}
