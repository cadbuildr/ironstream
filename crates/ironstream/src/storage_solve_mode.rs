// FILE: storage_solve_mode.rs
// occt: Storage_SolveMode

//! Enumeration for storage solve modes
//! Maps to `enum Storage_SolveMode` in OCCT

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StorageSolveMode {
    /// Add solve mode
    AddSolve = 0,
    /// Write solve mode
    WriteSolve = 1,
    /// Read solve mode
    ReadSolve = 2,
}

impl StorageSolveMode {
    /// Convert from integer representation
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(StorageSolveMode::AddSolve),
            1 => Some(StorageSolveMode::WriteSolve),
            2 => Some(StorageSolveMode::ReadSolve),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_mode_values() {
        assert_eq!(StorageSolveMode::AddSolve as i32, 0);
        assert_eq!(StorageSolveMode::WriteSolve as i32, 1);
        assert_eq!(StorageSolveMode::ReadSolve as i32, 2);
    }

    #[test]
    fn test_solve_mode_from_int() {
        assert_eq!(StorageSolveMode::from_int(0), Some(StorageSolveMode::AddSolve));
        assert_eq!(StorageSolveMode::from_int(1), Some(StorageSolveMode::WriteSolve));
        assert_eq!(StorageSolveMode::from_int(2), Some(StorageSolveMode::ReadSolve));
        assert_eq!(StorageSolveMode::from_int(3), None);
    }

    #[test]
    fn test_solve_mode_ordering() {
        assert!(StorageSolveMode::AddSolve < StorageSolveMode::WriteSolve);
        assert!(StorageSolveMode::WriteSolve < StorageSolveMode::ReadSolve);
    }

    #[test]
    fn test_solve_mode_clone_copy() {
        let mode = StorageSolveMode::WriteSolve;
        let mode_copy = mode;
        assert_eq!(mode, mode_copy);
    }
}
