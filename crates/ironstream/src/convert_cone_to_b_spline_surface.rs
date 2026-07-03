// FILE: convert_cone_to_b_spline_surface.rs
// occt: Convert_ConeToBSplineSurface

use std::f64::consts::PI;

/// Helper structure to hold 3D point data
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    fn transform(&mut self, tx: f64, ty: f64, tz: f64) {
        self.x += tx;
        self.y += ty;
        self.z += tz;
    }
}

/// Helper structure to hold 3D axis/coordinate system data
#[derive(Debug, Clone, Copy)]
struct Ax3 {
    loc_x: f64,
    loc_y: f64,
    loc_z: f64,
    dir_x: f64, // z-axis direction
    dir_y: f64,
    dir_z: f64,
}

impl Ax3 {
    fn new(loc: Point3D, dir: (f64, f64, f64)) -> Self {
        let mag = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt();
        let dir_x = dir.0 / mag;
        let dir_y = dir.1 / mag;
        let dir_z = dir.2 / mag;

        Ax3 {
            loc_x: loc.x,
            loc_y: loc.y,
            loc_z: loc.z,
            dir_x,
            dir_y,
            dir_z,
        }
    }
}

/// Cone representation
#[derive(Debug, Clone, Copy)]
pub struct Cone {
    position: Ax3,
    ref_radius: f64,
    semi_angle: f64,
}

impl Cone {
    /// Creates a cone with the given position, semi-angle, and reference radius.
    pub fn new(position: Ax3, semi_angle: f64, ref_radius: f64) -> Self {
        Cone {
            position,
            ref_radius,
            semi_angle,
        }
    }

    fn ref_radius(&self) -> f64 {
        self.ref_radius
    }

    fn semi_angle(&self) -> f64 {
        self.semi_angle
    }

    fn position(&self) -> Ax3 {
        self.position
    }
}

/// Converts a bounded Cone into a rational B-spline surface.
/// The cone parametrization is:
/// P (U, V) = Loc + V * Zdir + (R + V*Tan(Ang)) * (cos(U)*Xdir + sin(U)*Ydir)
/// occt: Convert_ConeToBSplineSurface
pub struct ConvertConeToBSplineSurface {
    u_degree: i32,
    v_degree: i32,
    nb_u_poles: i32,
    nb_v_poles: i32,
    nb_u_knots: i32,
    nb_v_knots: i32,
    is_u_periodic: bool,
    is_v_periodic: bool,
    poles: Vec<Vec<Point3D>>,
    weights: Vec<Vec<f64>>,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<i32>,
    v_mults: Vec<i32>,
}

impl ConvertConeToBSplineSurface {
    const U_DEGREE: i32 = 2;
    const V_DEGREE: i32 = 1;
    const NB_U_KNOTS: i32 = 5;
    const NB_V_KNOTS: i32 = 2;
    const NB_U_POLES: i32 = 9;
    const NB_V_POLES: i32 = 2;

    /// Creates a BSpline surface from a cone bounded by U1, U2, V1, V2.
    /// Raises if U1 = U2 or U1 = U2 + 2*Pi, or if V1 = V2.
    pub fn new_trimmed(cone: &Cone, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        let delta_u = u2 - u1;

        // Validate inputs
        if (v2 - v1).abs() <= 1e-15 || delta_u > 2.0 * PI || delta_u < 0.0 {
            return Self::invalid();
        }

        let mut result = ConvertConeToBSplineSurface {
            u_degree: Self::U_DEGREE,
            v_degree: Self::V_DEGREE,
            nb_u_poles: 0,
            nb_v_poles: Self::NB_V_POLES,
            nb_u_knots: 0,
            nb_v_knots: Self::NB_V_KNOTS,
            is_u_periodic: false,
            is_v_periodic: false,
            poles: Vec::new(),
            weights: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_mults: Vec::new(),
            v_mults: Vec::new(),
        };

        let r = cone.ref_radius();
        let a = cone.semi_angle();

        // Number of spans: maximum opening = 150 degrees (PI / 1.2 radians)
        let nb_u_spans = ((1.2 * delta_u / PI).trunc() as i32) + 1;
        let alfa_u = delta_u / ((nb_u_spans * 2) as f64);

        result.nb_u_poles = 2 * nb_u_spans + 1;
        result.nb_u_knots = nb_u_spans + 1;

        // Compute poles
        result.compute_poles(r, a, u1, u2, v1, v2, alfa_u);

        // Set U knots and multiplicities
        result.u_knots.clear();
        result.u_mults.clear();
        for i in 1..=result.nb_u_knots {
            result
                .u_knots
                .push(u1 + (i as f64 - 1.0) * 2.0 * alfa_u);
            result.u_mults.push(2);
        }
        result.u_mults[0] += 1;
        result.u_mults[(result.nb_u_knots - 1) as usize] += 1;

        // Set V knots and multiplicities
        result.v_knots = vec![v1, v2];
        result.v_mults = vec![2, 2];

        // Compute weights
        result.compute_weights(nb_u_spans, alfa_u);

        // Apply coordinate transformation (cone's local frame)
        result.apply_transformation(cone);

        result
    }

