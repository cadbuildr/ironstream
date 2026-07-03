// FILE: geom_tools.rs

// occt: GeomTools utilities
pub struct GeomToolsCurveAnalysis;

impl GeomToolsCurveAnalysis {
    pub fn bounds_of_cos(degree: u32) -> f64 {
        degree as f64 + 1.0
    }

    pub fn bound_of_bspline(degree: u32, nb_poles: u32) -> f64 {
        let _ = degree;
        (nb_poles - 1) as f64
    }

    pub fn guess_kind(first: f64, last: f64, is_periodic: bool) -> &'static str {
        if is_periodic {
            "Circle"
        } else if first == 0.0 && last == 1.0 {
            "BSpline"
        } else {
            "Curve"
        }
    }
}

// occt: GeomTools UV bounds
pub struct GeomToolsUVBounds {
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub is_u_closed: bool,
    pub is_v_closed: bool,
}

impl GeomToolsUVBounds {
    pub fn new(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Self {
            u_min,
            u_max,
            v_min,
            v_max,
            is_u_closed: false,
            is_v_closed: false,
        }
    }

    pub fn with_u_closed(mut self) -> Self {
        self.is_u_closed = true;
        self
    }

    pub fn with_v_closed(mut self) -> Self {
        self.is_v_closed = true;
        self
    }

    pub fn u_range(&self) -> f64 {
        self.u_max - self.u_min
    }

    pub fn v_range(&self) -> f64 {
        self.v_max - self.v_min
    }

    pub fn center_u(&self) -> f64 {
        (self.u_min + self.u_max) * 0.5
    }

    pub fn center_v(&self) -> f64 {
        (self.v_min + self.v_max) * 0.5
    }

    pub fn contains_uv(&self, u: f64, v: f64) -> bool {
        u >= self.u_min && u <= self.u_max && v >= self.v_min && v <= self.v_max
    }
}

// occt: GeomTools_CurveSet
pub struct GeomToolsCurveSet {
    curves: Vec<(String, [f64; 2])>,
}

impl GeomToolsCurveSet {
    pub fn new() -> Self {
        Self { curves: Vec::new() }
    }

    pub fn add(&mut self, type_name: String, first: f64, last: f64) -> usize {
        let idx = self.curves.len();
        self.curves.push((type_name, [first, last]));
        idx
    }

    pub fn nb_curves(&self) -> usize {
        self.curves.len()
    }

    pub fn curve_type(&self, idx: usize) -> Option<&str> {
        self.curves.get(idx).map(|(name, _)| name.as_str())
    }

    pub fn curve_range(&self, idx: usize) -> Option<[f64; 2]> {
        self.curves.get(idx).map(|(_, range)| *range)
    }

    pub fn clear(&mut self) {
        self.curves.clear();
    }
}

// occt: GeomTools_SurfaceSet
pub struct GeomToolsSurfaceSet {
    surfaces: Vec<(String, [f64; 4])>,
}

impl GeomToolsSurfaceSet {
    pub fn new() -> Self {
        Self { surfaces: Vec::new() }
    }

    pub fn add(
        &mut self,
        type_name: String,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
    ) -> usize {
        let idx = self.surfaces.len();
        self.surfaces.push((type_name, [u_min, u_max, v_min, v_max]));
        idx
    }

    pub fn nb_surfaces(&self) -> usize {
        self.surfaces.len()
    }

    pub fn surface_type(&self, idx: usize) -> Option<&str> {
        self.surfaces.get(idx).map(|(name, _)| name.as_str())
    }

    pub fn surface_range(&self, idx: usize) -> Option<[f64; 4]> {
        self.surfaces.get(idx).map(|(_, range)| *range)
    }

