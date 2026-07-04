// FILE: step_visual_cubic_bezier_tessellated_edge.rs
// occt: StepVisual_CubicBezierTessellatedEdge

/// A cubic Bezier tessellated edge in STEP representation.
///
/// This represents a tessellated edge using cubic Bezier curves.
pub struct CubicBezierTessellatedEdge {
    control_points: Vec<(f64, f64, f64)>,
}

impl CubicBezierTessellatedEdge {
    /// Creates a new cubic Bezier tessellated edge.
    pub fn new() -> Self {
        CubicBezierTessellatedEdge {
            control_points: Vec::new(),
        }
    }

    /// Sets the control points.
    pub fn set_control_points(&mut self, points: Vec<(f64, f64, f64)>) {
        self.control_points = points;
    }

    /// Returns the control points.
    pub fn control_points(&self) -> &[(f64, f64, f64)] {
        &self.control_points
    }

    /// Returns the number of control points.
    pub fn nb_control_points(&self) -> usize {
        self.control_points.len()
    }
}

impl Default for CubicBezierTessellatedEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_tessellated_edge_new() {
        let edge = CubicBezierTessellatedEdge::new();
        assert_eq!(edge.nb_control_points(), 0);
    }

    #[test]
    fn test_set_control_points() {
        let mut edge = CubicBezierTessellatedEdge::new();
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (2.0, 0.0, 0.0)];
        edge.set_control_points(points.clone());
        assert_eq!(edge.nb_control_points(), 3);
        assert_eq!(edge.control_points()[0], (0.0, 0.0, 0.0));
    }
}
