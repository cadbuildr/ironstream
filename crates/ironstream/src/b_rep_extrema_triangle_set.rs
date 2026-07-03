// FILE: b_rep_extrema_triangle_set.rs
// occt: BRepExtrema_TriangleSet

/// Triangle set for mesh processing
pub struct TriangleSet;

impl TriangleSet {
    pub fn new() -> Self {
        TriangleSet
    }
}

impl Default for TriangleSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_set_creation() {
        let _set = TriangleSet::new();
    }
}
