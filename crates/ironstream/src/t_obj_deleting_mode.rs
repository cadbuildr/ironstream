// FILE: t_obj_deleting_mode.rs
// occt: TObj_DeletingMode

//! Faithful port of the `TObj_DeletingMode` enumeration used by
//! TObj_Object::Detach and TObj_Model to control object removal.

/// Deleting mode of TObj objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TObjDeletingMode {
    /// Delete objects only without dependence.
    FreeOnly = 0,
    /// Remove object if depending one will be correct elsewhere.
    KeepDepending = 1,
    /// Delete this object and all depending objects.
    Forced = 2,
}

impl TObjDeletingMode {
    /// Whether this mode allows removing an object that other objects
    /// still reference (the check performed by TObj_Object::CanDetach).
    pub fn allows_removal_with_dependents(&self) -> bool {
        match self {
            TObjDeletingMode::FreeOnly => false,
            TObjDeletingMode::KeepDepending => true,
            TObjDeletingMode::Forced => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_values_match_occt() {
        assert_eq!(TObjDeletingMode::FreeOnly as i32, 0);
        assert_eq!(TObjDeletingMode::KeepDepending as i32, 1);
        assert_eq!(TObjDeletingMode::Forced as i32, 2);
    }

    #[test]
    fn free_only_is_strictest() {
        assert!(!TObjDeletingMode::FreeOnly.allows_removal_with_dependents());
        assert!(TObjDeletingMode::KeepDepending.allows_removal_with_dependents());
        assert!(TObjDeletingMode::Forced.allows_removal_with_dependents());
    }
}
