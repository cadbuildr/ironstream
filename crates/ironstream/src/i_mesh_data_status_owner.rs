// FILE: i_mesh_data_status_owner.rs
// occt: IMeshData_StatusOwner

/// Extension interface class providing status functionality.
pub struct IMeshDataStatusOwner {
    status: i32,
}

impl IMeshDataStatusOwner {
    /// Create with default status (no error).
    pub fn new() -> Self {
        IMeshDataStatusOwner { status: 0 }
    }

    /// Returns true if status is strictly equal to the given value.
    pub fn is_equal(&self, value: i32) -> bool {
        self.status == value
    }

    /// Returns true if status bit is set.
    pub fn is_set(&self, value: i32) -> bool {
        (self.status & value) != 0
    }

    /// Add status to status flags.
    pub fn set_status(&mut self, value: i32) {
        self.status |= value;
    }

    /// Remove status from status flags.
    pub fn unset_status(&mut self, value: i32) {
        self.status &= !value;
    }

    /// Returns complete status mask.
    pub fn get_status_mask(&self) -> i32 {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let owner = IMeshDataStatusOwner::new();
        assert_eq!(owner.get_status_mask(), 0);
    }

    #[test]
    fn test_set_status() {
        let mut owner = IMeshDataStatusOwner::new();
        owner.set_status(0x01);
        assert!(owner.is_set(0x01));
        assert!(owner.is_equal(0x01));
    }

    #[test]
    fn test_unset_status() {
        let mut owner = IMeshDataStatusOwner::new();
        owner.set_status(0x03);
        owner.unset_status(0x01);
        assert!(!owner.is_set(0x01));
        assert!(owner.is_set(0x02));
        assert_eq!(owner.get_status_mask(), 0x02);
    }
}
