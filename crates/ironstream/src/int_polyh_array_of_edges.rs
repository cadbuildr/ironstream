// FILE: int_polyh_array_of_edges.rs
// occt: IntPolyh_ArrayOfEdges

/// Implementation of IntPolyh_ArrayOfEdges
pub struct IntPolyh_ArrayOfEdges;

impl IntPolyh_ArrayOfEdges {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPolyh_ArrayOfEdges
    }
}

impl Default for IntPolyh_ArrayOfEdges {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPolyh_ArrayOfEdges::new();
    }
}
