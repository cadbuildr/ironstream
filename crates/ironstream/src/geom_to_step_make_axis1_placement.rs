// FILE: geom_to_step_make_axis1_placement.rs
// occt: GeomToStep_MakeAxis1Placement

/// Base class for GeomToStep converters providing common error reporting
pub struct GeomToStep_Root {
    done: bool,
}

impl GeomToStep_Root {
    pub fn new() -> Self {
        GeomToStep_Root { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, value: bool) {
        self.done = value;
    }
}

/// Represents a STEP Axis1 Placement (origin point + direction)
#[derive(Clone, Debug)]
pub struct StepGeom_Axis1Placement {
    pub name: Option<String>,
    pub location: (f64, f64, f64),
    pub direction: (f64, f64, f64),
}

impl Default for StepGeom_Axis1Placement {
    fn default() -> Self {
        StepGeom_Axis1Placement {
            name: None,
            location: (0.0, 0.0, 0.0),
            direction: (0.0, 0.0, 1.0),
        }
    }
}

/// Converts geometric axis placement to STEP Axis1Placement
pub struct GeomToStep_MakeAxis1Placement {
    root: GeomToStep_Root,
    result: Option<StepGeom_Axis1Placement>,
}

impl GeomToStep_MakeAxis1Placement {
    /// Creates a new converter for Axis1 Placement
    pub fn new() -> Self {
        GeomToStep_MakeAxis1Placement {
            root: GeomToStep_Root::new(),
            result: None,
        }
    }

    /// Creates converter from a point and direction vector
    pub fn from_point_and_direction(px: f64, py: f64, pz: f64, dx: f64, dy: f64, dz: f64) -> Self {
        let mut conv = Self::new();
        let norm = (dx * dx + dy * dy + dz * dz).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_Axis1Placement {
                name: None,
                location: (px, py, pz),
                direction: (dx / norm, dy / norm, dz / norm),
            });
            conv.root.set_done(true);
        }
        conv
    }

    /// Returns whether the conversion succeeded
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }

    /// Returns the resulting STEP Axis1Placement
    pub fn value(&self) -> Option<&StepGeom_Axis1Placement> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeAxis1Placement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeAxis1Placement::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_point_and_direction() {
        let conv = GeomToStep_MakeAxis1Placement::from_point_and_direction(
            1.0, 2.0, 3.0, 0.0, 0.0, 1.0,
        );
        assert!(conv.is_done());
        let result = conv.value().unwrap();
        assert_eq!(result.location, (1.0, 2.0, 3.0));
        assert_eq!(result.direction.2, 1.0); // Z component of normalized direction
    }

    #[test]
    fn test_zero_direction() {
        let conv = GeomToStep_MakeAxis1Placement::from_point_and_direction(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        );
        assert!(!conv.is_done());
    }

    #[test]
    fn test_normalization() {
        let conv = GeomToStep_MakeAxis1Placement::from_point_and_direction(
            0.0, 0.0, 0.0, 2.0, 0.0, 0.0,
        );
        assert!(conv.is_done());
        let result = conv.value().unwrap();
        let dir = result.direction;
        let norm = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }
}