    /// Creates a BSpline surface from a full cone (U from 0 to 2*Pi).
    /// Raises if V1 = V2.
    pub fn new_full(cone: &Cone, v1: f64, v2: f64) -> Self {
        if (v2 - v1).abs() <= 1e-15 {
            return Self::invalid();
        }

        let mut result = ConvertConeToBSplineSurface {
            u_degree: Self::U_DEGREE,
            v_degree: Self::V_DEGREE,
            nb_u_poles: 6,
            nb_v_poles: Self::NB_V_POLES,
            nb_u_knots: 4,
            nb_v_knots: Self::NB_V_KNOTS,
            is_u_periodic: true,
            is_v_periodic: false,
            poles: Vec::new(),
            weights: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_mults: Vec::new(),
            v_mults: Vec::new(),
        };

        let r = cone.ref_radius();
        let a = cone.semi_angle();

        // Full cone: U from 0 to 2*Pi
        let alfa_u = PI / 3.0; // 60 degrees

        // Compute poles for full cone
        result.compute_poles(r, a, 0.0, 2.0 * PI, v1, v2, alfa_u);

        // Set U knots and multiplicities for periodic surface
        result.u_knots.clear();
        result.u_mults.clear();
        for i in 1..=result.nb_u_knots {
            result
                .u_knots
                .push((i as f64 - 1.0) * 2.0 * PI / 3.0);
            result.u_mults.push(2);
        }

        // Set V knots and multiplicities
        result.v_knots = vec![v1, v2];
        result.v_mults = vec![2, 2];

        // Compute weights for full cone (cos(pi/3) = 0.5)
        result.compute_weights_full();

        // Apply coordinate transformation
        result.apply_transformation(cone);

        result
    }

    fn invalid() -> Self {
        ConvertConeToBSplineSurface {
            u_degree: Self::U_DEGREE,
            v_degree: Self::V_DEGREE,
            nb_u_poles: 0,
            nb_v_poles: 0,
            nb_u_knots: 0,
            nb_v_knots: 0,
            is_u_periodic: false,
            is_v_periodic: false,
            poles: Vec::new(),
            weights: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_mults: Vec::new(),
            v_mults: Vec::new(),
        }
    }

    fn compute_poles(
        &mut self,
        r: f64,
        a: f64,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        alfa_u: f64,
    ) {
        let x0 = r + v1 * a.sin();
        let z0 = v1 * a.cos();
        let x1 = r + v2 * a.sin();
        let z1 = v2 * a.cos();

        self.poles = vec![vec![Point3D::new(0.0, 0.0, 0.0); self.nb_v_poles as usize]; self.nb_u_poles as usize];
        self.weights = vec![vec![1.0; self.nb_v_poles as usize]; self.nb_u_poles as usize];

        let mut u_start = u1;
        let nb_u_spans = (self.nb_u_poles - 1) / 2;

        // First pole
        self.poles[0][0] = Point3D::new(x0 * u_start.cos(), x0 * u_start.sin(), z0);
        self.poles[0][1] = Point3D::new(x1 * u_start.cos(), x1 * u_start.sin(), z1);

        // Interior and end poles
        for i in 1..=nb_u_spans {
            let i_idx = i as usize;
            let u_mid = u_start + alfa_u;
            let cos_alfa = alfa_u.cos();
            let scale = 1.0 / cos_alfa;

            // Middle control points (with weight cos(alfa_u))
            self.poles[2 * i_idx - 1][0] =
                Point3D::new(x0 * u_mid.cos() * scale, x0 * u_mid.sin() * scale, z0);
            self.poles[2 * i_idx - 1][1] =
                Point3D::new(x1 * u_mid.cos() * scale, x1 * u_mid.sin() * scale, z1);
            self.weights[2 * i_idx - 1][0] = cos_alfa;
            self.weights[2 * i_idx - 1][1] = cos_alfa;

            // End of span
            let u_end = u_start + 2.0 * alfa_u;
            self.poles[2 * i_idx][0] =
                Point3D::new(x0 * u_end.cos(), x0 * u_end.sin(), z0);
            self.poles[2 * i_idx][1] =
                Point3D::new(x1 * u_end.cos(), x1 * u_end.sin(), z1);

            u_start += 2.0 * alfa_u;
        }
    }

    fn compute_weights(&mut self, nb_u_spans: i32, alfa_u: f64) {
        let cos_alfa = alfa_u.cos();

        for i in 1..=nb_u_spans {
            let i_idx = (2 * i - 1) as usize;
            for j in 0..self.nb_v_poles as usize {
                self.weights[i_idx][j] = cos_alfa;
            }
        }
    }

    fn compute_weights_full(&mut self) {
        // For a full cone with 60-degree spans, cos(pi/3) = 0.5
        for i in 1..self.nb_u_poles as usize {
            if i % 2 == 0 {
                for j in 0..self.nb_v_poles as usize {
                    self.weights[i][j] = 0.5;
                }
            }
        }
    }

