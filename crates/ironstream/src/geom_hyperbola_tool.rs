// FILE: geom_hyperbola_tool.rs
// occt: Geom_Hyperbola, GCE2d_MakeHyperbola, ElCLib hyperbola params

use std::f64::consts::PI;

/// 3D Hyperbola geometry.
#[derive(Clone, Debug)]
pub struct Hyperbola3d {
    pub center: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
}

impl Hyperbola3d {
    pub fn new(center: [f64; 3], x_axis: [f64; 3], y_axis: [f64; 3], major: f64, minor: f64) -> Self {
        Self { center, x_axis: normalize(x_axis), y_axis: normalize(y_axis), major_radius: major, minor_radius: minor }
    }

    pub fn standard(major: f64, minor: f64) -> Self {
        Self::new([0.0;3], [1.0,0.0,0.0], [0.0,1.0,0.0], major, minor)
    }

    /// Point on right branch at parameter t (hyperbolic parametrization).
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let x = self.major_radius * t.cosh();
        let y = self.minor_radius * t.sinh();
        add(self.center, add(scale(x, self.x_axis), scale(y, self.y_axis)))
    }

    /// Point on left (conjugate) branch.
    pub fn conjugate_point_at(&self, t: f64) -> [f64; 3] {
        let x = -self.major_radius * t.cosh();
        let y = self.minor_radius * t.sinh();
        add(self.center, add(scale(x, self.x_axis), scale(y, self.y_axis)))
    }

    pub fn eccentricity(&self) -> f64 {
        (self.major_radius * self.major_radius + self.minor_radius * self.minor_radius).sqrt() / self.major_radius
    }

    pub fn focal_distance(&self) -> f64 {
        (self.major_radius * self.major_radius + self.minor_radius * self.minor_radius).sqrt()
    }

    pub fn foci(&self) -> ([f64; 3], [f64; 3]) {
        let c = self.focal_distance();
        let f1 = add(self.center, scale(c, self.x_axis));
        let f2 = add(self.center, scale(-c, self.x_axis));
        (f1, f2)
    }

    pub fn directrix_distance(&self) -> f64 {
        self.major_radius / self.eccentricity()
    }

    pub fn asymptote_slopes(&self) -> (f64, f64) {
        let slope = self.minor_radius / self.major_radius;
        (slope, -slope)
    }

    /// Tangent direction at parameter t.
    pub fn tangent_at(&self, t: f64) -> [f64; 3] {
        let tx = self.major_radius * t.sinh();
        let ty = self.minor_radius * t.cosh();
        normalize(add(scale(tx, self.x_axis), scale(ty, self.y_axis)))
    }

    /// Arc length approximation via Gauss quadrature (4-point) over [t1, t2].
    pub fn arc_length(&self, t1: f64, t2: f64, n: usize) -> f64 {
        let dt = (t2 - t1) / n as f64;
        let mut len = 0.0;
        for i in 0..n {
            let ta = t1 + i as f64 * dt;
            let tb = ta + dt;
            let pa = self.point_at(ta); let pb = self.point_at(tb);
            len += mag(sub(pb, pa));
        }
        len
    }
}

/// 2D Hyperbola (for sketch/profile contexts).
#[derive(Clone, Debug)]
pub struct Hyperbola2d {
    pub center: [f64; 2],
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle: f64,  // rotation of x-axis
}

impl Hyperbola2d {
    pub fn new(center: [f64; 2], major: f64, minor: f64, angle: f64) -> Self {
        Self { center, major_radius: major, minor_radius: minor, angle }
    }

    pub fn point_at(&self, t: f64) -> [f64; 2] {
        let x = self.major_radius * t.cosh();
        let y = self.minor_radius * t.sinh();
        let cos_a = self.angle.cos(); let sin_a = self.angle.sin();
        [
            self.center[0] + x * cos_a - y * sin_a,
            self.center[1] + x * sin_a + y * cos_a,
        ]
    }

    pub fn eccentricity(&self) -> f64 {
        (1.0 + (self.minor_radius / self.major_radius).powi(2)).sqrt()
    }

    pub fn contains_point_approx(&self, p: [f64; 2], tol: f64) -> bool {
        // Map to local frame and check hyperbola equation |x^2/a^2 - y^2/b^2 - 1| < tol
        let cos_a = self.angle.cos(); let sin_a = self.angle.sin();
        let dx = p[0] - self.center[0]; let dy = p[1] - self.center[1];
        let lx = dx * cos_a + dy * sin_a;
        let ly = -dx * sin_a + dy * cos_a;
        let val = (lx / self.major_radius).powi(2) - (ly / self.minor_radius).powi(2) - 1.0;
        val.abs() < tol
    }
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn scale(s: f64, v: [f64; 3]) -> [f64; 3] { [s*v[0],s*v[1],s*v[2]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14{[v[0]/m,v[1]/m,v[2]/m]}else{[1.0,0.0,0.0]} }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyperbola3d_point() {
        let h = Hyperbola3d::standard(2.0, 1.0);
        let p = h.point_at(0.0);
        assert!((p[0] - 2.0).abs() < 1e-10);
        assert!(p[1].abs() < 1e-10);
    }

    #[test]
    fn hyperbola3d_eccentricity() {
        let h = Hyperbola3d::standard(3.0, 4.0);
        assert!(h.eccentricity() > 1.0);
        assert!((h.eccentricity() - 5.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn hyperbola3d_foci() {
        let h = Hyperbola3d::standard(3.0, 4.0);
        let (f1, f2) = h.foci();
        assert!((f1[0] - 5.0).abs() < 1e-10);
        assert!((f2[0] + 5.0).abs() < 1e-10);
    }

    #[test]
    fn hyperbola3d_arc_length_positive() {
        let h = Hyperbola3d::standard(2.0, 1.0);
        let l = h.arc_length(0.0, 1.0, 100);
        assert!(l > 0.0);
    }

    #[test]
    fn hyperbola2d_point_on_curve() {
        let h = Hyperbola2d::new([0.0,0.0], 2.0, 1.0, 0.0);
        let p = h.point_at(0.0);
        assert!(h.contains_point_approx(p, 1e-8));
    }
}
