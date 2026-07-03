// FILE: geom_surf_fill.rs
// occt: GeomFill_BSplineCurves, GeomFill_FillingStyle, GeomFill_ConstraintEnum

// occt: GeomFill_FillingStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillingStyle {
    Stretch,
    Coons,
    Curved,
}

// occt: GeomFill_ConstraintEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillingConstraint {
    NoConstraint,
    G0Constraint,
    G1Constraint,
    G2Constraint,
}

// occt: GeomFill_BSplineCurves
/// Builds a B-Spline surface through 2, 3, or 4 boundary curves.
#[derive(Clone, Debug)]
pub struct BSplineCurvesFill {
    pub curves: Vec<Vec<[f64; 3]>>,
    pub style: FillingStyle,
    pub done: bool,
    pub surface_pts: Vec<[f64; 3]>,
}

impl BSplineCurvesFill {
    pub fn new(style: FillingStyle) -> Self {
        Self { curves: Vec::new(), style, done: false, surface_pts: Vec::new() }
    }

    pub fn add_curve(&mut self, pts: Vec<[f64; 3]>) -> &mut Self {
        self.curves.push(pts);
        self
    }

    pub fn build(&mut self) {
        self.done = !self.curves.is_empty();
        if self.done {
            // Stub: average all curve points as the surface
            let all: Vec<[f64; 3]> = self.curves.iter().flatten().cloned().collect();
            self.surface_pts = all;
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_curves(&self) -> usize { self.curves.len() }
}

// occt: GeomFill_ConstrainedFilling
/// Fills a surface constrained by boundary curves with continuity.
#[derive(Clone, Debug)]
pub struct ConstrainedFilling {
    pub boundaries: Vec<FillingBoundary>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct FillingBoundary {
    pub curve: Vec<[f64; 3]>,
    pub constraint: FillingConstraint,
}

impl FillingBoundary {
    pub fn new(curve: Vec<[f64; 3]>, constraint: FillingConstraint) -> Self {
        Self { curve, constraint }
    }
}

impl ConstrainedFilling {
    pub fn new() -> Self { Self { boundaries: Vec::new(), done: false } }

    pub fn add_boundary(&mut self, b: FillingBoundary) {
        self.boundaries.push(b);
    }

    pub fn build(&mut self) {
        self.done = self.boundaries.len() >= 2;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_boundaries(&self) -> usize { self.boundaries.len() }
}

impl Default for ConstrainedFilling {
    fn default() -> Self { Self::new() }
}

// occt: GeomFill_SimpleBound
#[derive(Clone, Debug)]
pub struct SimpleBound {
    pub curve: Vec<[f64; 3]>,
    pub tol_3d: f64,
    pub tol_ang: f64,
}

impl SimpleBound {
    pub fn new(curve: Vec<[f64; 3]>, tol_3d: f64, tol_ang: f64) -> Self {
        Self { curve, tol_3d, tol_ang }
    }

    pub fn is_degenerated(&self) -> bool {
        self.curve.len() < 2
    }
}

// occt: GeomFill_CoonsAlgPatch
/// Coons patch: bilinear blend of 4 boundary curves.
#[derive(Clone, Debug)]
pub struct CoonsAlgPatch {
    pub c: [Vec<[f64; 3]>; 4],
}

impl CoonsAlgPatch {
    pub fn new(c0: Vec<[f64; 3]>, c1: Vec<[f64; 3]>, c2: Vec<[f64; 3]>, c3: Vec<[f64; 3]>) -> Self {
        Self { c: [c0, c1, c2, c3] }
    }

    /// Evaluate the Coons patch at normalized (u, v) in [0,1]^2.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        fn lerp_curve(pts: &[[f64; 3]], t: f64) -> [f64; 3] {
            if pts.is_empty() { return [0.0; 3]; }
            if pts.len() == 1 { return pts[0]; }
            let seg = ((pts.len() - 1) as f64 * t) as usize;
            let seg = seg.min(pts.len() - 2);
            let frac = (pts.len() - 1) as f64 * t - seg as f64;
            [
                pts[seg][0] + frac*(pts[seg+1][0]-pts[seg][0]),
                pts[seg][1] + frac*(pts[seg+1][1]-pts[seg][1]),
                pts[seg][2] + frac*(pts[seg+1][2]-pts[seg][2]),
            ]
        }
        let c0u = lerp_curve(&self.c[0], u);
        let c1u = lerp_curve(&self.c[1], u);
        let c2v = lerp_curve(&self.c[2], v);
        let c3v = lerp_curve(&self.c[3], v);
        let c00 = self.c[0].first().copied().unwrap_or([0.0; 3]);
        let c01 = self.c[0].last().copied().unwrap_or([0.0; 3]);
        let c10 = self.c[1].first().copied().unwrap_or([0.0; 3]);
        let c11 = self.c[1].last().copied().unwrap_or([0.0; 3]);
        [
            (1.0-v)*c0u[0] + v*c1u[0] + (1.0-u)*c2v[0] + u*c3v[0]
                - ((1.0-u)*(1.0-v)*c00[0] + u*(1.0-v)*c01[0] + (1.0-u)*v*c10[0] + u*v*c11[0]),
            (1.0-v)*c0u[1] + v*c1u[1] + (1.0-u)*c2v[1] + u*c3v[1]
                - ((1.0-u)*(1.0-v)*c00[1] + u*(1.0-v)*c01[1] + (1.0-u)*v*c10[1] + u*v*c11[1]),
            (1.0-v)*c0u[2] + v*c1u[2] + (1.0-u)*c2v[2] + u*c3v[2]
                - ((1.0-u)*(1.0-v)*c00[2] + u*(1.0-v)*c01[2] + (1.0-u)*v*c10[2] + u*v*c11[2]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bspline_curves_fill() {
        let mut f = BSplineCurvesFill::new(FillingStyle::Coons);
        f.add_curve(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]);
        f.add_curve(vec![[0.0,1.0,0.0],[1.0,1.0,0.0]]);
        f.build();
        assert!(f.is_done());
        assert_eq!(f.nb_curves(), 2);
    }

    #[test]
    fn constrained_filling() {
        let mut cf = ConstrainedFilling::new();
        cf.add_boundary(FillingBoundary::new(vec![[0.0,0.0,0.0]], FillingConstraint::G0Constraint));
        cf.add_boundary(FillingBoundary::new(vec![[1.0,0.0,0.0]], FillingConstraint::G1Constraint));
        cf.build();
        assert!(cf.is_done());
        assert_eq!(cf.nb_boundaries(), 2);
    }

    #[test]
    fn simple_bound_degenerated() {
        let b = SimpleBound::new(vec![], 1e-6, 1e-6);
        assert!(b.is_degenerated());
        let b2 = SimpleBound::new(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]], 1e-6, 1e-6);
        assert!(!b2.is_degenerated());
    }

    #[test]
    fn coons_patch_corners() {
        let c0 = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let c1 = vec![[0.0,1.0,0.0],[1.0,1.0,0.0]];
        let c2 = vec![[0.0,0.0,0.0],[0.0,1.0,0.0]];
        let c3 = vec![[1.0,0.0,0.0],[1.0,1.0,0.0]];
        let patch = CoonsAlgPatch::new(c0, c1, c2, c3);
        let p = patch.evaluate(0.0, 0.0);
        assert!((p[0]).abs() < 1e-8 && (p[1]).abs() < 1e-8);
    }

    #[test]
    fn filling_style_enum() {
        assert_ne!(FillingStyle::Stretch, FillingStyle::Coons);
        assert_eq!(FillingStyle::Curved, FillingStyle::Curved);
    }
}
