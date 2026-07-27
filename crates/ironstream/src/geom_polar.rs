// FILE: geom_polar.rs
// occt: ElSLib // (elementary surface params), ElCLib (elementary curve params), spherical/cylindrical coordinate conversions

/// Convert Cartesian (x,y,z) to spherical (r, theta, phi) in radians.
pub fn cartesian_to_spherical(p: [f64; 3]) -> [f64; 3] {
    let r = (p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt();
    if r < 1e-14 { return [0.0, 0.0, 0.0]; }
    let theta = p[2].clamp(-r, r) / r; // cos(phi)
    let phi_angle = theta.acos();
    let theta_angle = p[1].atan2(p[0]);
    [r, theta_angle, phi_angle]
}

/// Convert spherical (r, theta, phi) to Cartesian (x,y,z).
pub fn spherical_to_cartesian(s: [f64; 3]) -> [f64; 3] {
    let (r, theta, phi) = (s[0], s[1], s[2]);
    [r * phi.sin() * theta.cos(), r * phi.sin() * theta.sin(), r * phi.cos()]
}

/// Convert Cartesian to cylindrical (rho, phi, z).
pub fn cartesian_to_cylindrical(p: [f64; 3]) -> [f64; 3] {
    let rho = (p[0]*p[0]+p[1]*p[1]).sqrt();
    let phi = p[1].atan2(p[0]);
    [rho, phi, p[2]]
}

/// Convert cylindrical (rho, phi, z) to Cartesian.
pub fn cylindrical_to_cartesian(c: [f64; 3]) -> [f64; 3] {
    [c[0] * c[1].cos(), c[0] * c[1].sin(), c[2]]
}

// occt: ElSLib // — elementary surface parameter computations
pub struct ElSLib;

impl ElSLib {
    /// Point on cylinder at (u, v): u = angle, v = height.
    pub fn cylinder_point(radius: f64, u: f64, v: f64) -> [f64; 3] {
        [radius * u.cos(), radius * u.sin(), v]
    }

    /// Point on cone at (u, v): u = angle, v = along axis.
    pub fn cone_point(radius: f64, half_angle: f64, u: f64, v: f64) -> [f64; 3] {
        let r = radius + v * half_angle.tan();
        [r * u.cos(), r * u.sin(), v]
    }

    /// Point on sphere at (u, v): u = longitude, v = latitude.
    pub fn sphere_point(radius: f64, u: f64, v: f64) -> [f64; 3] {
        [radius * v.cos() * u.cos(), radius * v.cos() * u.sin(), radius * v.sin()]
    }

    /// Point on torus at (u, v): u = major angle, v = minor angle.
    pub fn torus_point(major: f64, minor: f64, u: f64, v: f64) -> [f64; 3] {
        let r = major + minor * v.cos();
        [r * u.cos(), r * u.sin(), minor * v.sin()]
    }

    /// U-parameter range for a closed surface.
    pub fn period() -> f64 { 2.0 * std::f64::consts::PI }
}

// occt-ref: ElCLib // — elementary curve parameter computations
pub struct ElCLib;

impl ElCLib {
    /// Point on circle at angle u.
    pub fn circle_point(radius: f64, center: [f64; 3], u: f64) -> [f64; 3] {
        [center[0] + radius * u.cos(), center[1] + radius * u.sin(), center[2]]
    }

    /// Point on line: P = origin + t * direction.
    pub fn line_point(origin: [f64; 3], direction: [f64; 3], t: f64) -> [f64; 3] {
        [origin[0]+t*direction[0], origin[1]+t*direction[1], origin[2]+t*direction[2]]
    }

    /// Ellipse point at parameter u.
    pub fn ellipse_point(center: [f64; 3], major: f64, minor: f64, u: f64) -> [f64; 3] {
        [center[0] + major * u.cos(), center[1] + minor * u.sin(), center[2]]
    }

    /// Invert: find parameter t on line for point p (closest approach).
    pub fn line_parameter(origin: [f64; 3], direction: [f64; 3], p: [f64; 3]) -> f64 {
        let d = [p[0]-origin[0], p[1]-origin[1], p[2]-origin[2]];
        d[0]*direction[0]+d[1]*direction[1]+d[2]*direction[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spherical_round_trip() {
        let p = [1.0, 2.0, 3.0];
        let s = cartesian_to_spherical(p);
        let p2 = spherical_to_cartesian(s);
        assert!((p2[0]-p[0]).abs() < 1e-10);
        assert!((p2[1]-p[1]).abs() < 1e-10);
        assert!((p2[2]-p[2]).abs() < 1e-10);
    }

    #[test]
    fn cylindrical_round_trip() {
        let p = [3.0, 4.0, 5.0];
        let c = cartesian_to_cylindrical(p);
        let p2 = cylindrical_to_cartesian(c);
        assert!((p2[0]-p[0]).abs() < 1e-10);
        assert!((p2[1]-p[1]).abs() < 1e-10);
        assert!((p2[2]-p[2]).abs() < 1e-10);
    }

    #[test]
    fn elsl_cylinder() {
        let p = ElSLib::cylinder_point(1.0, 0.0, 2.0);
        assert!((p[0]-1.0).abs() < 1e-10);
        assert!((p[2]-2.0).abs() < 1e-10);
    }

    #[test]
    fn elsl_sphere() {
        let p = ElSLib::sphere_point(1.0, 0.0, 0.0);
        assert!((p[0]-1.0).abs() < 1e-10);
        assert!((p[2]).abs() < 1e-10);
    }

    #[test]
    fn elcl_circle() {
        let p = ElCLib::circle_point(2.0, [0.0;3], 0.0);
        assert!((p[0]-2.0).abs() < 1e-10);
    }

    #[test]
    fn elcl_line_param() {
        let t = ElCLib::line_parameter([0.0;3], [1.0,0.0,0.0], [3.0,4.0,0.0]);
        assert!((t-3.0).abs() < 1e-10);
    }
}
