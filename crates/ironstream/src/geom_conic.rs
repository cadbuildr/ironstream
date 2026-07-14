// FILE: geom_conic.rs
// occt: Geom_Conic
// occt-ref: Geom_Parabola, Geom_Hyperbola
//       Geom_OffsetCurve (not geom_offset_curve.rs), Geom_TrimmedCurve

use std::f64::consts::PI;

/// Eccentricity tolerance.
const EPS: f64 = 1e-12;

/// Base parameters shared by all conics.
#[derive(Clone, Debug)]
pub struct ConicParams {
    pub location: [f64; 3],
    pub x_dir: [f64; 3],
    pub y_dir: [f64; 3],
    pub z_dir: [f64; 3],
}

impl ConicParams {
    pub fn standard() -> Self {
        Self {
            location: [0.0; 3],
            x_dir: [1.0, 0.0, 0.0],
            y_dir: [0.0, 1.0, 0.0],
            z_dir: [0.0, 0.0, 1.0],
        }
    }

    pub fn with_location(mut self, loc: [f64; 3]) -> Self { self.location = loc; self }
}

/// Parabola: x²= 4py in standard position.
// occt-ref: Geom_Parabola
#[derive(Clone, Debug)]
pub struct GeomParabola {
    pub params: ConicParams,
    pub focal_param: f64,  // 2p (= distance vertex to directrix)
    pub curve_id: u32,
}

impl GeomParabola {
    pub fn new(curve_id: u32, focal: f64) -> Self {
        Self {
            params: ConicParams::standard(),
            focal_param: focal.abs(),
            curve_id,
        }
    }

    pub fn focal_length(&self) -> f64 { self.focal_param / 2.0 }
    pub fn parameter(&self) -> f64 { self.focal_param }
    pub fn directrix_x(&self) -> f64 { -self.focal_param / 2.0 }
    pub fn eccentricity(&self) -> f64 { 1.0 }
    pub fn first_parameter(&self) -> f64 { f64::NEG_INFINITY }
    pub fn last_parameter(&self) -> f64 { f64::INFINITY }
    pub fn is_closed(&self) -> bool { false }
    pub fn is_periodic(&self) -> bool { false }

    /// Point on parabola at parameter t: (t²/2p, t, 0).
    pub fn value(&self, t: f64) -> [f64; 3] {
        let x = t * t / (2.0 * self.focal_param.max(EPS));
        [self.params.location[0] + x, self.params.location[1] + t, self.params.location[2]]
    }

    pub fn d1(&self, t: f64) -> ([f64; 3], [f64; 3]) {
        let p = self.value(t);
        let dxdt = t / self.focal_param.max(EPS);
        (p, [dxdt, 1.0, 0.0])
    }

    /// Curvature radius at parameter t.
    pub fn curvature(&self, t: f64) -> f64 {
        let p = self.focal_param.max(EPS);
        let denom = (1.0 + (t / p) * (t / p)).powi(3).sqrt();
        p * denom
    }
}

/// Hyperbola: x²/a² - y²/b² = 1.
// occt-ref: Geom_Hyperbola
#[derive(Clone, Debug)]
pub struct GeomHyperbola {
    pub params: ConicParams,
    pub major_radius: f64,  // a
    pub minor_radius: f64,  // b
    pub curve_id: u32,
}

impl GeomHyperbola {
    pub fn new(curve_id: u32, major: f64, minor: f64) -> Self {
        assert!(major > 0.0 && minor > 0.0);
        Self {
            params: ConicParams::standard(),
            major_radius: major,
            minor_radius: minor,
            curve_id,
        }
    }

    pub fn major_radius(&self) -> f64 { self.major_radius }
    pub fn minor_radius(&self) -> f64 { self.minor_radius }

    pub fn eccentricity(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        (a * a + b * b).sqrt() / a
    }

    pub fn focal_distance(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        (a * a + b * b).sqrt()
    }

    pub fn directrix1_x(&self) -> f64 {
        let e = self.eccentricity();
        if e < EPS { f64::INFINITY } else { self.major_radius / e }
    }

    pub fn is_closed(&self) -> bool { false }
    pub fn is_periodic(&self) -> bool { false }
    pub fn first_parameter(&self) -> f64 { f64::NEG_INFINITY }
    pub fn last_parameter(&self) -> f64 { f64::INFINITY }

    /// Right branch: x = a·cosh(t), y = b·sinh(t).
    pub fn value(&self, t: f64) -> [f64; 3] {
        let x = self.major_radius * t.cosh();
        let y = self.minor_radius * t.sinh();
        [self.params.location[0] + x, self.params.location[1] + y, self.params.location[2]]
    }

    pub fn d1(&self, t: f64) -> ([f64; 3], [f64; 3]) {
        let p = self.value(t);
        let dx = self.major_radius * t.sinh();
        let dy = self.minor_radius * t.cosh();
        (p, [dx, dy, 0.0])
    }

