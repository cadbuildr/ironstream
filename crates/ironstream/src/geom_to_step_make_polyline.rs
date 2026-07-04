// FILE: geom_to_step_make_polyline.rs
// occt: GeomToStep_MakePolyline

#[derive(Clone, Debug)]
pub struct StepGeom_Polyline {
    pub points: Vec<(f64, f64, f64)>,
}

pub struct GeomToStep_MakePolyline {
    done: bool,
    result: Option<StepGeom_Polyline>,
}

impl GeomToStep_MakePolyline {
    pub fn new() -> Self {
        GeomToStep_MakePolyline {
            done: false,
            result: None,
        }
    }

    pub fn from_points(points: Vec<(f64, f64, f64)>) -> Self {
        let mut conv = Self::new();
        if points.len() >= 2 {
            conv.result = Some(StepGeom_Polyline { points });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Polyline> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakePolyline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_points() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 0.0)];
        let conv = GeomToStep_MakePolyline::from_points(points);
        assert!(conv.is_done());
    }

    #[test]
    fn test_single_point() {
        let points = vec![(0.0, 0.0, 0.0)];
        let conv = GeomToStep_MakePolyline::from_points(points);
        assert!(!conv.is_done());
    }
}
