// FILE: b_rep_g_curve.rs
// occt: BRep_GCurve

/// Base class for geometric curves in BRep.
pub struct BRepGCurve {
    id: u32,
    handle: String,
    first_param: f64,
    last_param: f64,
    continuous: bool,
}

impl BRepGCurve {
    /// Create a new geometric curve.
    pub fn new(id: u32, handle: String, first: f64, last: f64) -> Self {
        Self {
            id,
            handle,
            first_param: first,
            last_param: last,
            continuous: true,
        }
    }

    /// Get the curve id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get the curve handle/geometry.
    pub fn handle(&self) -> &str {
        &self.handle
    }

    /// Get the first parameter.
    pub fn first_parameter(&self) -> f64 {
        self.first_param
    }

    /// Get the last parameter.
    pub fn last_parameter(&self) -> f64 {
        self.last_param
    }

    /// Check if the curve is continuous.
    pub fn is_continuous(&self) -> bool {
        self.continuous
    }

    /// Set continuity.
    pub fn set_continuous(&mut self, continuous: bool) {
        self.continuous = continuous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = BRepGCurve::new(1, "Geom_Line".to_string(), 0.0, 1.0);

        assert_eq!(curve.id(), 1);
        assert_eq!(curve.handle(), "Geom_Line");
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 1.0);
        assert!(curve.is_continuous());
    }
}