    /// Conjugate (imaginary) hyperbola: swap a and b.
    pub fn conjugate_branch(&self) -> Self {
        Self::new(self.curve_id + 1, self.minor_radius, self.major_radius)
    }
}

/// Trimmed curve: restricts another curve to [first, last].
// occt-ref: Geom_TrimmedCurve
#[derive(Clone, Debug)]
pub struct GeomTrimmedCurve {
    pub basis_curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub is_same_sense: bool,
    pub trim_id: u32,
}

impl GeomTrimmedCurve {
    pub fn new(basis_id: u32, first: f64, last: f64) -> Self {
        assert!(last > first, "TrimmedCurve: last must be > first");
        Self {
            basis_curve_id: basis_id,
            first,
            last,
            is_same_sense: true,
            trim_id: basis_id + 9000,
        }
    }

    pub fn with_reversed(mut self) -> Self { self.is_same_sense = false; self }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn parameter_range(&self) -> f64 { self.last - self.first }
    pub fn basis_curve_id(&self) -> u32 { self.basis_curve_id }
    pub fn is_closed(&self) -> bool { (self.last - self.first).abs() >= 2.0 * PI - 1e-9 }
    pub fn is_periodic(&self) -> bool { false }
    pub fn is_same_sense(&self) -> bool { self.is_same_sense }

    /// Stub: evaluate basis at parameter, clamp to [first, last].
    pub fn value(&self, t: f64) -> [f64; 3] {
        let tc = t.clamp(self.first, self.last);
        // Stub: parametric point on a unit circle
        [tc.cos(), tc.sin(), 0.0]
    }

    /// Check parameter is in valid range.
    pub fn is_valid_param(&self, t: f64) -> bool {
        t >= self.first - 1e-9 && t <= self.last + 1e-9
    }
}

/// Offset curve: offset of a basis curve by a given distance along normal.
// occt-ref: Geom_OffsetCurve // (extended version)
#[derive(Clone, Debug)]
pub struct GeomOffsetCurveExt {
    pub basis_curve_id: u32,
    pub offset: f64,
    pub direction: [f64; 3],  // offset direction (normal)
    pub offset_id: u32,
    pub is_done: bool,
}

impl GeomOffsetCurveExt {
    pub fn new(basis_id: u32, offset: f64, direction: [f64; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let dir = if len > EPS { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [0.0,0.0,1.0] };
        Self {
            basis_curve_id: basis_id,
            offset,
            direction: dir,
            offset_id: basis_id + 8000,
            is_done: basis_id > 0,
        }
    }

    pub fn basis_curve_id(&self) -> u32 { self.basis_curve_id }
    pub fn offset(&self) -> f64 { self.offset }
    pub fn direction(&self) -> [f64; 3] { self.direction }
    pub fn offset_id(&self) -> u32 { self.offset_id }
    pub fn is_done(&self) -> bool { self.is_done }

    /// Stub offset value: point + offset * direction.
    pub fn value_at(&self, base_point: [f64; 3]) -> [f64; 3] {
        [
            base_point[0] + self.offset * self.direction[0],
            base_point[1] + self.offset * self.direction[1],
            base_point[2] + self.offset * self.direction[2],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parabola_value() {
        let p = GeomParabola::new(1, 2.0); // 2p=2, so p_focal=1
        // at t=0: vertex (0,0,0)
        let v = p.value(0.0);
        assert!(v[0].abs() < 1e-10 && v[1].abs() < 1e-10);
        // at t=2: x=4/(2*2)=1, y=2
        let v2 = p.value(2.0);
        assert!((v2[0] - 1.0).abs() < 1e-10);
        assert!((v2[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn parabola_eccentricity() {
        let p = GeomParabola::new(1, 1.0);
        assert!((p.eccentricity() - 1.0).abs() < 1e-12);
        assert!(!p.is_closed());
        assert!(!p.is_periodic());
    }

    #[test]
    fn hyperbola_eccentricity() {
        let h = GeomHyperbola::new(1, 3.0, 4.0); // a=3, b=4, c=5
        assert!((h.eccentricity() - 5.0/3.0).abs() < 1e-10);
        assert!((h.focal_distance() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn hyperbola_value_at_zero() {
        let h = GeomHyperbola::new(1, 2.0, 1.0);
        // at t=0: x=a=2, y=0
        let v = h.value(0.0);
        assert!((v[0] - 2.0).abs() < 1e-10);
        assert!(v[1].abs() < 1e-10);
    }

    #[test]
    fn trimmed_curve_range() {
        let t = GeomTrimmedCurve::new(5, 0.0, PI);
        assert!((t.parameter_range() - PI).abs() < 1e-10);
        assert!(t.is_valid_param(PI/2.0));
        assert!(!t.is_valid_param(PI + 1.0));
    }

    #[test]
    fn offset_curve_ext() {
        let oc = GeomOffsetCurveExt::new(3, 1.0, [0.0, 1.0, 0.0]);
        let p = oc.value_at([0.0, 0.0, 0.0]);
        assert!((p[1] - 1.0).abs() < 1e-10);
        assert!(oc.is_done());
    }
}
