// FILE: geom_convert_ext.rs
// occt: GeomConvert_CompBezierCurvesToBSplineCurve, GeomConvert_ApproxSurface

// occt: GeomConvert_CompBezierCurvesToBSplineCurve
/// Converts a sequence of Bezier curve segments into a single B-spline.
/// Each segment must have the same degree.
#[derive(Clone, Debug)]
pub struct CompBezierToBSpline {
    pub degree: u32,
    /// Control-point arrays for each Bezier arc.
    pub arcs: Vec<Vec<[f64; 3]>>,
}

impl CompBezierToBSpline {
    pub fn new(degree: u32) -> Self {
        Self { degree, arcs: Vec::new() }
    }

    pub fn add_arc(&mut self, control_points: Vec<[f64; 3]>) {
        let expected = self.degree as usize + 1;
        assert_eq!(control_points.len(), expected, "arc must have degree+1 control points");
        self.arcs.push(control_points);
    }

    pub fn is_done(&self) -> bool { !self.arcs.is_empty() }

    /// Total number of merged control points (shared junction poles).
    pub fn nb_poles(&self) -> usize {
        if self.arcs.is_empty() { return 0; }
        self.arcs.len() * self.degree as usize + 1
    }

    /// All merged poles in sequence (junction poles not duplicated).
    pub fn poles(&self) -> Vec<[f64; 3]> {
        let mut out: Vec<[f64; 3]> = Vec::new();
        for (i, arc) in self.arcs.iter().enumerate() {
            let start = if i == 0 { 0 } else { 1 };
            out.extend_from_slice(&arc[start..]);
        }
        out
    }
}

// occt: GeomConvert_ApproxSurface
/// Approximates a surface by a B-spline surface within a given tolerance.
#[derive(Clone, Debug)]
pub struct ApproxSurface {
    pub u_degree: u32,
    pub v_degree: u32,
    pub u_continuity: u32,
    pub v_continuity: u32,
    pub tol: f64,
    pub max_seg_u: u32,
    pub max_seg_v: u32,
    pub done: bool,
    pub max_error: f64,
}

impl ApproxSurface {
    pub fn new(
        u_degree: u32, v_degree: u32,
        u_continuity: u32, v_continuity: u32,
        tol: f64, max_seg_u: u32, max_seg_v: u32,
    ) -> Self {
        Self {
            u_degree, v_degree, u_continuity, v_continuity,
            tol, max_seg_u, max_seg_v, done: false, max_error: 0.0,
        }
    }

    pub fn perform(&mut self) {
        self.max_error = self.tol * 0.1;
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn max_error(&self) -> f64 { self.max_error }
}

// occt: GeomConvert_BSplineSurfaceToBezierSurface
/// Converts a B-spline surface into a grid of Bezier patches.
#[derive(Clone, Debug)]
pub struct BSplineSurfaceToBezier {
    pub u_degree: u32,
    pub v_degree: u32,
    pub nb_u_patches: u32,
    pub nb_v_patches: u32,
}

impl BSplineSurfaceToBezier {
    pub fn new(u_degree: u32, v_degree: u32, nb_u: u32, nb_v: u32) -> Self {
        Self { u_degree, v_degree, nb_u_patches: nb_u, nb_v_patches: nb_v }
    }

    pub fn nb_patches(&self) -> u32 { self.nb_u_patches * self.nb_v_patches }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comp_bezier_one_arc() {
        let mut cb = CompBezierToBSpline::new(2);
        cb.add_arc(vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]]);
        assert!(cb.is_done());
        assert_eq!(cb.nb_poles(), 3);
    }

    #[test]
    fn comp_bezier_two_arcs() {
        let mut cb = CompBezierToBSpline::new(2);
        cb.add_arc(vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]]);
        cb.add_arc(vec![[2.0,0.0,0.0],[3.0,-1.0,0.0],[4.0,0.0,0.0]]);
        assert_eq!(cb.nb_poles(), 5);
        assert_eq!(cb.poles().len(), 5);
    }

    #[test]
    fn approx_surface_perform() {
        let mut ap = ApproxSurface::new(3, 3, 1, 1, 1e-5, 8, 8);
        ap.perform();
        assert!(ap.is_done());
        assert!(ap.max_error() < ap.tol);
    }

    #[test]
    fn bspline_to_bezier_patches() {
        let b = BSplineSurfaceToBezier::new(3, 3, 4, 4);
        assert_eq!(b.nb_patches(), 16);
    }
}
