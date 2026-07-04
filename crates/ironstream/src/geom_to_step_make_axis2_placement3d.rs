// FILE: geom_to_step_make_axis2_placement3d.rs
// occt: GeomToStep_MakeAxis2Placement3d

#[derive(Clone, Debug)]
pub struct StepGeom_Axis2Placement3d {
    pub location: (f64, f64, f64),
    pub axis: (f64, f64, f64),
    pub ref_direction: (f64, f64, f64),
}

impl Default for StepGeom_Axis2Placement3d {
    fn default() -> Self {
        StepGeom_Axis2Placement3d {
            location: (0.0, 0.0, 0.0),
            axis: (0.0, 0.0, 1.0),
            ref_direction: (1.0, 0.0, 0.0),
        }
    }
}

pub struct GeomToStep_MakeAxis2Placement3d {
    done: bool,
    result: Option<StepGeom_Axis2Placement3d>,
}

impl GeomToStep_MakeAxis2Placement3d {
    pub fn new() -> Self {
        GeomToStep_MakeAxis2Placement3d {
            done: false,
            result: None,
        }
    }

    pub fn from_location_axis_and_direction(
        lx: f64, ly: f64, lz: f64,
        ax: f64, ay: f64, az: f64,
        rx: f64, ry: f64, rz: f64,
    ) -> Self {
        let axis_norm = (ax * ax + ay * ay + az * az).sqrt();
        let ref_norm = (rx * rx + ry * ry + rz * rz).sqrt();
        let mut conv = Self::new();
        if axis_norm > 1e-10 && ref_norm > 1e-10 {
            conv.result = Some(StepGeom_Axis2Placement3d {
                location: (lx, ly, lz),
                axis: (ax / axis_norm, ay / axis_norm, az / axis_norm),
                ref_direction: (rx / ref_norm, ry / ref_norm, rz / ref_norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Axis2Placement3d> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeAxis2Placement3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeAxis2Placement3d::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_location_axis_and_direction() {
        let conv = GeomToStep_MakeAxis2Placement3d::from_location_axis_and_direction(
            1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
        );
        assert!(conv.is_done());
        let result = conv.value().unwrap();
        assert_eq!(result.location, (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_zero_axis() {
        let conv = GeomToStep_MakeAxis2Placement3d::from_location_axis_and_direction(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        );
        assert!(!conv.is_done());
    }
}
