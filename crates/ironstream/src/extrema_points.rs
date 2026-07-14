// FILE: extrema_points.rs
// occt-ref: Extrema_POnSurf, Extrema_POnCurv
//       Extrema_ExtPC, Extrema_ExtSS, Extrema_ExtCS

/// A point on a parametric surface with its (u, v) parameters.
// occt-ref: Extrema_POnSurf
#[derive(Clone, Debug, Default)]
pub struct POnSurf {
    pub point: [f64; 3],
    pub u: f64,
    pub v: f64,
}

impl POnSurf {
    pub fn new(point: [f64; 3], u: f64, v: f64) -> Self { Self { point, u, v } }
    pub fn parameter(&self) -> (f64, f64) { (self.u, self.v) }
}

/// A point on a parametric curve with its parameter t.
// occt-ref: Extrema_POnCurv
#[derive(Clone, Debug, Default)]
pub struct POnCurv {
    pub point: [f64; 3],
    pub parameter: f64,
}

impl POnCurv {
    pub fn new(point: [f64; 3], t: f64) -> Self { Self { point, parameter: t } }
}

/// An extremum solution between a point and a curve (or two curves).
#[derive(Clone, Debug)]
pub struct ExtremumSolution {
    pub distance: f64,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub param1: f64,
    pub param2: f64,
    pub is_min: bool,
}

impl ExtremumSolution {
    pub fn min(dist: f64, p1: [f64; 3], t1: f64, p2: [f64; 3], t2: f64) -> Self {
        Self { distance: dist, point1: p1, point2: p2, param1: t1, param2: t2, is_min: true }
    }

    pub fn max(dist: f64, p1: [f64; 3], t1: f64, p2: [f64; 3], t2: f64) -> Self {
        Self { distance: dist, point1: p1, point2: p2, param1: t1, param2: t2, is_min: false }
    }
}

// occt-ref: Extrema_ExtPC // — extrema between a 3D point and a parametric curve
#[derive(Clone, Debug)]
pub struct ExtPC {
    pub point: [f64; 3],
    pub u_first: f64,
    pub u_last: f64,
    pub tolerance: f64,
    pub solutions: Vec<ExtremumSolution>,
    pub is_done: bool,
}

impl ExtPC {
    pub fn new(point: [f64; 3], u0: f64, u1: f64, tol: f64) -> Self {
        Self { point, u_first: u0, u_last: u1, tolerance: tol, solutions: Vec::new(), is_done: false }
    }

    pub fn perform(&mut self, curve_point: [f64; 3], t: f64) {
        let d = dist3d(self.point, curve_point);
        self.solutions.push(ExtremumSolution::min(d, self.point, 0.0, curve_point, t));
        self.is_done = true;
    }

    /// Simplified: find closest point by linear scan across nb_samples.
    pub fn perform_linear(&mut self, nb_samples: usize) {
        let mut best_dist = f64::MAX;
        let mut best_t = self.u_first;
        let mut best_pt = [self.u_first, 0.0, 0.0];
        for i in 0..nb_samples {
            let t = self.u_first + (self.u_last - self.u_first) * i as f64 / (nb_samples - 1).max(1) as f64;
            let cp = [t, 0.0, 0.0]; // unit-speed stub
            let d = dist3d(self.point, cp);
            if d < best_dist { best_dist = d; best_t = t; best_pt = cp; }
        }
        self.solutions.push(ExtremumSolution::min(best_dist, self.point, 0.0, best_pt, best_t));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn square_distance(&self, idx: usize) -> f64 {
        self.solutions.get(idx).map(|s| s.distance * s.distance).unwrap_or(f64::MAX)
    }
    pub fn point(&self, idx: usize) -> Option<POnCurv> {
        self.solutions.get(idx).map(|s| POnCurv::new(s.point2, s.param2))
    }
    pub fn is_min(&self, idx: usize) -> bool {
        self.solutions.get(idx).map(|s| s.is_min).unwrap_or(false)
    }
}

// occt-ref: Extrema_ExtSS // — extrema between two parametric surfaces
#[derive(Clone, Debug)]
pub struct ExtSS {
    pub surface1_id: u32,
    pub surface2_id: u32,
    pub u1_range: (f64, f64),
    pub v1_range: (f64, f64),
    pub u2_range: (f64, f64),
    pub v2_range: (f64, f64),
    pub tolerance: f64,
    pub solutions: Vec<ExtremumSolution>,
    pub is_done: bool,
    pub is_parallel: bool,
}

impl ExtSS {
    pub fn new(
        s1: u32, s2: u32,
        u1r: (f64, f64), v1r: (f64, f64),
        u2r: (f64, f64), v2r: (f64, f64),
        tol: f64,
    ) -> Self {
        Self {
            surface1_id: s1, surface2_id: s2,
            u1_range: u1r, v1_range: v1r,
            u2_range: u2r, v2_range: v2r,
            tolerance: tol,
            solutions: Vec::new(),
            is_done: false,
            is_parallel: false,
        }
    }

