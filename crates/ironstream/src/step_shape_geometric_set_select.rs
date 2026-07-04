// FILE: step_shape_geometric_set_select.rs
// occt: StepShape_GeometricSetSelect

//! Representation of STEP SELECT type GeometricSetSelect

#[derive(Clone, Debug)]
pub enum GeometricSetSelect {
    /// Point
    Point(String),
    /// Curve
    Curve(String),
    /// Surface
    Surface(String),
}

impl GeometricSetSelect {
    /// Returns a GeometricSetSelect SelectType
    pub fn new() -> Option<Self> {
        None
    }

    /// Recognizes a GeometricSetSelect Kind Entity that is:
    /// 1 -> Point
    /// 2 -> Curve
    /// 3 -> Surface
    /// 0 else
    pub fn case_num(entity_type: &str) -> i32 {
        match entity_type {
            "Point" => 1,
            "Curve" => 2,
            "Surface" => 3,
            _ => 0,
        }
    }

    /// Returns value as Point (None if another type)
    pub fn point(&self) -> Option<&str> {
        if let GeometricSetSelect::Point(p) = self {
            Some(p)
        } else {
            None
        }
    }

    /// Returns value as Curve (None if another type)
    pub fn curve(&self) -> Option<&str> {
        if let GeometricSetSelect::Curve(c) = self {
            Some(c)
        } else {
            None
        }
    }

    /// Returns value as Surface (None if another type)
    pub fn surface(&self) -> Option<&str> {
        if let GeometricSetSelect::Surface(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl Default for GeometricSetSelect {
    fn default() -> Self {
        GeometricSetSelect::Point(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        assert_eq!(GeometricSetSelect::case_num("Point"), 1);
        assert_eq!(GeometricSetSelect::case_num("Curve"), 2);
        assert_eq!(GeometricSetSelect::case_num("Surface"), 3);
        assert_eq!(GeometricSetSelect::case_num("Unknown"), 0);
    }

    #[test]
    fn test_point() {
        let gss = GeometricSetSelect::Point("point1".to_string());
        assert_eq!(gss.point(), Some("point1"));
        assert!(gss.curve().is_none());
    }

    #[test]
    fn test_curve() {
        let gss = GeometricSetSelect::Curve("curve1".to_string());
        assert_eq!(gss.curve(), Some("curve1"));
        assert!(gss.point().is_none());
    }

    #[test]
    fn test_surface() {
        let gss = GeometricSetSelect::Surface("surf1".to_string());
        assert_eq!(gss.surface(), Some("surf1"));
        assert!(gss.curve().is_none());
    }
}
