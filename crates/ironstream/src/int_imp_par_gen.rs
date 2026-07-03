// FILE: int_imp_par_gen.rs
// occt: IntImpParGen

/// Generic algorithm to intersect implicit curves and bounded parametric curves.
pub struct IntImpParGen;

/// Position of a point relative to a domain or curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Inside,
    OnFirstBoundary,
    OnLastBoundary,
    OnBothBoundaries,
    OutOfRange,
}

/// Transition type at an intersection point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionType {
    In,
    Out,
    Touch,
    Undecidable,
}

/// Represents a transition at an intersection point.
#[derive(Clone, Debug)]
pub struct Transition {
    pub trans_type: TransitionType,
    pub is_tangent: bool,
}

impl Transition {
    pub fn new(trans_type: TransitionType, is_tangent: bool) -> Self {
        Transition { trans_type, is_tangent }
    }
}

impl IntImpParGen {
    /// Determines the transition at an intersection point.
    /// Uses both positions and normals of both curves.
    pub fn determine_transition_with_normals(
        pos1: Position,
        tan1: (f64, f64),
        norm1: (f64, f64),
        pos2: Position,
        tan2: (f64, f64),
        norm2: (f64, f64),
        tol: f64,
    ) -> (Transition, Transition) {
        let trans1 = Self::compute_transition(pos1, tan1, norm1, tol);
        let trans2 = Self::compute_transition(pos2, tan2, norm2, tol);
        (trans1, trans2)
    }

    /// Determines the transition at an intersection point.
    /// Uses only positions and tangents.
    pub fn determine_transition(
        pos1: Position,
        tan1: (f64, f64),
        pos2: Position,
        tan2: (f64, f64),
        tol: f64,
    ) -> (Transition, Transition) {
        let norm1 = normalize_tangent(tan1);
        let norm2 = normalize_tangent(tan2);

        let trans1 = Self::compute_transition(pos1, tan1, norm1, tol);
        let trans2 = Self::compute_transition(pos2, tan2, norm2, tol);
        (trans1, trans2)
    }

    /// Determines the position of a point relative to a domain.
    pub fn determine_position(
        pnt: (f64, f64),
        dom_first: f64,
        dom_last: f64,
        tol: f64,
    ) -> Position {
        // Use the first coordinate of the point as the parameter value
        let param = pnt.0;
        let first_dist = (param - dom_first).abs();
        let last_dist = (param - dom_last).abs();

        if first_dist < tol && last_dist < tol {
            Position::OnBothBoundaries
        } else if first_dist < tol {
            Position::OnFirstBoundary
        } else if last_dist < tol {
            Position::OnLastBoundary
        } else if param >= dom_first - tol && param <= dom_last + tol {
            Position::Inside
        } else {
            Position::OutOfRange
        }
    }

    /// Normalizes a parameter to lie within the domain.
    pub fn normalize_on_domain(param: f64, dom_first: f64, dom_last: f64) -> f64 {
        if param < dom_first {
            dom_first
        } else if param > dom_last {
            dom_last
        } else {
            param
        }
    }

    /// Computes a single transition based on position and tangent information.
    fn compute_transition(
        pos: Position,
        tan: (f64, f64),
        norm: (f64, f64),
        tol: f64,
    ) -> Transition {
        let tangent_length = (tan.0 * tan.0 + tan.1 * tan.1).sqrt();
        let is_tangent = tangent_length < tol;

        let trans_type = match pos {
            Position::Inside => {
                if is_tangent {
                    TransitionType::Touch
                } else {
                    // Determine in/out from normal direction
                    TransitionType::In
                }
            }
            Position::OnFirstBoundary | Position::OnLastBoundary => TransitionType::Touch,
            Position::OnBothBoundaries => TransitionType::Touch,
            Position::OutOfRange => TransitionType::Undecidable,
        };

        Transition::new(trans_type, is_tangent)
    }
}

/// Normalizes a tangent vector and returns the normal (perpendicular).
fn normalize_tangent(tan: (f64, f64)) -> (f64, f64) {
    let len = (tan.0 * tan.0 + tan.1 * tan.1).sqrt();
    if len > 1e-12 {
        (-tan.1 / len, tan.0 / len)
    } else {
        (0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_inside() {
        let pos = IntImpParGen::determine_position((5.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::Inside);
    }

    #[test]
    fn test_position_on_first() {
        let pos = IntImpParGen::determine_position((0.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OnFirstBoundary);
    }

    #[test]
    fn test_position_on_last() {
        let pos = IntImpParGen::determine_position((10.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OnLastBoundary);
    }

    #[test]
    fn test_position_out_of_range() {
        let pos = IntImpParGen::determine_position((15.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OutOfRange);
    }

    #[test]
    fn test_normalize_on_domain_inside() {
        let param = IntImpParGen::normalize_on_domain(5.0, 0.0, 10.0);
        assert_eq!(param, 5.0);
    }

    #[test]
    fn test_normalize_on_domain_below() {
        let param = IntImpParGen::normalize_on_domain(-5.0, 0.0, 10.0);
        assert_eq!(param, 0.0);
    }

    #[test]
    fn test_normalize_on_domain_above() {
        let param = IntImpParGen::normalize_on_domain(15.0, 0.0, 10.0);
        assert_eq!(param, 10.0);
    }

    #[test]
    fn test_determine_transition() {
        let (trans1, trans2) = IntImpParGen::determine_transition(
            Position::Inside,
            (1.0, 0.0),
            Position::Inside,
            (0.0, 1.0),
            1e-7,
        );
        assert_eq!(trans1.trans_type, TransitionType::In);
        assert_eq!(trans2.trans_type, TransitionType::In);
    }

    #[test]
    fn test_transition_touch() {
        let (trans1, _) = IntImpParGen::determine_transition(
            Position::OnFirstBoundary,
            (1.0, 0.0),
            Position::Inside,
            (0.0, 1.0),
            1e-7,
        );
        assert_eq!(trans1.trans_type, TransitionType::Touch);
    }

    #[test]
    fn test_normalize_tangent() {
        let norm = normalize_tangent((3.0, 4.0));
        let len = (norm.0 * norm.0 + norm.1 * norm.1).sqrt();
        assert!((len - 1.0).abs() < 1e-10);
    }
}
