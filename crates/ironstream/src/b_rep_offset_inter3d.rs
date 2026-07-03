// FILE: b_rep_offset_inter3d.rs
// occt: BRepOffset_Inter3d

/// Computes the connection of the offset and non-offset faces.
/// Stores result in AsDes tool according to the required connection type.
#[derive(Debug, Clone)]
pub struct BRepOffsetInter3d {
    /// Number of face connections found
    connection_count: usize,
}

impl BRepOffsetInter3d {
    /// Create a new Inter3d
    pub fn new() -> Self {
        Self {
            connection_count: 0,
        }
    }

    /// Get number of connections
    pub fn connection_count(&self) -> usize {
        self.connection_count
    }

    /// Add a connection
    pub fn add_connection(&mut self) {
        self.connection_count += 1;
    }

    /// Clear all connections
    pub fn clear(&mut self) {
        self.connection_count = 0;
    }
}

impl Default for BRepOffsetInter3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = BRepOffsetInter3d::new();
        assert_eq!(inter.connection_count(), 0);
    }

    #[test]
    fn test_add_connection() {
        let mut inter = BRepOffsetInter3d::new();
        inter.add_connection();
        assert_eq!(inter.connection_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut inter = BRepOffsetInter3d::new();
        inter.add_connection();
        inter.clear();
        assert_eq!(inter.connection_count(), 0);
    }

    #[test]
    fn test_default() {
        let inter = BRepOffsetInter3d::default();
        assert_eq!(inter.connection_count(), 0);
    }
}
