// FILE: step_visual_complex_triangulated_face.rs
// occt: StepVisual_ComplexTriangulatedFace

/// A complex triangulated face in STEP representation.
///
/// This represents a tessellated face composed of triangle strips and triangle fans.
pub struct ComplexTriangulatedFace {
    pnindex: Vec<i32>,
    triangle_strips: Vec<i32>,
    triangle_fans: Vec<i32>,
}

impl ComplexTriangulatedFace {
    /// Creates a new complex triangulated face.
    pub fn new() -> Self {
        ComplexTriangulatedFace {
            pnindex: Vec::new(),
            triangle_strips: Vec::new(),
            triangle_fans: Vec::new(),
        }
    }

    /// Returns the Pnindex array.
    pub fn pnindex(&self) -> &[i32] {
        &self.pnindex
    }

    /// Sets the Pnindex array.
    pub fn set_pnindex(&mut self, indices: Vec<i32>) {
        self.pnindex = indices;
    }

    /// Returns the number of Pnindex values.
    pub fn nb_pnindex(&self) -> usize {
        self.pnindex.len()
    }

    /// Returns the Pnindex value at the given index (1-based).
    pub fn pnindex_value(&self, index: usize) -> Option<i32> {
        if index > 0 && index <= self.pnindex.len() {
            Some(self.pnindex[index - 1])
        } else {
            None
        }
    }

    /// Returns the TriangleStrips array.
    pub fn triangle_strips(&self) -> &[i32] {
        &self.triangle_strips
    }

    /// Sets the TriangleStrips array.
    pub fn set_triangle_strips(&mut self, strips: Vec<i32>) {
        self.triangle_strips = strips;
    }

    /// Returns the number of TriangleStrips.
    pub fn nb_triangle_strips(&self) -> usize {
        self.triangle_strips.len()
    }

    /// Returns the TriangleFans array.
    pub fn triangle_fans(&self) -> &[i32] {
        &self.triangle_fans
    }

    /// Sets the TriangleFans array.
    pub fn set_triangle_fans(&mut self, fans: Vec<i32>) {
        self.triangle_fans = fans;
    }

    /// Returns the number of TriangleFans.
    pub fn nb_triangle_fans(&self) -> usize {
        self.triangle_fans.len()
    }
}

impl Default for ComplexTriangulatedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_triangulated_face_new() {
        let face = ComplexTriangulatedFace::new();
        assert_eq!(face.nb_pnindex(), 0);
        assert_eq!(face.nb_triangle_strips(), 0);
        assert_eq!(face.nb_triangle_fans(), 0);
    }

    #[test]
    fn test_pnindex_operations() {
        let mut face = ComplexTriangulatedFace::new();
        let indices = vec![1, 2, 3, 4, 5];
        face.set_pnindex(indices.clone());
        assert_eq!(face.nb_pnindex(), 5);
        assert_eq!(face.pnindex_value(1), Some(1));
        assert_eq!(face.pnindex_value(3), Some(3));
        assert_eq!(face.pnindex_value(5), Some(5));
        assert_eq!(face.pnindex_value(6), None);
        assert_eq!(face.pnindex(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_triangle_strips() {
        let mut face = ComplexTriangulatedFace::new();
        let strips = vec![10, 11, 12];
        face.set_triangle_strips(strips.clone());
        assert_eq!(face.nb_triangle_strips(), 3);
        assert_eq!(face.triangle_strips(), &[10, 11, 12]);
    }

    #[test]
    fn test_triangle_fans() {
        let mut face = ComplexTriangulatedFace::new();
        let fans = vec![20, 21];
        face.set_triangle_fans(fans.clone());
        assert_eq!(face.nb_triangle_fans(), 2);
        assert_eq!(face.triangle_fans(), &[20, 21]);
    }
}
