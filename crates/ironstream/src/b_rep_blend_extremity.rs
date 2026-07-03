// FILE: b_rep_blend_extremity.rs
// occt: BRepBlend_Extremity

#[derive(Clone, Debug)]
pub struct BRepBlendExtremity {
    pt_x: f64,
    pt_y: f64,
    pt_z: f64,
    tang_x: f64,
    tang_y: f64,
    tang_z: f64,
    param: f64,
    u: f64,
    v: f64,
    tol: f64,
    isvtx: bool,
    hastang: bool,
}

impl BRepBlendExtremity {
    pub fn new() -> Self {
        Self {
            pt_x: 0.0,
            pt_y: 0.0,
            pt_z: 0.0,
            tang_x: 0.0,
            tang_y: 0.0,
            tang_z: 0.0,
            param: 0.0,
            u: 0.0,
            v: 0.0,
            tol: 0.0,
            isvtx: false,
            hastang: false,
        }
    }

    pub fn with_surface(pt_x: f64, pt_y: f64, pt_z: f64, u: f64, v: f64, param: f64, tol: f64) -> Self {
        Self {
            pt_x,
            pt_y,
            pt_z,
            tang_x: 0.0,
            tang_y: 0.0,
            tang_z: 0.0,
            param,
            u,
            v,
            tol,
            isvtx: false,
            hastang: false,
        }
    }

    pub fn with_curve(pt_x: f64, pt_y: f64, pt_z: f64, w: f64, param: f64, tol: f64) -> Self {
        Self {
            pt_x,
            pt_y,
            pt_z,
            tang_x: 0.0,
            tang_y: 0.0,
            tang_z: 0.0,
            param,
            u: w,
            v: 0.0,
            tol,
            isvtx: false,
            hastang: false,
        }
    }

    pub fn set_value(&mut self, pt_x: f64, pt_y: f64, pt_z: f64, u: f64, v: f64, param: f64, tol: f64) {
        self.pt_x = pt_x;
        self.pt_y = pt_y;
        self.pt_z = pt_z;
        self.u = u;
        self.v = v;
        self.param = param;
        self.tol = tol;
    }

    pub fn set_tangent(&mut self, tx: f64, ty: f64, tz: f64) {
        self.tang_x = tx;
        self.tang_y = ty;
        self.tang_z = tz;
        self.hastang = true;
    }

    pub fn value(&self) -> (f64, f64, f64) {
        (self.pt_x, self.pt_y, self.pt_z)
    }

    pub fn tangent(&self) -> (f64, f64, f64) {
        (self.tang_x, self.tang_y, self.tang_z)
    }

    pub fn has_tangent(&self) -> bool {
        self.hastang
    }

    pub fn tolerance(&self) -> f64 {
        self.tol
    }

    pub fn parameters(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    pub fn is_vertex(&self) -> bool {
        self.isvtx
    }

    pub fn parameter(&self) -> f64 {
        self.param
    }

    pub fn set_is_vertex(&mut self, is_vtx: bool) {
        self.isvtx = is_vtx;
    }
}

impl Default for BRepBlendExtremity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ext = BRepBlendExtremity::new();
        assert_eq!(ext.value(), (0.0, 0.0, 0.0));
        assert!(!ext.has_tangent());
        assert!(!ext.is_vertex());
    }

    #[test]
    fn test_with_surface() {
        let ext = BRepBlendExtremity::with_surface(1.0, 2.0, 3.0, 0.5, 0.6, 0.7, 0.01);
        assert_eq!(ext.value(), (1.0, 2.0, 3.0));
        assert_eq!(ext.parameters(), (0.5, 0.6));
        assert_eq!(ext.tolerance(), 0.01);
    }

    #[test]
    fn test_set_tangent() {
        let mut ext = BRepBlendExtremity::new();
        ext.set_tangent(0.1, 0.2, 0.3);
        assert!(ext.has_tangent());
        assert_eq!(ext.tangent(), (0.1, 0.2, 0.3));
    }

    #[test]
    fn test_set_is_vertex() {
        let mut ext = BRepBlendExtremity::new();
        ext.set_is_vertex(true);
        assert!(ext.is_vertex());
    }
}
