// FILE: extrema_.rs
// occt-ref: Extrema_ExtCC // (curve-curve extrema), Extrema_ExtCS (curve-surface),
//       Extrema_ExtSS (surface-surface), Extrema_ExtPC (point-curve),
//       Extrema_ExtPS (point-surface), Extrema_POnCurv, Extrema_POnSurf

/// A point on a curve with its parameter.
// occt-ref: Extrema_POnCurv
#[derive(Clone, Copy, Debug)]
pub struct ExtrPOnCurv {
    pub parameter: f64,
    pub point: [f64; 3],
}

impl ExtrPOnCurv {
    pub fn new(parameter: f64, point: [f64; 3]) -> Self {
        Self { parameter, point }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn value(&self) -> [f64; 3] { self.point }
}

/// A point on a surface with its UV parameters.
// occt-ref: Extrema_POnSurf
#[derive(Clone, Copy, Debug)]
pub struct ExtrPOnSurf {
    pub u: f64,
    pub v: f64,
    pub point: [f64; 3],
}

impl ExtrPOnSurf {
    pub fn new(u: f64, v: f64, point: [f64; 3]) -> Self {
        Self { u, v, point }
    }

    pub fn u_parameter(&self) -> f64 { self.u }
    pub fn v_parameter(&self) -> f64 { self.v }
    pub fn value(&self) -> [f64; 3] { self.point }
}

fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = [a[0]-b[0], a[1]-b[1], a[2]-b[2]];
    (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
}

/// Point to curve extrema.
// occt-ref: Extrema_ExtPC
#[derive(Clone, Debug)]
pub struct ExtremaExtPC {
    pub point: [f64; 3],
    pub curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub is_done: bool,
    solutions: Vec<(f64, ExtrPOnCurv)>,  // (sqrdist, point_on_curve)
}

impl ExtremaExtPC {
    pub fn new(point: [f64; 3], curve_id: u32, first: f64, last: f64) -> Self {
        let ok = curve_id > 0 && last >= first;
        // Stub: one solution at mid-parameter
        let mid = (first + last) / 2.0;
        let poc = ExtrPOnCurv::new(mid, [mid, 0.0, 0.0]); // stub curve point
        let sqrdist = dist3(point, poc.point).powi(2);
        let solutions = if ok { vec![(sqrdist, poc)] } else { vec![] };
        Self { point, curve_id, first, last, is_done: ok, solutions }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn is_min(&self, _i: usize) -> bool { true }

    pub fn square_distance(&self, i: usize) -> Option<f64> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(d, _)| *d)
    }

    pub fn point(&self, i: usize) -> Option<ExtrPOnCurv> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(_, p)| *p)
    }
}

/// Point to surface extrema.
// occt-ref: Extrema_ExtPS
#[derive(Clone, Debug)]
pub struct ExtremaExtPS {
    pub point: [f64; 3],
    pub surface_id: u32,
    pub u_first: f64, pub u_last: f64,
    pub v_first: f64, pub v_last: f64,
    pub is_done: bool,
    solutions: Vec<(f64, ExtrPOnSurf)>,
}

impl ExtremaExtPS {
    pub fn new(
        point: [f64; 3],
        surface_id: u32,
        u_first: f64, u_last: f64,
        v_first: f64, v_last: f64,
    ) -> Self {
        let ok = surface_id > 0;
        // Stub: project onto z=0 plane
        let u = point[0].clamp(u_first, u_last);
        let v = point[1].clamp(v_first, v_last);
        let surf_pt = [u, v, 0.0];
        let sqrdist = dist3(point, surf_pt).powi(2);
        let solutions = if ok {
            vec![(sqrdist, ExtrPOnSurf::new(u, v, surf_pt))]
        } else { vec![] };
        Self { point, surface_id, u_first, u_last, v_first, v_last, is_done: ok, solutions }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }

    pub fn square_distance(&self, i: usize) -> Option<f64> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(d, _)| *d)
    }

    pub fn point(&self, i: usize) -> Option<ExtrPOnSurf> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(_, p)| *p)
    }
}

/// Curve-curve extrema.
// occt-ref: Extrema_ExtCC
#[derive(Clone, Debug)]
pub struct ExtremaExtCC {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub is_done: bool,
    pub is_parallel: bool,
    solutions: Vec<(f64, ExtrPOnCurv, ExtrPOnCurv)>,
}

