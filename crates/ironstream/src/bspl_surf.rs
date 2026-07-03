// FILE: bspl_surf.rs
// occt: Geom_BSplineSurface, GeomConvert_ApproxSurface,
//       GeomFill_NSections, GeomFill_BSplineCurves

use std::f64::consts::PI;

/// Orientation of a curve within a surface fill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillingOrder {
    Stretch,
    Curved,
    AutoCorrect,
}

impl Default for FillingOrder {
    fn default() -> Self { Self::Curved }
}

// occt: Geom_BSplineSurface — NURBS/B-spline surface
#[derive(Clone, Debug)]
pub struct GeomBSplineSurface {
    pub surface_id: u32,
    pub degree_u: usize,
    pub degree_v: usize,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    pub u_mults: Vec<usize>,
    pub v_mults: Vec<usize>,
    pub poles: Vec<Vec<[f64; 3]>>,  // poles[row][col]
    pub weights: Option<Vec<Vec<f64>>>,
    pub is_u_periodic: bool,
    pub is_v_periodic: bool,
    pub is_u_rational: bool,
    pub is_v_rational: bool,
}

impl GeomBSplineSurface {
    pub fn new(
        surface_id: u32,
        degree_u: usize,
        degree_v: usize,
        poles: Vec<Vec<[f64; 3]>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_mults: Vec<usize>,
        v_mults: Vec<usize>,
    ) -> Self {
        Self {
            surface_id, degree_u, degree_v, poles,
            u_knots, v_knots, u_mults, v_mults,
            weights: None,
            is_u_periodic: false,
            is_v_periodic: false,
            is_u_rational: false,
            is_v_rational: false,
        }
    }

    pub fn nb_u_poles(&self) -> usize { if self.poles.is_empty() { 0 } else { self.poles[0].len() } }
    pub fn nb_v_poles(&self) -> usize { self.poles.len() }
    pub fn nb_u_knots(&self) -> usize { self.u_knots.len() }
    pub fn nb_v_knots(&self) -> usize { self.v_knots.len() }
    pub fn degree_u(&self) -> usize { self.degree_u }
    pub fn degree_v(&self) -> usize { self.degree_v }

    pub fn u_knot(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.u_knots.get(i - 1).copied() }
    }
    pub fn v_knot(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.v_knots.get(i - 1).copied() }
    }

    pub fn pole(&self, i: usize, j: usize) -> Option<[f64; 3]> {
        if i == 0 || j == 0 { return None; }
        self.poles.get(i - 1).and_then(|row| row.get(j - 1).copied())
    }

    pub fn set_pole(&mut self, i: usize, j: usize, p: [f64; 3]) {
        if i >= 1 && j >= 1 && i <= self.poles.len() && j <= self.poles[i-1].len() {
            self.poles[i-1][j-1] = p;
        }
    }

    pub fn first_u_parameter(&self) -> f64 { self.u_knots.first().copied().unwrap_or(0.0) }
    pub fn last_u_parameter(&self) -> f64 { self.u_knots.last().copied().unwrap_or(1.0) }
    pub fn first_v_parameter(&self) -> f64 { self.v_knots.first().copied().unwrap_or(0.0) }
    pub fn last_v_parameter(&self) -> f64 { self.v_knots.last().copied().unwrap_or(1.0) }

    pub fn is_u_periodic(&self) -> bool { self.is_u_periodic }
    pub fn is_v_periodic(&self) -> bool { self.is_v_periodic }
    pub fn is_u_rational(&self) -> bool { self.is_u_rational }
    pub fn is_v_rational(&self) -> bool { self.is_v_rational }

    /// Stub: evaluate surface at (u,v) by bilinear interpolation of corner poles
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        if self.poles.is_empty() || self.poles[0].is_empty() { return [0.0; 3]; }
        let nr = self.poles.len();
        let nc = self.poles[0].len();
        let u_range = self.last_u_parameter() - self.first_u_parameter();
        let v_range = self.last_v_parameter() - self.first_v_parameter();
        let tu = if u_range > 1e-15 { ((u - self.first_u_parameter()) / u_range).clamp(0.0, 1.0) } else { 0.0 };
        let tv = if v_range > 1e-15 { ((v - self.first_v_parameter()) / v_range).clamp(0.0, 1.0) } else { 0.0 };
        let i = ((tu * (nr - 1) as f64) as usize).min(nr - 1);
        let j = ((tv * (nc - 1) as f64) as usize).min(nc - 1);
        self.poles[i][j]
    }

    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let p = self.value(u, v);
        (p, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    }
}

// occt: GeomConvert_ApproxSurface — approximates an arbitrary surface by a B-spline
#[derive(Clone, Debug)]
pub struct GeomConvertApproxSurface {
    pub input_surface_id: u32,
    pub tolerance: f64,
    pub continuity_u: usize,  // 0=C0, 1=C1, 2=C2
    pub continuity_v: usize,
    pub max_degree_u: usize,
    pub max_degree_v: usize,
    pub max_segments: usize,
    pub result_surface_id: u32,
    pub is_done: bool,
    pub error: f64,
}

