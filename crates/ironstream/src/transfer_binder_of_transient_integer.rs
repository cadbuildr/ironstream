// FILE: transfer_binder_of_transient_integer.rs
// occt: Transfer_BinderOfTransientInteger

/// A binder that associates a transient object with an integer result.
/// Maps transient entities to integer values.
#[derive(Clone, Debug)]
pub struct TransferBinderOfTransientInteger {
    /// The integer result value
    result: i32,
    /// Whether a result has been set
    has_result: bool,
}

impl TransferBinderOfTransientInteger {
    /// Creates a new empty binder.
    pub fn new() -> Self {
        Self {
            result: 0,
            has_result: false,
        }
    }

    /// Creates a binder with an initial integer result.
    pub fn with_result(result: i32) -> Self {
        Self {
            result,
            has_result: true,
        }
    }

    /// Sets the integer result.
    pub fn set_result(&mut self, result: i32) {
        self.result = result;
        self.has_result = true;
    }

    /// Returns the integer result if set.
    pub fn result(&self) -> Option<i32> {
        if self.has_result {
            Some(self.result)
        } else {
            None
        }
    }

    /// Returns whether a result has been set.
    pub fn has_result(&self) -> bool {
        self.has_result
    }

    /// Clears the result.
    pub fn clear(&mut self) {
        self.result = 0;
        self.has_result = false;
    }
}

impl Default for TransferBinderOfTransientInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let binder = TransferBinderOfTransientInteger::new();
        assert!(!binder.has_result());
        assert_eq!(binder.result(), None);
    }

    #[test]
    fn test_with_result() {
        let binder = TransferBinderOfTransientInteger::with_result(42);
        assert!(binder.has_result());
        assert_eq!(binder.result(), Some(42));
    }

    #[test]
    fn test_set_result() {
        let mut binder = TransferBinderOfTransientInteger::new();
        binder.set_result(100);
        assert!(binder.has_result());
        assert_eq!(binder.result(), Some(100));

        binder.set_result(-50);
        assert_eq!(binder.result(), Some(-50));
    }

    #[test]
    fn test_clear() {
        let mut binder = TransferBinderOfTransientInteger::with_result(99);
        assert!(binder.has_result());

        binder.clear();
        assert!(!binder.has_result());
        assert_eq!(binder.result(), None);
    }
}