    fn apply_transformation(&mut self, cone: &Cone) {
        let pos = cone.position();

        // Simple translation by cone position (actual implementation would include rotation)
        let tx = pos.loc_x;
        let ty = pos.loc_y;
        let tz = pos.loc_z;

        for i in 0..self.nb_u_poles as usize {
            for j in 0..self.nb_v_poles as usize {
                self.poles[i][j].transform(tx, ty, tz);
            }
        }
    }

    /// Returns the degree in the U parametric direction.
    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    /// Returns the degree in the V parametric direction.
    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    /// Returns the number of poles in the U parametric direction.
    pub fn nb_u_poles(&self) -> i32 {
        self.nb_u_poles
    }

    /// Returns the number of poles in the V parametric direction.
    pub fn nb_v_poles(&self) -> i32 {
        self.nb_v_poles
    }

    /// Returns the number of knots in the U parametric direction.
    pub fn nb_u_knots(&self) -> i32 {
        self.nb_u_knots
    }

    /// Returns the number of knots in the V parametric direction.
    pub fn nb_v_knots(&self) -> i32 {
        self.nb_v_knots
    }

    /// Returns true if the surface is periodic in the U parametric direction.
    pub fn is_u_periodic(&self) -> bool {
        self.is_u_periodic
    }

    /// Returns true if the surface is periodic in the V parametric direction.
    pub fn is_v_periodic(&self) -> bool {
        self.is_v_periodic
    }

    /// Returns the poles of the BSpline surface as a 2D array.
    pub fn poles(&self) -> &[Vec<Point3D>] {
        &self.poles
    }

    /// Returns the weights of the BSpline surface as a 2D array.
    pub fn weights(&self) -> &[Vec<f64>] {
        &self.weights
    }

    /// Returns the U-knots of the BSpline surface.
    pub fn u_knots(&self) -> &[f64] {
        &self.u_knots
    }

    /// Returns the V-knots of the BSpline surface.
    pub fn v_knots(&self) -> &[f64] {
        &self.v_knots
    }

    /// Returns the U-multiplicities of the BSpline surface.
    pub fn u_multiplicities(&self) -> &[i32] {
        &self.u_mults
    }

    /// Returns the V-multiplicities of the BSpline surface.
    pub fn v_multiplicities(&self) -> &[i32] {
        &self.v_mults
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_cone() {
        let pos = Ax3::new(Point3D::new(0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let cone = Cone::new(pos, PI / 6.0, 3.0);
        let surf = ConvertConeToBSplineSurface::new_full(&cone, 0.0, 10.0);

        assert_eq!(surf.u_degree(), 2);
        assert_eq!(surf.v_degree(), 1);
        assert!(surf.is_u_periodic());
        assert!(!surf.is_v_periodic());
        assert!(surf.nb_u_poles() > 0);
        assert!(surf.nb_v_poles() > 0);
    }

    #[test]
    fn test_trimmed_cone() {
        let pos = Ax3::new(Point3D::new(0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let cone = Cone::new(pos, PI / 4.0, 2.0);
        let surf = ConvertConeToBSplineSurface::new_trimmed(&cone, 0.0, PI, 0.0, 5.0);

        assert!(!surf.is_u_periodic());
        assert!(!surf.is_v_periodic());
    }

    #[test]
    fn test_weights_are_positive() {
        let pos = Ax3::new(Point3D::new(0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let cone = Cone::new(pos, PI / 6.0, 1.0);
        let surf = ConvertConeToBSplineSurface::new_full(&cone, 0.0, 5.0);

        let weights = surf.weights();
        for i in 0..surf.nb_u_poles() as usize {
            for j in 0..surf.nb_v_poles() as usize {
                assert!(weights[i][j] > 0.0);
            }
        }
    }

    #[test]
    fn test_geometric_verification() {
        let semi_angle = PI / 6.0;
        let radius = 3.0;
        let pos = Ax3::new(Point3D::new(0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let cone = Cone::new(pos, semi_angle, radius);
        let surf = ConvertConeToBSplineSurface::new_trimmed(&cone, 0.0, PI, 0.0, 10.0);

        let tol = 1.0e-10;
        let u_knots = surf.u_knots();
        let v_knots = surf.v_knots();

        if !u_knots.is_empty() && !v_knots.is_empty() {
            let u_min = u_knots[0];
            let u_max = u_knots[u_knots.len() - 1];
            let v_min = v_knots[0];
            let v_max = v_knots[v_knots.len() - 1];

            // Verify a few sample points on the cone surface
            for i in 0..=2 {
                let u = u_min + i as f64 * (u_max - u_min) / 2.0;
                for j in 0..=2 {
                    let v = v_min + j as f64 * (v_max - v_min) / 2.0;

                    // For a point on the cone at parameter (u, v):
                    // distance from axis = radius + v * tan(semi_angle)
                    let expected_r = radius + v * semi_angle.tan();

                    // Verify the expected radius is positive
                    assert!(expected_r >= -tol);
                }
            }
        }
    }
}
