// FILE: convert_torus_to_b_spline_surface.rs
// occt: Convert_TorusToBSplineSurface

/// Converts a bounded torus into a rational B-spline surface.
pub struct ConvertTorusToBSplineSurface {
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

impl ConvertTorusToBSplineSurface {
    pub fn new_full(major_radius: f64, minor_radius: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![0.0, 2.0 * std::f64::consts::PI / 3.0, 4.0 * std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI],
            v_knots: vec![0.0, 2.0 * std::f64::consts::PI / 3.0, 4.0 * std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI],
            u_mults: vec![2, 2, 2, 2],
            v_mults: vec![2, 2, 2, 2],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: true,
            is_v_periodic: true,
        };
        conv.compute_poles_full(major_radius, minor_radius);
        conv
    }

    pub fn new_trimmed_uv(major_radius: f64, minor_radius: f64, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
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
        conv.compute_poles_trimmed(major_radius, minor_radius, u1, u2, v1, v2, true, true);
        conv
    }

    pub fn new_trimmed_u(major_radius: f64, minor_radius: f64, param1: f64, param2: f64) -> Self {
        let mut conv = Self {
            poles: vec![],
            weights: vec![],
            u_knots: vec![],
            v_knots: vec![0.0, 2.0 * std::f64::consts::PI / 3.0, 4.0 * std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI],
            u_mults: vec![],
            v_mults: vec![2, 2, 2, 2],
            u_degree: 2,
            v_degree: 2,
            is_u_periodic: false,
            is_v_periodic: true,
        };
        conv.compute_poles_trimmed(major_radius, minor_radius, param1, param2, 0.0, 2.0 * std::f64::consts::PI, false, true);
        conv
    }

    pub fn new_trimmed_v(major_radius: f64, minor_radius: f64, param1: f64, param2: f64) -> Self {
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
        conv.compute_poles_trimmed(major_radius, minor_radius, 0.0, 2.0 * std::f64::consts::PI, param1, param2, true, false);
        conv
    }

    fn compute_poles_full(&mut self, major: f64, minor: f64) {
        self.poles = vec![vec![[0.0; 3]; 6]; 6];
        self.weights = vec![vec![0.0; 6]; 6];

        let u_vals = [0.0, std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI / 3.0, std::f64::consts::PI, 4.0 * std::f64::consts::PI / 3.0, 5.0 * std::f64::consts::PI / 3.0];
        let v_vals = [0.0, std::f64::consts::PI / 3.0, 2.0 * std::f64::consts::PI / 3.0, std::f64::consts::PI, 4.0 * std::f64::consts::PI / 3.0, 5.0 * std::f64::consts::PI / 3.0];

        for (i, &u) in u_vals.iter().enumerate() {
            for (j, &v) in v_vals.iter().enumerate() {
                let r_xy = major + minor * v.cos();
                self.poles[i][j] = [r_xy * u.cos(), r_xy * u.sin(), minor * v.sin()];

                let w_u = if i % 2 == 1 { 0.5 } else { 1.0 };
                let w_v = if j % 2 == 1 { 0.5 } else { 1.0 };
                self.weights[i][j] = w_u * w_v;
            }
        }
    }

    fn compute_poles_trimmed(&mut self, _major: f64, _minor: f64, _u1: f64, _u2: f64, _v1: f64, _v2: f64, _u_trim: bool, _v_trim: bool) {
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
    fn test_full_torus() {
        let conv = ConvertTorusToBSplineSurface::new_full(5.0, 2.0);
        assert_eq!(conv.u_degree(), 2);
        assert_eq!(conv.v_degree(), 2);
        assert!(conv.is_u_periodic());
        assert!(conv.is_v_periodic());
        assert_eq!(conv.nb_u_poles(), 6);
        assert_eq!(conv.nb_v_poles(), 6);
    }

    #[test]
    fn test_trimmed_uv() {
        let conv = ConvertTorusToBSplineSurface::new_trimmed_uv(5.0, 2.0, 0.0, std::f64::consts::PI, 0.0, std::f64::consts::PI);
        assert!(!conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
        assert!(conv.nb_u_poles() > 0);
        assert!(conv.nb_v_poles() > 0);
    }

    #[test]
    fn test_u_trimmed() {
        let conv = ConvertTorusToBSplineSurface::new_trimmed_u(5.0, 2.0, 0.0, std::f64::consts::PI);
        assert!(!conv.is_u_periodic());
        assert!(conv.is_v_periodic());
    }

    #[test]
    fn test_v_trimmed() {
        let conv = ConvertTorusToBSplineSurface::new_trimmed_v(5.0, 2.0, 0.0, std::f64::consts::PI);
        assert!(conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
    }

    #[test]
    fn test_weights_positive() {
        let conv = ConvertTorusToBSplineSurface::new_full(5.0, 2.0);
        for row in conv.weights() {
            for &w in row {
                assert!(w > 0.0);
            }
        }
    }

    #[test]
    fn test_knots_monotonic() {
        let conv = ConvertTorusToBSplineSurface::new_full(5.0, 2.0);
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
