// FILE: step_visual_triangulated_surface_set.rs
// occt: StepVisual_TriangulatedSurfaceSet

/// Represents a STEP TriangulatedSurfaceSet entity.
pub struct TriangulatedSurfaceSet {
    name: String,
    pnindex: Vec<i32>,
    triangles: Vec<Vec<i32>>,
}

impl TriangulatedSurfaceSet {
    /// Creates a new triangulated surface set.
    pub fn new() -> Self {
        TriangulatedSurfaceSet {
            name: String::new(),
            pnindex: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        pnindex: Vec<i32>,
        triangles: Vec<Vec<i32>>,
    ) {
        self.name = name;
        self.pnindex = pnindex;
        self.triangles = triangles;
    }

    /// Returns the pnindex.
    pub fn pnindex(&self) -> &[i32] {
        &self.pnindex
    }

    /// Sets the pnindex.
    pub fn set_pnindex(&mut self, pnindex: Vec<i32>) {
        self.pnindex = pnindex;
    }

    /// Returns the number of pnindex entries.
    pub fn nb_pnindex(&self) -> usize {
        self.pnindex.len()
    }

    /// Returns the pnindex value at the given index.
    pub fn pnindex_value(&self, idx: usize) -> Option<i32> {
        self.pnindex.get(idx).copied()
    }

    /// Returns the triangles.
    pub fn triangles(&self) -> &[Vec<i32>] {
        &self.triangles
    }

    /// Sets the triangles.
    pub fn set_triangles(&mut self, triangles: Vec<Vec<i32>>) {
        self.triangles = triangles;
    }

    /// Returns the number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }
}

impl Default for TriangulatedSurfaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tss = TriangulatedSurfaceSet::new();
        assert_eq!(tss.nb_pnindex(), 0);
        assert_eq!(tss.nb_triangles(), 0);
    }

    #[test]
    fn test_pnindex() {
        let mut tss = TriangulatedSurfaceSet::new();
        let indices = vec![10, 20, 30];
        tss.set_pnindex(indices);
        assert_eq!(tss.nb_pnindex(), 3);
        assert_eq!(tss.pnindex_value(0), Some(10));
    }

    #[test]
    fn test_triangles() {
        let mut tss = TriangulatedSurfaceSet::new();
        let triangles = vec![vec![1, 2, 3]];
        tss.set_triangles(triangles);
        assert_eq!(tss.nb_triangles(), 1);
    }
}
