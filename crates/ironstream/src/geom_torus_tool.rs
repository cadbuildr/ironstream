// FILE: geom_torus_tool.rs
// occt-ref: gp_Torus // (extended), BRepPrimAPI_MakeTorus analysis, Geom_ToroidalSurface (extended)

// occt-ref: TorusParameters
#[derive(Clone, Copy, Debug)]
pub struct TorusParams {
    pub major_radius: f64,
    pub minor_radius: f64,
    pub center: [f64; 3],
    pub axis: [f64; 3],
}

impl TorusParams {
    pub fn new(center: [f64; 3], axis: [f64; 3], major: f64, minor: f64) -> Self {
        let axis = normalize3(axis);
        Self { major_radius: major, minor_radius: minor, center, axis }
    }

    pub fn volume(&self) -> f64 {
        2.0 * std::f64::consts::PI * std::f64::consts::PI * self.major_radius * self.minor_radius * self.minor_radius
    }

    pub fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * std::f64::consts::PI * self.major_radius * self.minor_radius
    }

    /// Point on the torus at angles (u, v) in radians.
    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        // In local XZ plane: standard torus parameterization, then rotate to axis
        let r = self.major_radius + self.minor_radius * v.cos();
        let local = [r * u.cos(), r * u.sin(), self.minor_radius * v.sin()];
        // Rotate local to world (align Z→axis)
        let (x_dir, y_dir) = frame_from_axis(self.axis);
        [
            self.center[0] + local[0]*x_dir[0] + local[1]*y_dir[0] + local[2]*self.axis[0],
            self.center[1] + local[0]*x_dir[1] + local[1]*y_dir[1] + local[2]*self.axis[1],
            self.center[2] + local[0]*x_dir[2] + local[1]*y_dir[2] + local[2]*self.axis[2],
        ]
    }

    /// Discretize the torus into nu × nv points.
    pub fn discretize(&self, nu: usize, nv: usize) -> Vec<[f64; 3]> {
        let pi2 = 2.0 * std::f64::consts::PI;
        let mut pts = Vec::with_capacity(nu * nv);
        for i in 0..nu {
            let u = pi2 * i as f64 / nu as f64;
            for j in 0..nv {
                let v = pi2 * j as f64 / nv as f64;
                pts.push(self.point_at(u, v));
            }
        }
        pts
    }

    /// Check if a point is inside the torus solid.
    pub fn contains_point(&self, p: [f64; 3]) -> bool {
        let d = sub3(p, self.center);
        let (x_dir, y_dir) = frame_from_axis(self.axis);
        let px = dot3(d, x_dir);
        let py = dot3(d, y_dir);
        let pz = dot3(d, self.axis);
        let dist_to_circle = ((px*px+py*py).sqrt() - self.major_radius).powi(2) + pz*pz;
        dist_to_circle < self.minor_radius * self.minor_radius
    }
}

// occt-ref: BRepPrimAPI_MakeTorus // (analysis of torus sections)
#[derive(Clone, Debug)]
pub struct TorusSections {
    pub params: TorusParams,
    pub u1: f64,
    pub u2: f64,
}

impl TorusSections {
    pub fn full(params: TorusParams) -> Self {
        Self { params, u1: 0.0, u2: 2.0 * std::f64::consts::PI }
    }

    pub fn partial(params: TorusParams, angle1_deg: f64, angle2_deg: f64) -> Self {
        let pi = std::f64::consts::PI;
        Self { params, u1: angle1_deg * pi / 180.0, u2: angle2_deg * pi / 180.0 }
    }

    pub fn angle_deg(&self) -> f64 {
        (self.u2 - self.u1).abs() * 180.0 / std::f64::consts::PI
    }

    pub fn is_full(&self) -> bool {
        (self.u2 - self.u1 - 2.0 * std::f64::consts::PI).abs() < 1e-6
    }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [0.0, 0.0, 1.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn frame_from_axis(axis: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    // Choose a reference vector not parallel to axis, then Gram-Schmidt.
    let perp = if axis[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    // Project perp onto the plane perpendicular to axis.
    let d = dot3(perp, axis);
    let projected = [perp[0]-d*axis[0], perp[1]-d*axis[1], perp[2]-d*axis[2]];
    let x_dir = normalize3(projected);
    let y_dir = normalize3(cross3(axis, x_dir));
    (x_dir, y_dir)
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn torus_volume() {
        let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
        let expected = 2.0 * std::f64::consts::PI.powi(2) * 3.0;
        assert!((t.volume() - expected).abs() < 1e-8);
    }

    #[test]
    fn torus_surface_area() {
        let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
        let expected = 4.0 * std::f64::consts::PI.powi(2) * 3.0 * 1.0;
        assert!((t.surface_area() - expected).abs() < 1e-8);
    }

    #[test]
    fn torus_point_on_surface() {
        let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
        let p = t.point_at(0.0, 0.0);
        // At u=0,v=0: x = major+minor, y=0, z=0
        assert!((p[0] - 4.0).abs() < 1e-8);
        assert!((p[1]).abs() < 1e-8);
    }

    #[test]
    fn torus_contains_point() {
        let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
        assert!(t.contains_point([3.0, 0.0, 0.0]));
        assert!(!t.contains_point([0.0, 0.0, 0.0]));
    }

    #[test]
    fn torus_sections() {
        let params = TorusParams::new([0.0;3], [0.0,0.0,1.0], 2.0, 0.5);
        let full = TorusSections::full(params);
        assert!(full.is_full());
        let partial = TorusSections::partial(params, 0.0, 90.0);
        assert!(!partial.is_full());
        assert!((partial.angle_deg() - 90.0).abs() < 1e-6);
    }
}
