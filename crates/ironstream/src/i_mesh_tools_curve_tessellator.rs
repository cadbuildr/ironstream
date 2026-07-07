// FILE: i_mesh_tools_curve_tessellator.rs
// occt: IMeshTools_CurveTessellator

/// A tessellation point consisting of coordinates and parameter
#[derive(Clone, Debug)]
pub struct TessellationPoint {
    /// 3D point coordinates [x, y, z]
    point: [f64; 3],
    /// Parameter on curve
    parameter: f64,
}

impl TessellationPoint {
    pub fn new(x: f64, y: f64, z: f64, parameter: f64) -> Self {
        TessellationPoint {
            point: [x, y, z],
            parameter,
        }
    }

    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }
}

/// Interface class providing API for curve tessellation tools.
/// This is the base trait for concrete tessellator implementations.
pub trait IMeshToolsCurveTessellator {
    /// Returns number of tessellation points.
    fn points_nb(&self) -> i32;

    /// Returns parameters of solution with the given index.
    /// Returns (point, parameter) tuple or None if index is invalid.
    fn value(&self, index: i32) -> Option<TessellationPoint>;
}

/// Concrete implementation of a curve tessellator
pub struct SimpleCurveTessellator {
    /// Tessellation points
    points: Vec<TessellationPoint>,
}

impl SimpleCurveTessellator {
    /// Create a new curve tessellator
    pub fn new() -> Self {
        SimpleCurveTessellator {
            points: Vec::new(),
        }
    }

    /// Add a tessellation point
    pub fn add_point(&mut self, point: TessellationPoint) {
        self.points.push(point);
    }

    /// Get all points
    pub fn points(&self) -> &[TessellationPoint] {
        &self.points
    }

    /// Clear all points
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for SimpleCurveTessellator {
    fn default() -> Self {
        Self::new()
    }
}

impl IMeshToolsCurveTessellator for SimpleCurveTessellator {
    fn points_nb(&self) -> i32 {
        self.points.len() as i32
    }

    fn value(&self, index: i32) -> Option<TessellationPoint> {
        if index < 0 || index >= self.points.len() as i32 {
            None
        } else {
            Some(self.points[index as usize].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tessellation_point() {
        let pt = TessellationPoint::new(1.0, 2.0, 3.0, 0.5);
        assert_eq!(pt.point(), [1.0, 2.0, 3.0]);
        assert_eq!(pt.parameter(), 0.5);
    }

    #[test]
    fn test_create_tessellator() {
        let tess = SimpleCurveTessellator::new();
        assert_eq!(tess.points_nb(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut tess = SimpleCurveTessellator::new();
        tess.add_point(TessellationPoint::new(0.0, 0.0, 0.0, 0.0));
        tess.add_point(TessellationPoint::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(tess.points_nb(), 2);
    }

    #[test]
    fn test_value() {
        let mut tess = SimpleCurveTessellator::new();
        tess.add_point(TessellationPoint::new(1.5, 2.5, 3.5, 0.25));
        tess.add_point(TessellationPoint::new(4.5, 5.5, 6.5, 0.75));

        let pt0 = tess.value(0).unwrap();
        assert_eq!(pt0.point(), [1.5, 2.5, 3.5]);
        assert_eq!(pt0.parameter(), 0.25);

        let pt1 = tess.value(1).unwrap();
        assert_eq!(pt1.point(), [4.5, 5.5, 6.5]);
        assert_eq!(pt1.parameter(), 0.75);
    }

    #[test]
    fn test_value_out_of_bounds() {
        let mut tess = SimpleCurveTessellator::new();
        tess.add_point(TessellationPoint::new(0.0, 0.0, 0.0, 0.0));
        assert!(tess.value(-1).is_none());
        assert!(tess.value(10).is_none());
    }

    #[test]
    fn test_value_empty() {
        let tess = SimpleCurveTessellator::new();
        assert!(tess.value(0).is_none());
    }

    #[test]
    fn test_clear() {
        let mut tess = SimpleCurveTessellator::new();
        tess.add_point(TessellationPoint::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(tess.points_nb(), 1);
        tess.clear();
        assert_eq!(tess.points_nb(), 0);
    }

    #[test]
    fn test_default() {
        let tess = SimpleCurveTessellator::default();
        assert_eq!(tess.points_nb(), 0);
    }
}