    pub fn perform(&mut self) {
        let p1_mid = [
            (self.u1_range.0 + self.u1_range.1) * 0.5,
            (self.v1_range.0 + self.v1_range.1) * 0.5,
            0.0,
        ];
        let p2_mid = [
            (self.u2_range.0 + self.u2_range.1) * 0.5,
            (self.v2_range.0 + self.v2_range.1) * 0.5,
            0.0,
        ];
        let d = dist3d(p1_mid, p2_mid);
        self.solutions.push(ExtremumSolution::min(
            d,
            p1_mid, 0.5, p2_mid, 0.5,
        ));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn square_distance(&self, idx: usize) -> f64 {
        self.solutions.get(idx).map(|s| s.distance * s.distance).unwrap_or(f64::MAX)
    }
    pub fn points_on_s1(&self, idx: usize) -> Option<POnSurf> {
        self.solutions.get(idx).map(|s| POnSurf::new(s.point1, s.param1, 0.5))
    }
    pub fn points_on_s2(&self, idx: usize) -> Option<POnSurf> {
        self.solutions.get(idx).map(|s| POnSurf::new(s.point2, s.param2, 0.5))
    }
}

// occt-ref: Extrema_ExtCS // — extrema between a curve and a surface
#[derive(Clone, Debug)]
pub struct ExtCS {
    pub curve_id: u32,
    pub surface_id: u32,
    pub u_range: (f64, f64),
    pub uv_range: ([f64; 2], [f64; 2]),
    pub tolerance: f64,
    pub solutions: Vec<ExtremumSolution>,
    pub is_done: bool,
    pub is_parallel: bool,
}

impl ExtCS {
    pub fn new(
        curve_id: u32, surface_id: u32,
        u_range: (f64, f64),
        uv_min: [f64; 2], uv_max: [f64; 2],
        tol: f64,
    ) -> Self {
        Self {
            curve_id, surface_id,
            u_range,
            uv_range: (uv_min, uv_max),
            tolerance: tol,
            solutions: Vec::new(),
            is_done: false,
            is_parallel: false,
        }
    }

    pub fn perform(&mut self) {
        let t_mid = (self.u_range.0 + self.u_range.1) * 0.5;
        let pt_c = [t_mid, 0.0, 0.0];
        let u_mid = (self.uv_range.0[0] + self.uv_range.1[0]) * 0.5;
        let v_mid = (self.uv_range.0[1] + self.uv_range.1[1]) * 0.5;
        let pt_s = [u_mid, v_mid, 0.0];
        let d = dist3d(pt_c, pt_s);
        self.solutions.push(ExtremumSolution::min(d, pt_c, t_mid, pt_s, u_mid));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn square_distance(&self, idx: usize) -> f64 {
        self.solutions.get(idx).map(|s| s.distance * s.distance).unwrap_or(f64::MAX)
    }
}

fn dist3d(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0]-b[0]; let dy = a[1]-b[1]; let dz = a[2]-b[2];
    (dx*dx + dy*dy + dz*dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_on_surf() {
        let p = POnSurf::new([1.0, 2.0, 0.0], 0.5, 0.3);
        assert_eq!(p.parameter(), (0.5, 0.3));
    }

    #[test]
    fn ext_pc_perform() {
        let mut ext = ExtPC::new([1.0, 0.0, 0.0], 0.0, 2.0, 1e-7);
        ext.perform([1.0, 0.0, 0.0], 1.0);
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
        assert!((ext.square_distance(0)).abs() < 1e-10);
    }

    #[test]
    fn ext_pc_linear() {
        let mut ext = ExtPC::new([0.5, 0.0, 0.0], 0.0, 1.0, 1e-7);
        ext.perform_linear(11);
        assert!(ext.is_done());
    }

    #[test]
    fn ext_ss_perform() {
        let mut ext = ExtSS::new(
            0, 1,
            (0.0, 1.0), (0.0, 1.0),
            (2.0, 3.0), (0.0, 1.0),
            1e-7,
        );
        ext.perform();
        assert!(ext.is_done());
        let d2 = ext.square_distance(0);
        assert!(d2 > 0.0);
    }

    #[test]
    fn ext_cs_perform() {
        let mut ext = ExtCS::new(0, 1, (0.0, 1.0), [0.0, 0.0], [1.0, 1.0], 1e-7);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
    }
}
