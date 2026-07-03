// FILE: geom_curvature.rs
// occt: GeomLProp_CLProps, GeomLProp_SLProps, BRepLProp_CLProps, BRepLProp_SLProps

/// Local properties of a curve at parameter t (curvature, tangent, normal).
#[derive(Clone, Debug)]
pub struct CurveLProps {
    pub curvature: f64,
    pub tangent: [f64; 3],
    pub normal: [f64; 3],
    pub binormal: [f64; 3],
}

impl CurveLProps {
    /// Compute from a polyline approximation at index i.
    pub fn from_polyline(pts: &[[f64; 3]], i: usize) -> Option<Self> {
        if pts.len() < 3 || i == 0 || i + 1 >= pts.len() { return None; }
        let p0 = pts[i - 1]; let p1 = pts[i]; let p2 = pts[i + 1];
        let t1 = sub(p1, p0); let t2 = sub(p2, p1);
        let tangent = norm(add(t1, t2));
        let cross = cross3(t1, t2);
        let cross_len = mag(cross);
        let t1_len = mag(t1); let t2_len = mag(t2);
        let curvature = if t1_len * t2_len > 1e-14 {
            2.0 * cross_len / (t1_len * t2_len * (t1_len + t2_len))
        } else { 0.0 };
        let normal = if cross_len > 1e-14 {
            let b = norm(cross);
            cross3(b, tangent)
        } else { perp(tangent) };
        let binormal = norm(cross3(tangent, normal));
        Some(Self { curvature, tangent, normal, binormal })
    }

    pub fn is_straight(&self) -> bool { self.curvature < 1e-14 }
    pub fn radius_of_curvature(&self) -> Option<f64> {
        if self.curvature < 1e-14 { None } else { Some(1.0 / self.curvature) }
    }
}

/// Local properties of a surface at (u, v).
#[derive(Clone, Debug)]
pub struct SurfaceLProps {
    pub first_fundamental: [[f64; 2]; 2],  // E, F, G
    pub normal: [f64; 3],
    pub gaussian_curvature: f64,
    pub mean_curvature: f64,
    pub principal_curvatures: (f64, f64),
}

impl SurfaceLProps {
    /// From a flat patch (zero curvature everywhere).
    pub fn flat(normal: [f64; 3]) -> Self {
        Self {
            first_fundamental: [[1.0, 0.0], [0.0, 1.0]],
            normal,
            gaussian_curvature: 0.0,
            mean_curvature: 0.0,
            principal_curvatures: (0.0, 0.0),
        }
    }

    /// Approximate sphere surface props at (theta, phi) with radius r.
    pub fn sphere(r: f64, theta: f64, phi: f64) -> Self {
        let _ = (theta, phi);
        let k = 1.0 / r;
        let normal = [theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos()];
        Self {
            first_fundamental: [[r * r, 0.0], [0.0, r * r * theta.sin() * theta.sin()]],
            normal,
            gaussian_curvature: k * k,
            mean_curvature: k,
            principal_curvatures: (k, k),
        }
    }

    pub fn is_umbilic(&self) -> bool {
        (self.principal_curvatures.0 - self.principal_curvatures.1).abs() < 1e-12
    }
}

// occt: BRepLProp_CLProps convenience type
pub type BRepCurveLProps = CurveLProps;

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]+b[0], a[1]+b[1], a[2]+b[2]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn norm(v: [f64; 3]) -> [f64; 3] { let m = mag(v); if m > 1e-14 { [v[0]/m,v[1]/m,v[2]/m] } else { [1.0,0.0,0.0] } }
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
fn perp(n: [f64; 3]) -> [f64; 3] {
    let c = if n[0].abs() < 0.9 { [1.0,0.0,0.0] } else { [0.0,1.0,0.0] };
    norm(cross3(n, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_lprops_straight() {
        let pts = [[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0],[3.0,0.0,0.0]];
        let p = CurveLProps::from_polyline(&pts, 1).unwrap();
        assert!(p.is_straight());
        assert!(p.radius_of_curvature().is_none());
    }

    #[test]
    fn surface_lprops_flat() {
        let s = SurfaceLProps::flat([0.0,0.0,1.0]);
        assert!((s.gaussian_curvature).abs() < 1e-12);
        assert!(s.is_umbilic());
    }

    #[test]
    fn surface_lprops_sphere() {
        let s = SurfaceLProps::sphere(2.0, std::f64::consts::FRAC_PI_2, 0.0);
        assert!((s.gaussian_curvature - 0.25).abs() < 1e-8);
        assert!((s.mean_curvature - 0.5).abs() < 1e-8);
    }
}