    pub fn clear(&mut self) {
        self.surfaces.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uv_bounds_range() {
        let b = GeomToolsUVBounds::new(1.0, 5.0, -2.0, 3.0);
        assert_eq!(b.u_range(), 4.0);
        assert_eq!(b.v_range(), 5.0);
    }

    #[test]
    fn uv_bounds_center() {
        let b = GeomToolsUVBounds::new(0.0, 4.0, 2.0, 6.0);
        assert_eq!(b.center_u(), 2.0);
        assert_eq!(b.center_v(), 4.0);
    }

    #[test]
    fn uv_bounds_contains() {
        let b = GeomToolsUVBounds::new(0.0, 1.0, 0.0, 1.0);
        assert!(b.contains_uv(0.5, 0.5));
        assert!(!b.contains_uv(1.5, 0.5));
        assert!(!b.contains_uv(0.5, -0.1));
    }

    #[test]
    fn uv_bounds_closed_flags() {
        let b = GeomToolsUVBounds::new(0.0, 1.0, 0.0, 1.0)
            .with_u_closed()
            .with_v_closed();
        assert!(b.is_u_closed);
        assert!(b.is_v_closed);
    }

    #[test]
    fn curve_set_add_and_get() {
        let mut cs = GeomToolsCurveSet::new();
        let i = cs.add("Line".to_string(), 0.0, 10.0);
        assert_eq!(i, 0);
        assert_eq!(cs.nb_curves(), 1);
        assert_eq!(cs.curve_type(0), Some("Line"));
        let r = cs.curve_range(0).unwrap();
        assert_eq!(r[0], 0.0);
        assert_eq!(r[1], 10.0);
    }

    #[test]
    fn curve_set_out_of_bounds() {
        let cs = GeomToolsCurveSet::new();
        assert!(cs.curve_type(0).is_none());
        assert!(cs.curve_range(0).is_none());
    }

    #[test]
    fn curve_set_clear() {
        let mut cs = GeomToolsCurveSet::new();
        cs.add("Circle".to_string(), 0.0, 6.28);
        cs.clear();
        assert_eq!(cs.nb_curves(), 0);
    }

    #[test]
    fn surface_set_add_and_get() {
        let mut ss = GeomToolsSurfaceSet::new();
        let i = ss.add("Plane".to_string(), 0.0, 1.0, 0.0, 1.0);
        assert_eq!(i, 0);
        assert_eq!(ss.nb_surfaces(), 1);
        assert_eq!(ss.surface_type(0), Some("Plane"));
        let r = ss.surface_range(0).unwrap();
        assert_eq!(r, [0.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn surface_set_multiple_entries() {
        let mut ss = GeomToolsSurfaceSet::new();
        ss.add("Sphere".to_string(), -1.0, 1.0, 0.0, 6.28);
        ss.add("Cylinder".to_string(), 0.0, 5.0, 0.0, 6.28);
        assert_eq!(ss.nb_surfaces(), 2);
        assert_eq!(ss.surface_type(1), Some("Cylinder"));
    }

    #[test]
    fn surface_set_clear() {
        let mut ss = GeomToolsSurfaceSet::new();
        ss.add("Torus".to_string(), 0.0, 1.0, 0.0, 1.0);
        ss.clear();
        assert_eq!(ss.nb_surfaces(), 0);
    }

    #[test]
    fn guess_kind_periodic() {
        assert_eq!(GeomToolsCurveAnalysis::guess_kind(0.0, 6.28, true), "Circle");
    }

    #[test]
    fn guess_kind_bspline() {
        assert_eq!(GeomToolsCurveAnalysis::guess_kind(0.0, 1.0, false), "BSpline");
    }

    #[test]
    fn guess_kind_curve() {
        assert_eq!(GeomToolsCurveAnalysis::guess_kind(0.0, 5.0, false), "Curve");
    }

    #[test]
    fn bounds_of_cos_and_bspline() {
        assert_eq!(GeomToolsCurveAnalysis::bounds_of_cos(3), 4.0);
        assert_eq!(GeomToolsCurveAnalysis::bound_of_bspline(3, 5), 4.0);
    }
}
