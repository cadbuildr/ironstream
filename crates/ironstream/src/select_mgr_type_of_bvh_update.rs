// FILE: select_mgr_type_of_bvh_update.rs
// occt: SelectMgr_TypeOfBVHUpdate

/// Enumeration for BVH update types in selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfBvhUpdate {
    /// Sensitives must be added to 2nd level BVH.
    Add,
    /// All sensitives must be removed from 2nd level BVH.
    Remove,
    /// Sensitives must be removed and added after recomputation.
    Renew,
    /// 2nd level BVH needs to be rebuilt.
    Invalidate,
    /// Sensitives are up to date.
    None,
}

impl TypeOfBvhUpdate {
    pub fn is_add(&self) -> bool {
        *self == TypeOfBvhUpdate::Add
    }

    pub fn is_remove(&self) -> bool {
        *self == TypeOfBvhUpdate::Remove
    }

    pub fn is_renew(&self) -> bool {
        *self == TypeOfBvhUpdate::Renew
    }

    pub fn is_invalidate(&self) -> bool {
        *self == TypeOfBvhUpdate::Invalidate
    }

    pub fn is_none(&self) -> bool {
        *self == TypeOfBvhUpdate::None
    }

    pub fn as_str(&self) -> &str {
        match self {
            TypeOfBvhUpdate::Add => "Add",
            TypeOfBvhUpdate::Remove => "Remove",
            TypeOfBvhUpdate::Renew => "Renew",
            TypeOfBvhUpdate::Invalidate => "Invalidate",
            TypeOfBvhUpdate::None => "None",
        }
    }
}

impl Default for TypeOfBvhUpdate {
    fn default() -> Self {
        TypeOfBvhUpdate::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_comparison() {
        assert_eq!(TypeOfBvhUpdate::Add, TypeOfBvhUpdate::Add);
        assert_ne!(TypeOfBvhUpdate::Add, TypeOfBvhUpdate::Remove);
    }

    #[test]
    fn type_checks() {
        assert!(TypeOfBvhUpdate::Add.is_add());
        assert!(TypeOfBvhUpdate::Remove.is_remove());
        assert!(TypeOfBvhUpdate::Renew.is_renew());
        assert!(TypeOfBvhUpdate::Invalidate.is_invalidate());
        assert!(TypeOfBvhUpdate::None.is_none());
    }

    #[test]
    fn as_str() {
        assert_eq!(TypeOfBvhUpdate::Add.as_str(), "Add");
        assert_eq!(TypeOfBvhUpdate::Remove.as_str(), "Remove");
        assert_eq!(TypeOfBvhUpdate::Invalidate.as_str(), "Invalidate");
    }

    #[test]
    fn default() {
        assert_eq!(TypeOfBvhUpdate::default(), TypeOfBvhUpdate::None);
    }
}
