// FILE: step_visual_triangulated_face.rs
// occt: StepVisual_TriangulatedFace

/// Represents a STEP TriangulatedFace entity.
pub struct TriangulatedFace {
    name: String,
    pnindex: Vec<i32>,
    triangles: Vec<Vec<i32>>,
}

impl TriangulatedFace {
    /// Creates a new triangulated face.
    pub fn new() -> Self {
        TriangulatedFace {
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

impl Default for TriangulatedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tf = TriangulatedFace::new();
        assert_eq!(tf.nb_pnindex(), 0);
        assert_eq!(tf.nb_triangles(), 0);
    }

    #[test]
    fn test_pnindex() {
        let mut tf = TriangulatedFace::new();
        let indices = vec![1, 2, 3];
        tf.set_pnindex(indices);
        assert_eq!(tf.nb_pnindex(), 3);
        assert_eq!(tf.pnindex_value(0), Some(1));
        assert_eq!(tf.pnindex_value(1), Some(2));
        assert_eq!(tf.pnindex_value(2), Some(3));
    }

    #[test]
    fn test_triangles() {
        let mut tf = TriangulatedFace::new();
        let triangles = vec![vec![1, 2, 3], vec![4, 5, 6]];
        tf.set_triangles(triangles);
        assert_eq!(tf.nb_triangles(), 2);
    }
}
