// FILE: geom_surface_ext2.rs
// occt-ref: Geom_SurfaceOfLinearExtrusion, Geom_SurfaceOfRevolution
//       Geom_ConicalSurface, Geom_CylindricalSurface, Geom_SphericalSurface

use std::f64::consts::PI;

/// Surface of linear extrusion: sweep a curve along a direction.
#[derive(Clone, Debug)]
pub struct SurfaceOfLinearExtrusion {
    pub basis_points: Vec<[f64; 3]>,
    pub direction: [f64; 3],
}

impl SurfaceOfLinearExtrusion {
    pub fn new(basis: Vec<[f64; 3]>, direction: [f64; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let d = if len > 1e-14 { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [0.0,0.0,1.0] };
        Self { basis_points: basis, direction: d }
    }

    pub fn point_at(&self, u_idx: usize, v: f64) -> Option<[f64; 3]> {
        let p = self.basis_points.get(u_idx)?;
        Some([p[0]+v*self.direction[0], p[1]+v*self.direction[1], p[2]+v*self.direction[2]])
    }

    pub fn nb_u_points(&self) -> usize { self.basis_points.len() }
}

/// Surface of revolution: rotate a curve around an axis.
#[derive(Clone, Debug)]
pub struct SurfaceOfRevolution {
    pub basis_points: Vec<[f64; 3]>,
    pub axis_origin: [f64; 3],
    pub axis_dir: [f64; 3],
}

impl SurfaceOfRevolution {
    pub fn new(basis: Vec<[f64; 3]>, axis_origin: [f64; 3], axis_dir: [f64; 3]) -> Self {
        let len = (axis_dir[0]*axis_dir[0]+axis_dir[1]*axis_dir[1]+axis_dir[2]*axis_dir[2]).sqrt();
        let d = if len > 1e-14 { [axis_dir[0]/len,axis_dir[1]/len,axis_dir[2]/len] } else { [0.0,0.0,1.0] };
        Self { basis_points: basis, axis_origin, axis_dir: d }
    }

    pub fn point_at(&self, u_idx: usize, v: f64) -> Option<[f64; 3]> {
        let p = self.basis_points.get(u_idx)?;
        let d = sub(*p, self.axis_origin);
        let along = dot(d, self.axis_dir);
        let radial = sub(d, scale(along, self.axis_dir));
        let r = mag(radial);
        if r < 1e-14 { return Some(add(self.axis_origin, scale(along, self.axis_dir))); }
        let rx = normalize(radial);
        let ry = cross(self.axis_dir, rx);
        let rotated = add(scale(r * v.cos(), rx), scale(r * v.sin(), ry));
        Some(add(add(self.axis_origin, scale(along, self.axis_dir)), rotated))
    }

    pub fn nb_u_points(&self) -> usize { self.basis_points.len() }
}

// occt-ref: Geom_CylindricalSurface
#[derive(Clone, Debug)]
pub struct CylindricalSurface {
    pub axis_origin: [f64; 3],
    pub axis_dir: [f64; 3],
    pub radius: f64,
    pub ref_dir: [f64; 3],
}

impl CylindricalSurface {
    pub fn new(axis_origin: [f64; 3], axis_dir: [f64; 3], radius: f64) -> Self {
        let az = normalize(axis_dir);
        let ax = perp(az);
        Self { axis_origin, axis_dir: az, radius, ref_dir: ax }
    }

    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        let ry = cross(self.axis_dir, self.ref_dir);
        let x = self.radius * u.cos();
        let y = self.radius * u.sin();
        add(add(self.axis_origin, scale(x, self.ref_dir)), add(scale(y, ry), scale(v, self.axis_dir)))
    }

    pub fn normal_at(&self, u: f64, _v: f64) -> [f64; 3] {
        let ry = cross(self.axis_dir, self.ref_dir);
        normalize(add(scale(u.cos(), self.ref_dir), scale(u.sin(), ry)))
    }
}

