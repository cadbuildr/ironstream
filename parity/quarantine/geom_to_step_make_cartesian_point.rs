// FILE: geom_to_step_make_cartesian_point.rs
// occt: GeomToStep_MakeCartesianPoint

#[derive(Clone, Debug)]
pub struct StepGeom_CartesianPoint {
    pub coordinates: (f64, f64, f64),
}

pub struct GeomToStep_MakeCartesianPoint {
    done: bool,
    result: Option<StepGeom_CartesianPoint>,
}

impl GeomToStep_MakeCartesianPoint {
    pub fn new() -> Self {
        GeomToStep_MakeCartesianPoint {
            done: false,
            result: None,
        }
    }

    pub fn from_coordinates(x: f64, y: f64, z: f64) -> Self {
        let mut conv = Self::new();
        conv.result = Some(StepGeom_CartesianPoint {
            coordinates: (x, y, z),
        });
        conv.done = true;
        conv
    }

    pub fn from_2d_coordinates(x: f64, y: f64) -> Self {
        Self::from_coordinates(x, y, 0.0)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_CartesianPoint> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeCartesianPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeCartesianPoint::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_coordinates() {
        let conv = GeomToStep_MakeCartesianPoint::from_coordinates(1.0, 2.0, 3.0);
        assert!(conv.is_done());
        let pt = conv.value().unwrap();
        assert_eq!(pt.coordinates, (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_from_2d_coordinates() {
        let conv = GeomToStep_MakeCartesianPoint::from_2d_coordinates(1.0, 2.0);
        assert!(conv.is_done());
        let pt = conv.value().unwrap();
        assert_eq!(pt.coordinates.2, 0.0);
    }
}
