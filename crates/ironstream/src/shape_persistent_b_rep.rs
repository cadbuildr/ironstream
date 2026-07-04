// FILE: shape_persistent_b_rep.rs
// occt: ShapePersistent_BRep

/// BRep shape persistence for topological data
pub struct ShapePersistentBRep;

/// Point representation on BRep geometry
pub struct PointRepresentation {
    location: Option<String>,
    parameter: f64,
}

impl PointRepresentation {
    /// Create a new point representation
    pub fn new() -> Self {
        PointRepresentation {
            location: None,
            parameter: 0.0,
        }
    }

    /// Get the parameter value
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Set the parameter value
    pub fn set_parameter(&mut self, param: f64) {
        self.parameter = param;
    }

    /// Get the location
    pub fn location(&self) -> Option<&str> {
        self.location.as_ref().map(|s| s.as_str())
    }

    /// Set the location
    pub fn set_location(&mut self, loc: Option<String>) {
        self.location = loc;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "PBRep_PointRepresentation"
    }
}

impl Default for PointRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

/// Point on curve representation
pub struct PointOnCurve {
    base: PointRepresentation,
    curve: Option<String>,
}

impl PointOnCurve {
    /// Create a new point on curve
    pub fn new() -> Self {
        PointOnCurve {
            base: PointRepresentation::new(),
            curve: None,
        }
    }

    /// Get the associated curve
    pub fn curve(&self) -> Option<&str> {
        self.curve.as_ref().map(|s| s.as_str())
    }

    /// Set the associated curve
    pub fn set_curve(&mut self, curve: Option<String>) {
        self.curve = curve;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "PBRep_PointOnCurve"
    }
}

impl Default for PointOnCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// Points on surface representation
pub struct PointsOnSurface {
    base: PointRepresentation,
    surface: Option<String>,
}

impl PointsOnSurface {
    /// Create a new points on surface
    pub fn new() -> Self {
        PointsOnSurface {
            base: PointRepresentation::new(),
            surface: None,
        }
    }

    /// Get the associated surface
    pub fn surface(&self) -> Option<&str> {
        self.surface.as_ref().map(|s| s.as_str())
    }

    /// Set the associated surface
    pub fn set_surface(&mut self, surface: Option<String>) {
        self.surface = surface;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "PBRep_PointsOnSurface"
    }
}

impl Default for PointsOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_representation() {
        let mut pt = PointRepresentation::new();
        assert_eq!(pt.parameter(), 0.0);

        pt.set_parameter(3.14);
        assert_eq!(pt.parameter(), 3.14);
        assert_eq!(pt.p_name(), "PBRep_PointRepresentation");
    }

    #[test]
    fn test_point_on_curve() {
        let mut pt = PointOnCurve::new();
        pt.set_curve(Some("Line".to_string()));

        assert_eq!(pt.curve(), Some("Line"));
        assert_eq!(pt.p_name(), "PBRep_PointOnCurve");
    }

    #[test]
    fn test_points_on_surface() {
        let mut pts = PointsOnSurface::new();
        pts.set_surface(Some("Plane".to_string()));

        assert_eq!(pts.surface(), Some("Plane"));
        assert_eq!(pts.p_name(), "PBRep_PointsOnSurface");
    }
}
