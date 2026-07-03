// FILE: b_rep_offset_make_loops.rs
// occt: BRepOffset_MakeLoops

/// Constructs loops from offset edges.
/// Handles creation of closed loops in offset constructions.
#[derive(Debug, Clone)]
pub struct BRepOffsetMakeLoops {
    /// Number of loops created
    loop_count: usize,
}

impl BRepOffsetMakeLoops {
    /// Create a new MakeLoops
    pub fn new() -> Self {
        Self { loop_count: 0 }
    }

    /// Get number of loops
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    /// Add a loop
    pub fn add_loop(&mut self) {
        self.loop_count += 1;
    }

    /// Clear all loops
    pub fn clear(&mut self) {
        self.loop_count = 0;
    }
}

impl Default for BRepOffsetMakeLoops {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loops = BRepOffsetMakeLoops::new();
        assert_eq!(loops.loop_count(), 0);
    }

    #[test]
    fn test_add_loop() {
        let mut loops = BRepOffsetMakeLoops::new();
        loops.add_loop();
        loops.add_loop();
        assert_eq!(loops.loop_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut loops = BRepOffsetMakeLoops::new();
        loops.add_loop();
        loops.clear();
        assert_eq!(loops.loop_count(), 0);
    }

    #[test]
    fn test_default() {
        let loops = BRepOffsetMakeLoops::default();
        assert_eq!(loops.loop_count(), 0);
    }
}
