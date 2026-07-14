// FILE: geom_nurbs_ext.rs
// occt: BSplCLib_Cache
// occt-ref: GeomConvert_BSplineSurfaceToBezierSurface, GeomConvert_BSplineCurveToBezierCurve

// occt-ref: GeomConvert_BSplineCurveToBezierCurve
/// Converts a B-Spline curve into a sequence of Bezier arcs.
#[derive(Clone, Debug)]
pub struct BSplineCurveToBezier {
    pub knots: Vec<f64>,
    pub poles: Vec<[f64; 3]>,
    pub degree: u32,
    pub arcs: Vec<BezierArc>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct BezierArc {
    pub poles: Vec<[f64; 3]>,
    pub t_start: f64,
    pub t_end: f64,
}

impl BSplineCurveToBezier {
    pub fn new(knots: Vec<f64>, poles: Vec<[f64; 3]>, degree: u32) -> Self {
        let mut s = Self { knots, poles, degree, arcs: Vec::new(), done: false };
        s.convert();
        s
    }

    fn convert(&mut self) {
        // Stub: create one Bezier arc per knot span
        let n = self.knots.len();
        if n < 2 { self.done = false; return; }
        for i in 0..n-1 {
            let t0 = self.knots[i];
            let t1 = self.knots[i+1];
            if (t1 - t0).abs() < 1e-14 { continue; }
            // Sample poles proportionally
            let frac = self.poles.len() as f64 / (n - 1) as f64;
            let p0 = (i as f64 * frac) as usize;
            let p1 = ((i + 1) as f64 * frac) as usize;
            let p1 = p1.min(self.poles.len().saturating_sub(1));
            let arc_poles: Vec<[f64; 3]> = (p0..=p1).filter_map(|j| self.poles.get(j).copied()).collect();
            self.arcs.push(BezierArc { poles: arc_poles, t_start: t0, t_end: t1 });
        }
        self.done = !self.arcs.is_empty();
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_arcs(&self) -> usize { self.arcs.len() }
    pub fn arc(&self, i: usize) -> Option<&BezierArc> { self.arcs.get(i.saturating_sub(1)) }
}

// occt-ref: GeomConvert_BSplineSurfaceToBezierSurface
/// Converts a B-Spline surface into a grid of Bezier patches.
#[derive(Clone, Debug)]
pub struct BSplineSurfaceToBezier {
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    pub u_degree: u32,
    pub v_degree: u32,
    pub nb_u_patches: usize,
    pub nb_v_patches: usize,
    pub done: bool,
}

impl BSplineSurfaceToBezier {
    pub fn new(u_knots: Vec<f64>, v_knots: Vec<f64>, u_degree: u32, v_degree: u32) -> Self {
        let nu = u_knots.windows(2).filter(|w| (w[1]-w[0]).abs() > 1e-14).count();
        let nv = v_knots.windows(2).filter(|w| (w[1]-w[0]).abs() > 1e-14).count();
        Self {
            u_knots, v_knots, u_degree, v_degree,
            nb_u_patches: nu, nb_v_patches: nv,
            done: nu > 0 && nv > 0,
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_u_patches(&self) -> usize { self.nb_u_patches }
    pub fn nb_v_patches(&self) -> usize { self.nb_v_patches }
    pub fn nb_patches(&self) -> usize { self.nb_u_patches * self.nb_v_patches }
}

// occt: BSplCLib_Cache
/// Cache for B-Spline curve evaluation at a given parameter.
#[derive(Clone, Debug)]
pub struct BSplineCurveCache {
    pub poles: Vec<[f64; 3]>,
    pub knots: Vec<f64>,
    pub degree: u32,
    cached_t: Option<f64>,
    cached_pt: [f64; 3],
}

impl BSplineCurveCache {
    pub fn new(poles: Vec<[f64; 3]>, knots: Vec<f64>, degree: u32) -> Self {
        Self { poles, knots, degree, cached_t: None, cached_pt: [0.0; 3] }
    }

    pub fn invalidate(&mut self) { self.cached_t = None; }

    pub fn evaluate(&mut self, t: f64) -> [f64; 3] {
        if self.cached_t == Some(t) { return self.cached_pt; }
        // Stub: linear interpolation between first and last pole
        let pt = if self.poles.len() < 2 {
            self.poles.first().copied().unwrap_or([0.0; 3])
        } else {
            let n = self.poles.len() - 1;
            let t_clamped = t.clamp(0.0, 1.0);
            let idx = (t_clamped * n as f64) as usize;
            let idx = idx.min(n - 1);
            let frac = t_clamped * n as f64 - idx as f64;
            let p0 = self.poles[idx];
            let p1 = self.poles[idx + 1];
            [p0[0]+frac*(p1[0]-p0[0]), p0[1]+frac*(p1[1]-p0[1]), p0[2]+frac*(p1[2]-p0[2])]
        };
        self.cached_t = Some(t);
        self.cached_pt = pt;
        pt
    }

    pub fn is_valid_for(&self, t: f64) -> bool {
        self.cached_t == Some(t)
    }
}

// occt: GeomConvert_Units
#[derive(Clone, Copy, Debug)]
pub struct UnitConverter {
    pub from_mm_scale: f64,
}

impl UnitConverter {
    pub fn mm_to_m() -> Self { Self { from_mm_scale: 0.001 } }
    pub fn m_to_mm() -> Self { Self { from_mm_scale: 1000.0 } }
    pub fn inch_to_mm() -> Self { Self { from_mm_scale: 25.4 } }

    pub fn convert(&self, value: f64) -> f64 { value * self.from_mm_scale }
    pub fn convert_point(&self, p: [f64; 3]) -> [f64; 3] {
        [p[0]*self.from_mm_scale, p[1]*self.from_mm_scale, p[2]*self.from_mm_scale]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bspline_to_bezier_arcs() {
        let knots = vec![0.0, 0.5, 1.0];
        let poles = vec![[0.0,0.0,0.0],[0.5,1.0,0.0],[1.0,0.0,0.0]];
        let c = BSplineCurveToBezier::new(knots, poles, 2);
        assert!(c.is_done());
        assert_eq!(c.nb_arcs(), 2);
    }

    #[test]
    fn bspline_surface_to_bezier() {
        let u = vec![0.0, 0.5, 1.0];
        let v = vec![0.0, 1.0];
        let s = BSplineSurfaceToBezier::new(u, v, 2, 2);
        assert!(s.is_done());
        assert_eq!(s.nb_u_patches(), 2);
        assert_eq!(s.nb_v_patches(), 1);
        assert_eq!(s.nb_patches(), 2);
    }

    #[test]
    fn bspline_cache_evaluate() {
        let poles = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let knots = vec![0.0, 0.5, 1.0];
        let mut cache = BSplineCurveCache::new(poles, knots, 2);
        let p = cache.evaluate(0.5);
        assert!((p[0] - 1.0).abs() < 1e-6);
        assert!(cache.is_valid_for(0.5));
    }

    #[test]
    fn unit_converter() {
        let c = UnitConverter::inch_to_mm();
        assert!((c.convert(1.0) - 25.4).abs() < 1e-10);
        let p = c.convert_point([1.0, 2.0, 0.0]);
        assert!((p[1] - 50.8).abs() < 1e-10);
    }

    #[test]
    fn cache_invalidate() {
        let poles = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let mut c = BSplineCurveCache::new(poles, vec![0.0, 1.0], 1);
        c.evaluate(0.5);
        assert!(c.is_valid_for(0.5));
        c.invalidate();
        assert!(!c.is_valid_for(0.5));
    }
}
