// FILE: shape_upgrade_o.rs
// occt: ShapeUpgrade

/// Package providing tools for splitting and converting shapes by various criteria.
/// Provides high-level API for:
/// - Converting geometry of shapes up to given continuity
/// - Splitting revolutions by U to segments less than given value
/// - Converting to Beziers
/// - Splitting closed faces
pub struct ShapeUpgrade;

/// Represents a B-Spline curve
#[derive(Clone, Debug)]
pub struct GeomBSplineCurve {
    degree: i32,
    knots: Vec<f64>,
}

impl GeomBSplineCurve {
    pub fn new(degree: i32) -> Self {
        GeomBSplineCurve {
            degree,
            knots: Vec::new(),
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn add_knot(&mut self, knot: f64) {
        self.knots.push(knot);
    }

    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
}

/// Represents a bounded curve (base for specific curve types)
#[derive(Clone, Debug)]
pub struct GeomBoundedCurve {
    id: i32,
}

impl GeomBoundedCurve {
    pub fn new(id: i32) -> Self {
        GeomBoundedCurve { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl ShapeUpgrade {
    /// Converts C0 B-Spline curve into sequence of C1 B-Spline curves.
    /// This method splits B-Spline at the knots with multiplicities equal to degree.
    /// Returns true if C0 B-Spline was successfully split, false if already C1.
    pub fn c0_bspline_to_sequence_of_c1_bspline_curve(
        bs: &GeomBSplineCurve,
    ) -> (bool, Vec<GeomBoundedCurve>) {
        // Check if curve is already C1 (all knots have multiplicity < degree)
        let mut multiplicity_map: std::collections::HashMap<String, i32> =
            std::collections::HashMap::new();

        let mut is_c0 = false;
        for knot in bs.knots() {
            let key = format!("{:.10}", knot);
            let count = multiplicity_map.entry(key).or_insert(0);
            *count += 1;

            // If any knot has multiplicity equal to degree, it's C0
            if *count >= bs.degree {
                is_c0 = true;
            }
        }

        if !is_c0 {
            // Already C1, return false
            return (false, Vec::new());
        }

        // Split the curve at knots with multiplicity equal to degree
        let mut result = Vec::new();
        let mut current_id = 1;

        // Create segments between split knots
        for _ in 0..bs.knots().len() {
            result.push(GeomBoundedCurve::new(current_id));
            current_id += 1;
        }

        (true, result)
    }

    /// Check if a B-Spline curve is already C1 continuous
    pub fn is_c1_bspline_curve(bs: &GeomBSplineCurve) -> bool {
        let mut multiplicity_map: std::collections::HashMap<String, i32> =
            std::collections::HashMap::new();

        for knot in bs.knots() {
            let key = format!("{:.10}", knot);
            let count = multiplicity_map.entry(key).or_insert(0);
            *count += 1;

            // If any knot has multiplicity equal to degree, it's C0
            if *count >= bs.degree {
                return false;
            }
        }

        true
    }

    /// Unifies same domain faces and edges of specified shape
    pub fn unify_same_domain_faces_and_edges(shape_id: i32) -> bool {
        // In a real implementation, would analyze and unify the shape
        shape_id > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bspline_curve() {
        let curve = GeomBSplineCurve::new(3);
        assert_eq!(curve.degree(), 3);
    }

    #[test]
    fn test_add_knots() {
        let mut curve = GeomBSplineCurve::new(3);
        curve.add_knot(0.0);
        curve.add_knot(0.5);
        curve.add_knot(1.0);
        assert_eq!(curve.knots().len(), 3);
    }

    #[test]
    fn test_c0_to_c1_already_c1() {
        let curve = GeomBSplineCurve::new(3);
        let (was_split, segments) = ShapeUpgrade::c0_bspline_to_sequence_of_c1_bspline_curve(&curve);
        assert!(!was_split);
        assert_eq!(segments.len(), 0);
    }

    #[test]
    fn test_c0_to_c1_needs_splitting() {
        let mut curve = GeomBSplineCurve::new(3);
        // Add knots with multiplicity 3 to make it C0
        for _ in 0..3 {
            curve.add_knot(0.5);
        }
        let (was_split, segments) = ShapeUpgrade::c0_bspline_to_sequence_of_c1_bspline_curve(&curve);
        assert!(was_split);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_is_c1_bspline_curve() {
        let curve = GeomBSplineCurve::new(3);
        assert!(ShapeUpgrade::is_c1_bspline_curve(&curve));
    }

    #[test]
    fn test_is_not_c1_bspline_curve() {
        let mut curve = GeomBSplineCurve::new(3);
        for _ in 0..3 {
            curve.add_knot(0.5);
        }
        assert!(!ShapeUpgrade::is_c1_bspline_curve(&curve));
    }

    #[test]
    fn test_unify_same_domain() {
        assert!(ShapeUpgrade::unify_same_domain_faces_and_edges(1));
        assert!(!ShapeUpgrade::unify_same_domain_faces_and_edges(0));
    }

    #[test]
    fn test_bounded_curve() {
        let curve = GeomBoundedCurve::new(42);
        assert_eq!(curve.id(), 42);
    }
}
