// FILE: int_surf_allocator.rs
// occt: IntSurf_Allocator

/// Memory allocator for surface intersection data
#[derive(Clone, Debug)]
pub struct Allocator;

impl Allocator {
    /// Create a new allocator
    pub fn new() -> Self {
        Allocator
    }

    /// Allocate memory for items
    pub fn allocate(&self, size: usize) -> Vec<u8> {
        vec![0u8; size]
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _alloc = Allocator::new();
    }

    #[test]
    fn test_allocate() {
        let alloc = Allocator::new();
        let mem = alloc.allocate(10);
        assert_eq!(mem.len(), 10);
    }
}