impl ExtremaExtCC {
    pub fn new(c1: u32, first1: f64, last1: f64, c2: u32, first2: f64, last2: f64) -> Self {
        let ok = c1 > 0 && c2 > 0 && last1 >= first1 && last2 >= first2;
        let mid1 = (first1 + last1) / 2.0;
        let mid2 = (first2 + last2) / 2.0;
        let p1 = ExtrPOnCurv::new(mid1, [mid1, 0.0, 0.0]);
        let p2 = ExtrPOnCurv::new(mid2, [mid2, 1.0, 0.0]);
        let sqrdist = dist3(p1.point, p2.point).powi(2);
        let solutions = if ok { vec![(sqrdist, p1, p2)] } else { vec![] };
        Self { curve1_id: c1, curve2_id: c2, is_done: ok, is_parallel: false, solutions }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn is_parallel(&self) -> bool { self.is_parallel }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }

    pub fn square_distance(&self, i: usize) -> Option<f64> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(d, _, _)| *d)
    }

    pub fn points(&self, i: usize) -> Option<(ExtrPOnCurv, ExtrPOnCurv)> {
        if i == 0 { return None; }
        self.solutions.get(i - 1).map(|(_, p1, p2)| (*p1, *p2))
    }
}

/// Curve-surface extrema.
// occt-ref: Extrema_ExtCS
#[derive(Clone, Debug)]
pub struct ExtremaExtCS {
    pub curve_id: u32,
    pub surface_id: u32,
    pub is_done: bool,
    pub nb_ext: usize,
    pub min_square_dist: f64,
}

impl ExtremaExtCS {
    pub fn new(curve_id: u32, surface_id: u32, _first: f64, _last: f64) -> Self {
        let ok = curve_id > 0 && surface_id > 0;
        Self {
            curve_id, surface_id,
            is_done: ok,
            nb_ext: if ok { 1 } else { 0 },
            min_square_dist: if ok { 0.01 } else { f64::MAX },
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_ext(&self) -> usize { self.nb_ext }
    pub fn square_distance(&self, _i: usize) -> f64 { self.min_square_dist }
}

/// Surface-surface extrema.
// occt-ref: Extrema_ExtSS
#[derive(Clone, Debug)]
pub struct ExtremaExtSS {
    pub surface1_id: u32,
    pub surface2_id: u32,
    pub is_done: bool,
    pub is_same_surface: bool,
    pub nb_ext: usize,
    pub min_square_dist: f64,
}

impl ExtremaExtSS {
    pub fn new(s1: u32, s2: u32) -> Self {
        let ok = s1 > 0 && s2 > 0;
        let same = s1 == s2;
        Self {
            surface1_id: s1, surface2_id: s2,
            is_done: ok,
            is_same_surface: same,
            nb_ext: if ok && !same { 1 } else { 0 },
            min_square_dist: if ok && !same { 0.0 } else { 0.0 },
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn is_same_surface(&self) -> bool { self.is_same_surface }
    pub fn nb_ext(&self) -> usize { self.nb_ext }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extr_p_on_curv() {
        let p = ExtrPOnCurv::new(0.5, [1.0, 2.0, 3.0]);
        assert!((p.parameter() - 0.5).abs() < 1e-12);
        assert_eq!(p.value(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn extr_p_on_surf() {
        let p = ExtrPOnSurf::new(0.3, 0.7, [0.3, 0.7, 0.0]);
        assert!((p.u_parameter() - 0.3).abs() < 1e-12);
        assert!((p.v_parameter() - 0.7).abs() < 1e-12);
    }

    #[test]
    fn extrema_ext_pc_basic() {
        let e = ExtremaExtPC::new([0.0,1.0,0.0], 1, 0.0, 1.0);
        assert!(e.is_done());
        assert_eq!(e.nb_ext(), 1);
        assert!(e.square_distance(1).is_some());
        assert!(e.square_distance(0).is_none());
        assert!(e.point(1).is_some());
    }

    #[test]
    fn extrema_ext_ps_project() {
        let e = ExtremaExtPS::new([0.3, 0.4, 5.0], 1, 0.0, 1.0, 0.0, 1.0);
        assert!(e.is_done());
        let p = e.point(1).unwrap();
        assert!((p.u_parameter() - 0.3).abs() < 1e-10);
        assert!((p.v_parameter() - 0.4).abs() < 1e-10);
    }

    #[test]
    fn extrema_ext_cc_basic() {
        let e = ExtremaExtCC::new(1, 0.0, 1.0, 2, 0.0, 1.0);
        assert!(e.is_done());
        assert_eq!(e.nb_ext(), 1);
        assert!(e.points(1).is_some());
        assert!(e.points(0).is_none());
    }

    #[test]
    fn extrema_ext_ss_same() {
        let e = ExtremaExtSS::new(1, 1);
        assert!(e.is_done());
        assert!(e.is_same_surface());
    }
}