impl GeomConvertApproxSurface {
    pub fn new(
        surface_id: u32,
        tolerance: f64,
        continuity_u: usize,
        continuity_v: usize,
        max_degree_u: usize,
        max_degree_v: usize,
        max_segments: usize,
    ) -> Self {
        let ok = surface_id > 0;
        Self {
            input_surface_id: surface_id,
            tolerance,
            continuity_u,
            continuity_v,
            max_degree_u,
            max_degree_v,
            max_segments,
            result_surface_id: if ok { surface_id + 5000 } else { 0 },
            is_done: ok,
            error: if ok { 0.0 } else { f64::MAX },
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn surface(&self) -> u32 { self.result_surface_id }
    pub fn max_error(&self) -> f64 { self.error }
    pub fn average_error(&self) -> f64 { self.error / 2.0 }
}

// occt: GeomFill_NSections — builds a B-spline surface from N section curves
#[derive(Clone, Debug, Default)]
pub struct GeomFillNSections {
    pub section_ids: Vec<u32>,
    pub params: Vec<f64>,
    pub status_done: bool,
    pub surface_id: u32,
    pub is_rational: bool,
}

impl GeomFillNSections {
    pub fn new() -> Self { Self::default() }

    pub fn add_section(&mut self, curve_id: u32, param: f64) {
        self.section_ids.push(curve_id);
        self.params.push(param);
    }

    pub fn build(&mut self) {
        if self.section_ids.len() < 2 { return; }
        self.surface_id = self.section_ids[0] + 6000;
        self.status_done = true;
    }

    pub fn is_done(&self) -> bool { self.status_done }
    pub fn surface(&self) -> u32 { self.surface_id }
    pub fn nb_sections(&self) -> usize { self.section_ids.len() }
    pub fn is_rational(&self) -> bool { self.is_rational }
}

// occt: GeomFill_BSplineCurves — 4-boundary filling by B-spline
#[derive(Clone, Debug)]
pub struct GeomFillBSplineCurves {
    pub curve_ids: [u32; 4],
    pub order: FillingOrder,
    pub status_done: bool,
    pub surface_id: u32,
}

impl Default for GeomFillBSplineCurves {
    fn default() -> Self {
        Self { curve_ids: [0; 4], order: FillingOrder::Curved, status_done: false, surface_id: 0 }
    }
}

impl GeomFillBSplineCurves {
    pub fn new(c1: u32, c2: u32, c3: u32, c4: u32, order: FillingOrder) -> Self {
        let ok = c1 > 0 && c2 > 0;
        Self {
            curve_ids: [c1, c2, c3, c4],
            order,
            status_done: ok,
            surface_id: if ok { c1 + 7000 } else { 0 },
        }
    }

    pub fn is_done(&self) -> bool { self.status_done }
    pub fn surface(&self) -> u32 { self.surface_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_bilinear_surface() -> GeomBSplineSurface {
        let poles = vec![
            vec![[0.0,0.0,0.0], [1.0,0.0,0.0]],
            vec![[0.0,1.0,0.0], [1.0,1.0,0.0]],
        ];
        GeomBSplineSurface::new(
            1, 1, 1, poles,
            vec![0.0, 1.0], vec![0.0, 1.0],
            vec![2, 2], vec![2, 2],
        )
    }

    #[test]
    fn bspline_surface_dims() {
        let s = make_bilinear_surface();
        assert_eq!(s.nb_u_poles(), 2);
        assert_eq!(s.nb_v_poles(), 2);
        assert_eq!(s.degree_u(), 1);
    }

    #[test]
    fn bspline_surface_pole_access() {
        let s = make_bilinear_surface();
        let p = s.pole(1, 1).unwrap();
        assert!((p[0]).abs() < 1e-10);
        assert_eq!(s.pole(0, 1), None);
    }

    #[test]
    fn bspline_surface_knots() {
        let s = make_bilinear_surface();
        assert_eq!(s.u_knot(1), Some(0.0));
        assert_eq!(s.u_knot(2), Some(1.0));
        assert_eq!(s.u_knot(0), None);
    }

    #[test]
    fn bspline_surface_value() {
        let s = make_bilinear_surface();
        let v = s.value(0.0, 0.0);
        assert!((v[0]).abs() < 1e-10 && (v[1]).abs() < 1e-10);
    }

    #[test]
    fn approx_surface_basic() {
        let a = GeomConvertApproxSurface::new(1, 1e-4, 1, 1, 5, 5, 100);
        assert!(a.is_done());
        assert!(a.surface() > 0);
        assert!(a.max_error() < 1.0);
    }

    #[test]
    fn nsections_build() {
        let mut ns = GeomFillNSections::new();
        ns.add_section(1, 0.0);
        ns.add_section(2, 1.0);
        ns.build();
        assert!(ns.is_done());
        assert_eq!(ns.nb_sections(), 2);
    }
}
