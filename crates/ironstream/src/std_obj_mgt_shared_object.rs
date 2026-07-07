// FILE: std_obj_mgt_shared_object.rs
// occt: StdObjMgt_SharedObject

use std::any::Any;

/// Template helper for shared object persistence.
/// Manages delayed initialization and import of transient objects.
pub struct StdObjMgtSharedObject;

/// Abstract persistent base for shared objects
pub trait AbstractPersistentBase: Any {
    /// Import the transient object
    fn import(&self) -> Option<Box<dyn Any>>;
}

/// Shared base implementation storing a transient object
pub struct SharedBase<T> {
    transient: Option<T>,
}

impl<T> SharedBase<T> {
    /// Create a new shared base
    pub fn new() -> Self {
        SharedBase {
            transient: None,
        }
    }

    /// Set the transient object
    pub fn set_transient(&mut self, transient: T) {
        self.transient = Some(transient);
    }

    /// Get the transient object
    pub fn transient(&self) -> Option<&T> {
        self.transient.as_ref()
    }

    /// Get mutable access to transient object
    pub fn transient_mut(&mut self) -> Option<&mut T> {
        self.transient.as_mut()
    }

    /// Import (consume) the transient object
    pub fn import(&mut self) -> Option<T> {
        self.transient.take()
    }
}

impl<T> Default for SharedBase<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Delayed import base for deferred object initialization
pub struct DelayedBase<T> {
    transient: Option<T>,
}

impl<T> DelayedBase<T> {
    /// Create a new delayed base
    pub fn new() -> Self {
        DelayedBase {
            transient: None,
        }
    }

    /// Set the transient object
    pub fn set_transient(&mut self, transient: T) {
        self.transient = Some(transient);
    }

    /// Import the transient object
    pub fn import(&mut self) -> Option<T> {
        self.transient.take()
    }

    /// Get the transient object without consuming it
    pub fn transient(&self) -> Option<&T> {
        self.transient.as_ref()
    }
}

impl<T> Default for DelayedBase<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Ignore data variant - reads/writes data but doesn't store it
pub struct IgnoreData<T> {
    transient: Option<T>,
}

impl<T> IgnoreData<T> {
    /// Create a new ignore data handler
    pub fn new() -> Self {
        IgnoreData {
            transient: None,
        }
    }

    /// Read and discard persistent data
    pub fn read(&mut self) {
        // Data is read but not stored
    }

    /// Write null data
    pub fn write(&self) {
        // Nothing to write
    }

    /// Get persistent children (none)
    pub fn p_children(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "IgnoreData"
    }

    /// Import returns null
    pub fn import(&mut self) -> Option<T> {
        None
    }
}

impl<T> Default for IgnoreData<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Delayed sub-base for managing transient/persistent lifecycle
pub struct DelayedSubBase<T, P> {
    transient: Option<T>,
    persistent: Option<P>,
}

impl<T, P> DelayedSubBase<T, P> {
    /// Create a new delayed sub-base
    pub fn new() -> Self {
        DelayedSubBase {
            transient: None,
            persistent: None,
        }
    }

    /// Set the persistent object
    pub fn set_persistent(&mut self, persistent: P) {
        self.persistent = Some(persistent);
    }

    /// Set the transient object
    pub fn set_transient(&mut self, transient: T) {
        self.transient = Some(transient);
    }

    /// Import with lazy loading of transient from persistent
    pub fn import(&mut self) -> Option<&T> {
        if self.transient.is_none() && self.persistent.is_some() {
            // In a real implementation, we would convert persistent to transient
            // For now, we just clear the persistent after attempted import
            self.persistent = None;
        }
        self.transient.as_ref()
    }

    /// Get the transient object
    pub fn transient(&self) -> Option<&T> {
        self.transient.as_ref()
    }
}

impl<T, P> Default for DelayedSubBase<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_base_new() {
        let base: SharedBase<i32> = SharedBase::new();
        assert!(base.transient().is_none());
    }

    #[test]
    fn test_shared_base_set_get() {
        let mut base: SharedBase<i32> = SharedBase::new();
        base.set_transient(42);

        assert_eq!(base.transient(), Some(&42));
    }

    #[test]
    fn test_shared_base_import() {
        let mut base: SharedBase<i32> = SharedBase::new();
        base.set_transient(42);

        let imported = base.import();
        assert_eq!(imported, Some(42));
        assert!(base.transient().is_none());
    }

    #[test]
    fn test_delayed_base_new() {
        let base: DelayedBase<i32> = DelayedBase::new();
        assert!(base.transient().is_none());
    }

    #[test]
    fn test_delayed_base_set_import() {
        let mut base: DelayedBase<i32> = DelayedBase::new();
        base.set_transient(99);

        let imported = base.import();
        assert_eq!(imported, Some(99));
        assert!(base.transient().is_none());
    }

    #[test]
    fn test_ignore_data_new() {
        let data: IgnoreData<i32> = IgnoreData::new();
        assert_eq!(data.p_name(), "IgnoreData");
        assert!(data.p_children().is_empty());
    }

    #[test]
    fn test_ignore_data_import() {
        let mut data: IgnoreData<i32> = IgnoreData::new();
        let imported = data.import();
        assert!(imported.is_none());
    }

    #[test]
    fn test_delayed_sub_base_new() {
        let base: DelayedSubBase<i32, String> = DelayedSubBase::new();
        assert!(base.transient().is_none());
    }

    #[test]
    fn test_delayed_sub_base_set() {
        let mut base: DelayedSubBase<i32, String> = DelayedSubBase::new();
        base.set_transient(42);
        base.set_persistent("persistent_data".to_string());

        assert_eq!(base.transient(), Some(&42));
    }

    #[test]
    fn test_delayed_sub_base_import() {
        let mut base: DelayedSubBase<i32, String> = DelayedSubBase::new();
        base.set_transient(42);

        let imported = base.import();
        assert_eq!(imported, Some(&42));
    }
}
