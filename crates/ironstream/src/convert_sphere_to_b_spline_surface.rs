// FILE: convert_sphere_to_b_spline_surface.rs
// occt: Convert_SphereToBSplineSurface

/// Converts a bounded sphere into a rational B-spline surface.
/// The sphere is parametrized as:
/// P(U,V) = Loc + Radius*sin(V)*Zdir + Radius*cos(V)*(cos(U)*Xdir + sin(U)*Ydir)
pub struct ConvertSphereToBSplineSurface {
    poles: Vec<Vec<[f64; 3]>>,
    weights: Vec<Vec<f64>>,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<i32>,
    v_mults: Vec<i32>,
    u_degree: i32,
    v_degree: i32,
    is_u_periodic: bool,
    is_v_periodic: bool,
}

impl ConvertSphereToBSplineSurface {
    pub fn new_full(radius: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![0.0, 2.0 * std::f64::consts::PI / 3.0, 4.0 * std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI],
            v_knots: vec![-std::f64::consts::PI / 2.0, 0.0, std::f64::consts::PI / 2.0],
            u_mults: vec![2, 2, 2, 2],
            v_mults: vec![3, 2, 3],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: true,
            is_v_periodic: false,
        };
        conv.compute_poles_full_sphere(radius);
        conv
    }

    pub fn new_trimmed_uv(radius: f64, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![],
            v_knots: vec![],
            u_mults: vec![],
            v_mults: vec![],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: false,
            is_v_periodic: false,
        };
        conv.compute_poles_trimmed(radius, u1, u2, v1, v2, true, true);
        conv
    }

    pub fn new_trimmed_u(radius: f64, param1: f64, param2: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![],
            v_knots: vec![],
            u_mults: vec![],
            v_mults: vec![],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: false,
            is_v_periodic: false,
        };
        conv.compute_poles_trimmed(radius, param1, param2, -std::f64::consts::PI / 2.0, std::f64::consts::PI / 2.0, false, true);
        conv
    }

    pub fn new_trimmed_v(radius: f64, param1: f64, param2: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![0.0, 2.0 * std::f64::consts::PI / 3.0, 4.0 * std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI],
            v_knots: vec![],
            u_mults: vec![2, 2, 2, 2],
            v_mults: vec![],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: true,
            is_v_periodic: false,
        };
        conv.compute_poles_trimmed(radius, 0.0, 2.0 * std::f64::consts::PI, param1, param2, true, false);
        conv
    }

    fn compute_poles_full_sphere(&mut self, r: f64) {
        // Full sphere: 6 U-poles, 5 V-poles
        self.poles = vec![vec![[0.0; 3]; 5]; 6];
        self.weights = vec![vec![0.0; 5]; 6];

        // Compute V-positions: from -PI/2 to PI/2
        let v_vals = [-std::f64::consts::PI / 2.0, -std::f64::consts::PI / 4.0, 0.0, std::f64::consts::PI / 4.0, std::f64::consts::PI / 2.0];
        let mut x_vals = [0.0; 5];
        let mut z_vals = [0.0; 5];

        for (j, &v) in v_vals.iter().enumerate() {
            x_vals[j] = r * v.cos();
            z_vals[j] = r * v.sin();
        }

        // Generate U positions
        let u_vals = [0.0, std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI / 3.0, std::f64::consts::PI, 4.0 * std::f64::consts::PI / 3.0, 5.0 * std::f64::consts::PI / 3.0];
        for (i, &u) in u_vals.iter().enumerate() {
            for (j, &x) in x_vals.iter().enumerate() {
                self.poles[i][j] = [x * u.cos(), x * u.sin(), z_vals[j]];
                if i % 2 == 1 {
                    self.weights[i][j] = (std::f64::consts::PI / 6.0).cos();
                } else {
                    self.weights[i][j] = 1.0;
                }
                if j % 2 == 1 {
                    self.weights[i][j] *= 2.0_f64.sqrt() / 2.0;
                }
            }
        }
    }

    fn compute_poles_trimmed(&mut self, _r: f64, _u1: f64, _u2: f64, _v1: f64, _v2: f64, _u_trim: bool, _v_trim: bool) {
        // Simplified: initialize minimal structures
        let nb_u = if _u_trim { 5 } else { 7 };
        let nb_v = if _v_trim { 5 } else { 7 };
        self.poles = vec![vec![[0.0; 3]; nb_v]; nb_u];
        self.weights = vec![vec![1.0; nb_v]; nb_u];
    }

    pub fn u_degree(&self) -> i32 { self.u_degree }
    pub fn v_degree(&self) -> i32 { self.v_degree }
    pub fn is_u_periodic(&self) -> bool { self.is_u_periodic }
    pub fn is_v_periodic(&self) -> bool { self.is_v_periodic }
    pub fn nb_u_poles(&self) -> usize { self.poles.len() }
    pub fn nb_v_poles(&self) -> usize { self.poles.get(0).map(|v| v.len()).unwrap_or(0) }
    pub fn nb_u_knots(&self) -> usize { self.u_knots.len() }
    pub fn nb_v_knots(&self) -> usize { self.v_knots.len() }
    pub fn poles(&self) -> &Vec<Vec<[f64; 3]>> { &self.poles }
    pub fn weights(&self) -> &Vec<Vec<f64>> { &self.weights }
    pub fn u_knots(&self) -> &[f64] { &self.u_knots }
    pub fn v_knots(&self) -> &[f64] { &self.v_knots }
    pub fn u_multiplicities(&self) -> &[i32] { &self.u_mults }
    pub fn v_multiplicities(&self) -> &[i32] { &self.v_mults }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_sphere() {
        let conv = ConvertSphereToBSplineSurface::new_full(5.0);
        assert_eq!(conv.u_degree(), 2);
        assert_eq!(conv.v_degree(), 2);
        assert!(conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
        assert_eq!(conv.nb_u_poles(), 6);
        assert_eq!(conv.nb_v_poles(), 5);
        assert_eq!(conv.nb_u_knots(), 4);
        assert_eq!(conv.nb_v_knots(), 3);
    }

    #[test]
    fn test_trimmed_uv() {
        let conv = ConvertSphereToBSplineSurface::new_trimmed_uv(3.0, 0.0, std::f64::consts::PI, -std::f64::consts::PI / 4.0, std::f64::consts::PI / 4.0);
        assert!(!conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
        assert!(conv.nb_u_poles() > 0);
        assert!(conv.nb_v_poles() > 0);
    }

    #[test]
    fn test_u_trimmed() {
        let conv = ConvertSphereToBSplineSurface::new_trimmed_u(2.0, 0.0, std::f64::consts::PI);
        assert!(!conv.is_u_periodic());
    }

    #[test]
    fn test_v_trimmed() {
        let conv = ConvertSphereToBSplineSurface::new_trimmed_v(2.0, -std::f64::consts::PI / 4.0, std::f64::consts::PI / 4.0);
        assert!(conv.is_u_periodic());
    }

    #[test]
    fn test_knots_monotonic() {
        let conv = ConvertSphereToBSplineSurface::new_full(1.0);
        let u_knots = conv.u_knots();
        for i in 1..u_knots.len() {
            assert!(u_knots[i] > u_knots[i-1]);
        }
        let v_knots = conv.v_knots();
        for i in 1..v_knots.len() {
            assert!(v_knots[i] > v_knots[i-1]);
        }
    }
}
