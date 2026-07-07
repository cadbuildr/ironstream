// FILE: geom_to_step_make_b_spline_surface_with_knots.rs
// occt: GeomToStep_MakeBSplineSurfaceWithKnots

#[derive(Clone, Debug)]
pub struct StepGeom_BSplineSurfaceWithKnots {
    pub u_degree: i32,
    pub v_degree: i32,
    pub control_points: Vec<Vec<(f64, f64, f64)>>,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
}

pub struct GeomToStep_MakeBSplineSurfaceWithKnots {
    done: bool,
    result: Option<StepGeom_BSplineSurfaceWithKnots>,
}

impl GeomToStep_MakeBSplineSurfaceWithKnots {
    pub fn new() -> Self {
        GeomToStep_MakeBSplineSurfaceWithKnots {
            done: false,
            result: None,
        }
    }

    pub fn from_surface(
        u_degree: i32,
        v_degree: i32,
        points: Vec<Vec<(f64, f64, f64)>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
    ) -> Self {
        let mut conv = Self::new();
        if u_degree > 0 && v_degree > 0 && !points.is_empty() && !u_knots.is_empty() && !v_knots.is_empty() {
            conv.result = Some(StepGeom_BSplineSurfaceWithKnots {
                u_degree,
                v_degree,
                control_points: points,
                u_knots,
                v_knots,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BSplineSurfaceWithKnots> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBSplineSurfaceWithKnots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBSplineSurfaceWithKnots::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_surface() {
        let points = vec![vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)]];
        let u_knots = vec![0.0, 1.0];
        let v_knots = vec![0.0, 1.0];
        let conv = GeomToStep_MakeBSplineSurfaceWithKnots::from_surface(
            1, 1, points, u_knots, v_knots,
        );
        assert!(conv.is_done());
    }
}
