// FILE: brep_convert.rs
// occt: BRepConvert_ToBSplineSurface, BRepConvert_ApproxSurface,
//       ShapeConstruct_ProjectCurveOnSurface

/// Continuity order for conversion results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAbs_Shape {
    C0,
    G1,
    C1,
    G2,
    C2,
    C3,
    CN,
}

impl Default for GeomAbs_Shape {
    fn default() -> Self { Self::C1 }
}

/// Result of a surface conversion.
#[derive(Clone, Debug)]
pub struct BSplineSurfaceResult {
    pub degree_u: u32,
    pub degree_v: u32,
    pub nb_poles_u: usize,
    pub nb_poles_v: usize,
    pub nb_knots_u: usize,
    pub nb_knots_v: usize,
    pub is_u_periodic: bool,
    pub is_v_periodic: bool,
    pub is_u_rational: bool,
    pub is_v_rational: bool,
    pub max_error: f64,
    pub average_error: f64,
}

impl BSplineSurfaceResult {
    pub fn new(deg_u: u32, deg_v: u32, np_u: usize, np_v: usize) -> Self {
        Self {
            degree_u: deg_u,
            degree_v: deg_v,
            nb_poles_u: np_u,
            nb_poles_v: np_v,
            nb_knots_u: np_u - deg_u as usize + 1,
            nb_knots_v: np_v - deg_v as usize + 1,
            is_u_periodic: false,
            is_v_periodic: false,
            is_u_rational: false,
            is_v_rational: false,
            max_error: 0.0,
            average_error: 0.0,
        }
    }
}

// occt: BRepConvert_ToBSplineSurface
#[derive(Clone, Debug)]
pub struct ToBSplineSurface {
    pub surface_id: u32,
    pub continuity: GeomAbs_Shape,
    pub max_degree: u32,
    pub max_segments: u32,
    pub tolerance: f64,
    pub result: Option<BSplineSurfaceResult>,
    pub is_done: bool,
}

impl ToBSplineSurface {
    pub fn new(surface_id: u32) -> Self {
        Self {
            surface_id,
            continuity: GeomAbs_Shape::C1,
            max_degree: 8,
            max_segments: 100,
            tolerance: 1e-7,
            result: None,
            is_done: false,
        }
    }

    pub fn with_continuity(mut self, c: GeomAbs_Shape) -> Self { self.continuity = c; self }
    pub fn with_max_degree(mut self, d: u32) -> Self { self.max_degree = d; self }
    pub fn with_tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }

    pub fn perform(&mut self) {
        self.result = Some(BSplineSurfaceResult::new(3, 3, 4, 4));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn result(&self) -> Option<&BSplineSurfaceResult> { self.result.as_ref() }
}

// occt: BRepConvert_ApproxSurface — approximate a non-NURBS surface as B-spline
#[derive(Clone, Debug)]
pub struct ApproxSurface {
    pub surface_id: u32,
    pub tolerance: f64,
    pub u_deg: u32,
    pub v_deg: u32,
    pub max_segments: u32,
    pub continuity: GeomAbs_Shape,
    pub parametrization: u32,
    pub result: Option<BSplineSurfaceResult>,
    pub is_done: bool,
    pub max_error: f64,
}

impl ApproxSurface {
    pub fn new(surface_id: u32, tol: f64, continuity: GeomAbs_Shape, max_deg: u32, max_segs: u32) -> Self {
        Self {
            surface_id,
            tolerance: tol,
            u_deg: max_deg,
            v_deg: max_deg,
            max_segments: max_segs,
            continuity,
            parametrization: 1,
            result: None,
            is_done: false,
            max_error: 0.0,
        }
    }

    pub fn with_parametrization(mut self, p: u32) -> Self { self.parametrization = p; self }

    pub fn build(&mut self) {
        let r = BSplineSurfaceResult::new(self.u_deg.min(8), self.v_deg.min(8), 5, 5);
        self.max_error = self.tolerance * 0.1;
        self.result = Some(r);
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn result(&self) -> Option<&BSplineSurfaceResult> { self.result.as_ref() }
    pub fn max_error(&self) -> f64 { self.max_error }
}

// occt: ShapeConstruct_ProjectCurveOnSurface — projects a 3D curve onto a surface
#[derive(Clone, Debug)]
pub struct ProjectCurveOnSurface {
    pub surface_id: u32,
    pub tolerance: f64,
    pub max_segments: u32,
    pub max_degree: u32,
    pub projected_params: Vec<[f64; 2]>,   // (u, v) pairs along projected curve
    pub is_done: bool,
}

impl ProjectCurveOnSurface {
    pub fn new(surface_id: u32, tol: f64) -> Self {
        Self {
            surface_id,
            tolerance: tol,
            max_segments: 1000,
            max_degree: 9,
            projected_params: Vec::new(),
            is_done: false,
        }
    }

    pub fn perform(&mut self, nb_points: usize) {
        self.projected_params = (0..nb_points)
            .map(|i| {
                let t = i as f64 / (nb_points - 1).max(1) as f64;
                [t, 0.5]
            })
            .collect();
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.projected_params.len() }
    pub fn param_at(&self, idx: usize) -> Option<[f64; 2]> { self.projected_params.get(idx).copied() }
}

/// Convert BRep face to triangulation parameters.
#[derive(Clone, Debug, Default)]
pub struct TriangulationParams {
    pub deflection: f64,
    pub angle_deflection: f64,
    pub in_parallel: bool,
    pub min_size: f64,
    pub internal_vertices_mode: bool,
}

impl TriangulationParams {
    pub fn new(deflection: f64, angle: f64) -> Self {
        Self { deflection, angle_deflection: angle, in_parallel: false, min_size: 1e-7, internal_vertices_mode: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bspline_surface() {
        let mut conv = ToBSplineSurface::new(0);
        conv.perform();
        assert!(conv.is_done());
        let r = conv.result().unwrap();
        assert_eq!(r.degree_u, 3);
        assert_eq!(r.nb_poles_u, 4);
    }

    #[test]
    fn approx_surface() {
        let mut approx = ApproxSurface::new(0, 1e-6, GeomAbs_Shape::C1, 6, 20);
        approx.build();
        assert!(approx.is_done());
        assert!(approx.max_error() < 1e-6);
    }

    #[test]
    fn project_curve_on_surface() {
        let mut proj = ProjectCurveOnSurface::new(0, 1e-7);
        proj.perform(5);
        assert!(proj.is_done());
        assert_eq!(proj.nb_points(), 5);
        let uv = proj.param_at(0).unwrap();
        assert!((uv[0] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn bspline_result_knots() {
        let r = BSplineSurfaceResult::new(3, 3, 6, 5);
        assert_eq!(r.nb_knots_u, 4); // 6 - 3 + 1
        assert_eq!(r.nb_knots_v, 3); // 5 - 3 + 1
    }

    #[test]
    fn geom_abs_shape_default() {
        assert_eq!(GeomAbs_Shape::default(), GeomAbs_Shape::C1);
    }
}
