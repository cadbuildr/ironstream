// FILE: step_visual_cubic_bezier_triangulated_face.rs
// occt: StepVisual_CubicBezierTriangulatedFace

/// A cubic Bezier triangulated face in STEP representation.
///
/// This represents a triangulated face using cubic Bezier surfaces.
pub struct CubicBezierTriangulatedFace {
    triangles: Vec<(usize, usize, usize)>,
}

impl CubicBezierTriangulatedFace {
    /// Creates a new cubic Bezier triangulated face.
    pub fn new() -> Self {
        CubicBezierTriangulatedFace {
            triangles: Vec::new(),
        }
    }

    /// Sets the triangles (3-tuples of vertex indices).
    pub fn set_triangles(&mut self, triangles: Vec<(usize, usize, usize)>) {
        self.triangles = triangles;
    }

    /// Returns the triangles.
    pub fn triangles(&self) -> &[(usize, usize, usize)] {
        &self.triangles
    }

    /// Returns the number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }
}

impl Default for CubicBezierTriangulatedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_triangulated_face_new() {
        let face = CubicBezierTriangulatedFace::new();
        assert_eq!(face.nb_triangles(), 0);
    }

    #[test]
    fn test_set_triangles() {
        let mut face = CubicBezierTriangulatedFace::new();
        let triangles = vec![(0, 1, 2), (1, 2, 3), (2, 3, 4)];
        face.set_triangles(triangles.clone());
        assert_eq!(face.nb_triangles(), 3);
        assert_eq!(face.triangles()[0], (0, 1, 2));
    }
}