// occt-ref: Geom_ConicalSurface
#[derive(Clone, Debug)]
pub struct ConicalSurface {
    pub apex: [f64; 3],
    pub axis_dir: [f64; 3],
    pub half_angle: f64,
    pub ref_dir: [f64; 3],
}

impl ConicalSurface {
    pub fn new(apex: [f64; 3], axis_dir: [f64; 3], half_angle: f64) -> Self {
        let az = normalize(axis_dir);
        let ax = perp(az);
        Self { apex, axis_dir: az, half_angle, ref_dir: ax }
    }

    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        let r = v * self.half_angle.tan();
        let ry = cross(self.axis_dir, self.ref_dir);
        let x = r * u.cos(); let y = r * u.sin();
        add(add(self.apex, scale(v, self.axis_dir)), add(scale(x, self.ref_dir), scale(y, ry)))
    }

    pub fn apex_angle(&self) -> f64 { 2.0 * self.half_angle }
}

// occt-ref: Geom_SphericalSurface
#[derive(Clone, Debug)]
pub struct SphericalSurface {
    pub center: [f64; 3],
    pub radius: f64,
}

impl SphericalSurface {
    pub fn new(center: [f64; 3], radius: f64) -> Self { Self { center, radius } }

    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        // u=longitude, v=latitude
        [
            self.center[0] + self.radius * v.cos() * u.cos(),
            self.center[1] + self.radius * v.cos() * u.sin(),
            self.center[2] + self.radius * v.sin(),
        ]
    }

    pub fn area(&self) -> f64 { 4.0 * PI * self.radius * self.radius }
    pub fn volume(&self) -> f64 { 4.0 / 3.0 * PI * self.radius * self.radius * self.radius }

    pub fn contains_point(&self, p: [f64; 3], tol: f64) -> bool {
        (mag(sub(p, self.center)) - self.radius).abs() < tol
    }
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn scale(s: f64, v: [f64; 3]) -> [f64; 3] { [s*v[0],s*v[1],s*v[2]] }
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14 {[v[0]/m,v[1]/m,v[2]/m]} else {[0.0,0.0,1.0]} }
// occt-note: gp_Ax2(P, V) — automatic "X Direction" for a main direction V.
// Picks the smallest component of V and builds an orthogonal direction so
// that e.g. V = +Z yields X = (1, 0, 0), matching OCCT axis placements.
fn perp(n: [f64; 3]) -> [f64; 3] {
    let (a, b, c) = (n[0], n[1], n[2]);
    let (aa, ba, ca) = (a.abs(), b.abs(), c.abs());
    let d = if ba <= aa && ba <= ca {
        if aa > ca { [-c, 0.0, a] } else { [c, 0.0, -a] }
    } else if aa <= ba && aa <= ca {
        if ba > ca { [0.0, -c, b] } else { [0.0, c, -b] }
    } else if aa > ba {
        [-b, a, 0.0]
    } else {
        [b, -a, 0.0]
    };
    normalize(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_linear_extrusion() {
        let basis = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let s = SurfaceOfLinearExtrusion::new(basis, [0.0,0.0,1.0]);
        let p = s.point_at(0, 3.0).unwrap();
        assert!((p[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn cylindrical_surface_point() {
        let s = CylindricalSurface::new([0.0;3], [0.0,0.0,1.0], 2.0);
        let p = s.point_at(0.0, 0.0);
        assert!((p[0] - 2.0).abs() < 1e-10);
        assert!(p[1].abs() < 1e-10);
    }

    #[test]
    fn spherical_surface_area_volume() {
        let s = SphericalSurface::new([0.0;3], 1.0);
        assert!((s.area() - 4.0 * std::f64::consts::PI).abs() < 1e-10);
        assert!((s.volume() - 4.0 / 3.0 * std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn conical_surface_apex_angle() {
        let s = ConicalSurface::new([0.0;3], [0.0,0.0,1.0], std::f64::consts::FRAC_PI_4);
        assert!((s.apex_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }
}
