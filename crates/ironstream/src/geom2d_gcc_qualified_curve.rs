// FILE: geom2d_gcc_qualified_curve.rs
// occt: Geom2dGcc_QualifiedCurve

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Geom2dGccCurveQualifier {
    Unqualified,
    Enclosing,
    Enclosed,
    Outside,
}

/// Qualified 2D curve for geometric constraint construction.
#[derive(Debug, Clone)]
pub struct Geom2dGccQualifiedCurve {
    pub qualifier: Geom2dGccCurveQualifier,
}

impl Geom2dGccQualifiedCurve {
    pub fn new(qualifier: Geom2dGccCurveQualifier) -> Self {
        Geom2dGccQualifiedCurve { qualifier }
    }

    pub fn unqualified() -> Self {
        Geom2dGccQualifiedCurve { qualifier: Geom2dGccCurveQualifier::Unqualified }
    }

    pub fn enclosing() -> Self {
        Geom2dGccQualifiedCurve { qualifier: Geom2dGccCurveQualifier::Enclosing }
    }

    pub fn enclosed() -> Self {
        Geom2dGccQualifiedCurve { qualifier: Geom2dGccCurveQualifier::Enclosed }
    }

    pub fn outside() -> Self {
        Geom2dGccQualifiedCurve { qualifier: Geom2dGccCurveQualifier::Outside }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualified_curve_creation() {
        let qc = Geom2dGccQualifiedCurve::unqualified();
        assert_eq!(qc.qualifier, Geom2dGccCurveQualifier::Unqualified);
    }

    #[test]
    fn test_qualified_curve_enclosing() {
        let qc = Geom2dGccQualifiedCurve::enclosing();
        assert_eq!(qc.qualifier, Geom2dGccCurveQualifier::Enclosing);
    }
}
