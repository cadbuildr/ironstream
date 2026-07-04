// FILE: t_data_std_current.rs
// occt: TDataStd_Current

/// Attribute marking the current label in a tree.
pub struct TDataStdCurrent;

impl TDataStdCurrent {
    /// Creates a new current marker.
    pub fn new() -> Self {
        TDataStdCurrent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        let _current = TDataStdCurrent::new();
    }
}
