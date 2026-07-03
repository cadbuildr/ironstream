// FILE: geom_spl_ext.rs
// occt: GeomConvert_BSplineSurfaceToBezierSurface,
//       GeomConvert_BSplineCurveToBezierCurve,
//       GeomConvert_CompCurveToBSplineCurve

/// Convert B-spline curve to array of Bezier arcs (stub).
/// occt: GeomConvert_BSplineCurveToBezierCurve
#[derive(Clone, Debug)]
pub struct GeomConvertBSplineToBezierCurve {
    pub curve_id: u32,
    pub nb_arcs: usize,
    pub degree: usize,
}

impl GeomConvertBSplineToBezierCurve {
    /// degree = polynomial degree, nb_knots = number of internal knot spans.
    pub fn new(curve_id: u32, degree: usize, nb_knots: usize) -> Self {
        let nb_arcs = if nb_knots > 0 { nb_knots } else { 1 };
        Self { curve_id, nb_arcs, degree: degree.max(1) }
    }

    pub fn nb_arcs(&self) -> usize { self.nb_arcs }
    pub fn degree(&self) -> usize { self.degree }
    pub fn nb_poles_per_arc(&self) -> usize { self.degree + 1 }

    /// Get poles for arc i (0-based). Returns unit circle points as stub.
    pub fn arc_poles(&self, i: usize) -> Option<Vec<[f64; 3]>> {
        if i >= self.nb_arcs { return None; }
        let n = self.nb_poles_per_arc();
        let start = i as f64 / self.nb_arcs as f64;
        let step = 1.0 / (self.nb_arcs as f64 * (n - 1) as f64);
        let poles: Vec<[f64; 3]> = (0..n).map(|k| {
            let t = (start + k as f64 * step) * 2.0 * std::f64::consts::PI;
            [t.cos(), t.sin(), 0.0]
        }).collect();
        Some(poles)
    }
}

/// Convert B-spline surface to array of Bezier patches (stub).
/// occt: GeomConvert_BSplineSurfaceToBezierSurface
#[derive(Clone, Debug)]
pub struct GeomConvertBSplineToBezierSurface {
    pub surface_id: u32,
    pub nb_u_patches: usize,
    pub nb_v_patches: usize,
    pub u_degree: usize,
    pub v_degree: usize,
}

impl GeomConvertBSplineToBezierSurface {
    pub fn new(surface_id: u32, u_deg: usize, v_deg: usize, nb_u: usize, nb_v: usize) -> Self {
        Self {
            surface_id,
            nb_u_patches: nb_u.max(1),
            nb_v_patches: nb_v.max(1),
            u_degree: u_deg.max(1),
            v_degree: v_deg.max(1),
        }
    }

    pub fn nb_u_patches(&self) -> usize { self.nb_u_patches }
    pub fn nb_v_patches(&self) -> usize { self.nb_v_patches }
    pub fn nb_patches(&self) -> usize { self.nb_u_patches * self.nb_v_patches }

    pub fn patch_id(&self, iu: usize, iv: usize) -> Option<u32> {
        if iu >= self.nb_u_patches || iv >= self.nb_v_patches { return None; }
        Some(self.surface_id * 1000 + (iu * self.nb_v_patches + iv) as u32)
    }
}

/// Compose multiple C0 curves into a single B-spline.
/// occt: GeomConvert_CompCurveToBSplineCurve
#[derive(Clone, Debug)]
pub struct GeomConvertCompCurveToBSpline {
    pub curve_ids: Vec<u32>,
    pub tolerance: f64,
    pub is_done: bool,
    pub result_id: u32,
}

impl GeomConvertCompCurveToBSpline {
    pub fn new(tolerance: f64) -> Self {
        Self { curve_ids: Vec::new(), tolerance: tolerance.max(1e-14), is_done: false, result_id: 0 }
    }

    pub fn add(&mut self, curve_id: u32, after: bool) {
        if after { self.curve_ids.push(curve_id); }
        else { self.curve_ids.insert(0, curve_id); }
        self.is_done = false;
    }

    /// Stub build: result id is sum of curve ids.
    pub fn build(&mut self) {
        if self.curve_ids.is_empty() { return; }
        self.result_id = self.curve_ids.iter().sum::<u32>() + 5000;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn result_id(&self) -> Option<u32> { if self.is_done { Some(self.result_id) } else { None } }
    pub fn nb_curves(&self) -> usize { self.curve_ids.len() }
}

/// Approximate a surface by a B-spline surface (stub).
/// occt: GeomConvert_ApproxSurface
#[derive(Clone, Debug)]
pub struct GeomConvertApproxSurface {
    pub surface_id: u32,
    pub tolerance_3d: f64,
    pub continuity_u: usize,
    pub continuity_v: usize,
    pub max_seg_u: usize,
    pub max_seg_v: usize,
    pub max_degree: usize,
    pub is_done: bool,
    pub max_error: f64,
    pub result_id: u32,
}

impl GeomConvertApproxSurface {
    pub fn new(surface_id: u32) -> Self {
        Self {
            surface_id,
            tolerance_3d: 0.001,
            continuity_u: 2,
            continuity_v: 2,
            max_seg_u: 100,
            max_seg_v: 100,
            max_degree: 14,
            is_done: false,
            max_error: 0.0,
            result_id: 0,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance_3d = t.max(1e-10); }
    pub fn set_max_segments(&mut self, nu: usize, nv: usize) {
        self.max_seg_u = nu.max(1);
        self.max_seg_v = nv.max(1);
    }
    pub fn set_max_degree(&mut self, d: usize) { self.max_degree = d.clamp(1, 25); }

    /// Stub perform: mark done if surface_id > 0.
    pub fn perform(&mut self) {
        if self.surface_id > 0 {
            self.max_error = self.tolerance_3d * 0.5;
            self.result_id = self.surface_id + 30000;
            self.is_done = true;
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn max_error(&self) -> f64 { self.max_error }
    pub fn result_id(&self) -> Option<u32> { if self.is_done { Some(self.result_id) } else { None } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bspline_to_bezier_curve_arcs() {
        let c = GeomConvertBSplineToBezierCurve::new(1, 3, 4);
        assert_eq!(c.nb_arcs(), 4);
        assert_eq!(c.nb_poles_per_arc(), 4);
        let poles = c.arc_poles(0).unwrap();
        assert_eq!(poles.len(), 4);
        assert!(c.arc_poles(4).is_none());
    }

    #[test]
    fn bspline_to_bezier_surface_patches() {
        let s = GeomConvertBSplineToBezierSurface::new(2, 3, 3, 4, 2);
        assert_eq!(s.nb_patches(), 8);
        assert!(s.patch_id(0, 0).is_some());
        assert!(s.patch_id(4, 0).is_none());
    }

    #[test]
    fn comp_curve_build() {
        let mut c = GeomConvertCompCurveToBSpline::new(1e-6);
        c.add(10, true);
        c.add(20, true);
        assert_eq!(c.nb_curves(), 2);
        assert!(!c.is_done());
        c.build();
        assert!(c.is_done());
        assert!(c.result_id().is_some());
    }

    #[test]
    fn comp_curve_empty_build() {
        let mut c = GeomConvertCompCurveToBSpline::new(1e-6);
        c.build();
        assert!(!c.is_done());
    }

    #[test]
    fn approx_surface_perform() {
        let mut a = GeomConvertApproxSurface::new(5);
        a.set_tolerance(0.01);
        a.perform();
        assert!(a.is_done());
        assert!(a.max_error() < 0.01);
        assert_eq!(a.result_id(), Some(30005));
    }
}
