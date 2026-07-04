// FILE: geom_to_step_make_b_spline_surface_with_knots_and_rational_b_spline_surface.rs
// occt: GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface

#[derive(Clone, Debug)]
pub struct StepGeom_BSplineSurfaceWithKnotsAndRationalBSplineSurface {
    pub u_degree: i32,
    pub v_degree: i32,
    pub control_points: Vec<Vec<(f64, f64, f64)>>,
    pub weights: Vec<Vec<f64>>,
}

pub struct GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    done: bool,
    result: Option<StepGeom_BSplineSurfaceWithKnotsAndRationalBSplineSurface>,
}

impl GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    pub fn new() -> Self {
        GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_rational_surface(
        u_degree: i32,
        v_degree: i32,
        points: Vec<Vec<(f64, f64, f64)>>,
        weights: Vec<Vec<f64>>,
    ) -> Self {
        let mut conv = Self::new();
        if u_degree > 0 && v_degree > 0 && !points.is_empty() && !weights.is_empty() {
            conv.result = Some(StepGeom_BSplineSurfaceWithKnotsAndRationalBSplineSurface {
                u_degree,
                v_degree,
                control_points: points,
                weights,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BSplineSurfaceWithKnotsAndRationalBSplineSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_rational_surface() {
        let points = vec![vec![(0.0, 0.0, 0.0)]];
        let weights = vec![vec![1.0]];
        let conv = GeomToStep_MakeBSplineSurfaceWithKnotsAndRationalBSplineSurface::from_rational_surface(
            1, 1, points, weights,
        );
        assert!(conv.is_done());
    }
}
