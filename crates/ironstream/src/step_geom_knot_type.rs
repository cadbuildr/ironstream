// FILE: step_geom_knot_type.rs
// occt: StepGeom_KnotType

//! Enumeration of knot types for splines.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnotType {
    UniformKnots,
    QuasiUniformKnots,
    PiecewiseBezierKnots,
}

impl KnotType {
    pub fn as_str(&self) -> &'static str {
        match self {
            KnotType::UniformKnots => "UniformKnots",
            KnotType::QuasiUniformKnots => "QuasiUniformKnots",
            KnotType::PiecewiseBezierKnots => "PiecewiseBezierKnots",
        }
    }

    pub fn from_str(s: &str) -> Option<KnotType> {
        match s {
            "UniformKnots" => Some(KnotType::UniformKnots),
            "QuasiUniformKnots" => Some(KnotType::QuasiUniformKnots),
            "PiecewiseBezierKnots" => Some(KnotType::PiecewiseBezierKnots),
            _ => None,
        }
    }
}

impl std::fmt::Display for KnotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for KnotType {
    fn default() -> Self {
        KnotType::UniformKnots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(KnotType::UniformKnots.as_str(), "UniformKnots");
        assert_eq!(KnotType::QuasiUniformKnots.as_str(), "QuasiUniformKnots");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            KnotType::from_str("UniformKnots"),
            Some(KnotType::UniformKnots)
        );
        assert_eq!(KnotType::from_str("Invalid"), None);
    }

    #[test]
    fn test_default() {
        let kt: KnotType = Default::default();
        assert_eq!(kt, KnotType::UniformKnots);
    }
}
