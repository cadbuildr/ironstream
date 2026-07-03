// FILE: b_spl_s_lib_cache.rs

//! A cache class for Bezier and B-spline surfaces.
//! Defines all data that can be cached on a span of the surface.
//! The data should be recalculated when going from span to span.

/// Minimal point structure for 3D coordinates
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

/// Minimal vector structure for 3D coordinates
#[derive(Clone, Copy, Debug)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3d { x, y, z }
    }
}

/// Cache parameters for one direction (U or V)
#[derive(Clone, Copy, Debug)]
struct CacheParams {
    knot_index: i32,
    start_knot: f64,
    end_knot: f64,
}

// occt: BSplSLib_Cache
/// Cache for B-spline and Bezier surface evaluation.
/// Caches poles and weights for efficient repeated evaluation over a surface span.
pub struct BsplSlibCache {
    is_rational: bool,
    params_u: CacheParams,
    params_v: CacheParams,
    degree_u: i32,
    degree_v: i32,
    // Cached poles and weights: stored as flat array
    // For non-rational: (deg_u+1) x (deg_v+1) x 3 (x,y,z)
    // For rational: (deg_u+1) x (deg_v+1) x 4 (x,y,z,w)
    poles_weights: std::vec::Vec<f64>,
}

impl BsplSlibCache {
    /// Constructor for caching of the span for the surface
    pub fn new(
        degree_u: i32,
        _periodic_u: bool,
        flat_knots_u: &[f64],
        degree_v: i32,
        _periodic_v: bool,
        flat_knots_v: &[f64],
        _weights: Option<&std::vec::Vec<std::vec::Vec<f64>>>,
    ) -> Self {
        let stride = if _weights.is_some() { 4 } else { 3 };
        let size = (degree_u + 1) as usize * (degree_v + 1) as usize * stride;

        BsplSlibCache {
            is_rational: _weights.is_some(),
            params_u: CacheParams {
                knot_index: -1,
                start_knot: 0.0,
                end_knot: 0.0,
            },
            params_v: CacheParams {
                knot_index: -1,
                start_knot: 0.0,
                end_knot: 0.0,
            },
            degree_u,
            degree_v,
            poles_weights: std::vec![0.0; size],
        }
    }

    /// Verifies validity of the cache using parameters of the point
    pub fn is_cache_valid(&self, parameter_u: f64, parameter_v: f64) -> bool {
        parameter_u >= self.params_u.start_knot
            && parameter_u <= self.params_u.end_knot
            && parameter_v >= self.params_v.start_knot
            && parameter_v <= self.params_v.end_knot
    }

    /// Recomputes the cache data. Does not verify validity of the cache
    pub fn build_cache(
        &mut self,
        parameter_u: f64,
        parameter_v: f64,
        flat_knots_u: &[f64],
        flat_knots_v: &[f64],
        poles: &std::vec::Vec<std::vec::Vec<Point>>,
        weights: Option<&std::vec::Vec<std::vec::Vec<f64>>>,
    ) {
        // Find knot span for U
        let knot_index_u = self.find_knot_span(parameter_u, flat_knots_u, self.degree_u);
        self.params_u.knot_index = knot_index_u;
        self.params_u.start_knot = flat_knots_u[knot_index_u as usize];
        self.params_u.end_knot = flat_knots_u[(knot_index_u + 1) as usize];

        // Find knot span for V
        let knot_index_v = self.find_knot_span(parameter_v, flat_knots_v, self.degree_v);
        self.params_v.knot_index = knot_index_v;
        self.params_v.start_knot = flat_knots_v[knot_index_v as usize];
        self.params_v.end_knot = flat_knots_v[(knot_index_v + 1) as usize];

        // Copy poles and weights for this span into cache
        let stride = if weights.is_some() { 4 } else { 3 };
        let mut idx = 0;

        for i in 0..=(self.degree_u as usize) {
            for j in 0..=(self.degree_v as usize) {
                let pole_row = (knot_index_u as usize - self.degree_u as usize) + i;
                let pole_col = (knot_index_v as usize - self.degree_v as usize) + j;

                if pole_row < poles.len() && pole_col < poles[pole_row].len() {
                    let p = poles[pole_row][pole_col];
                    self.poles_weights[idx] = p.x;
                    self.poles_weights[idx + 1] = p.y;
                    self.poles_weights[idx + 2] = p.z;

                    if let Some(w) = weights {
                        if pole_row < w.len() && pole_col < w[pole_row].len() {
                            self.poles_weights[idx + 3] = w[pole_row][pole_col];
                        }
                    }
                }
                idx += stride;
            }
        }
    }

    /// Calculates the point on the surface for specified parameters
    pub fn d0(&self, u: f64, v: f64) -> Point {
        let local_u = (u - self.params_u.start_knot) / (self.params_u.end_knot - self.params_u.start_knot);
        let local_v = (v - self.params_v.start_knot) / (self.params_v.end_knot - self.params_v.start_knot);
        let local_u_norm = 2.0 * local_u - 1.0;
        let local_v_norm = 2.0 * local_v - 1.0;
        self.d0_local(local_u_norm, local_v_norm)
    }

    /// Calculates the point and first derivatives
    pub fn d1(&self, u: f64, v: f64) -> (Point, Vector3d, Vector3d) {
        let local_u = (u - self.params_u.start_knot) / (self.params_u.end_knot - self.params_u.start_knot);
        let local_v = (v - self.params_v.start_knot) / (self.params_v.end_knot - self.params_v.start_knot);
        let local_u_norm = 2.0 * local_u - 1.0;
        let local_v_norm = 2.0 * local_v - 1.0;
        self.d1_local(local_u_norm, local_v_norm)
    }

    /// Calculates point and second derivatives
    pub fn d2(&self, u: f64, v: f64) -> (Point, Vector3d, Vector3d, Vector3d, Vector3d, Vector3d) {
        let local_u = (u - self.params_u.start_knot) / (self.params_u.end_knot - self.params_u.start_knot);
        let local_v = (v - self.params_v.start_knot) / (self.params_v.end_knot - self.params_v.start_knot);
        let local_u_norm = 2.0 * local_u - 1.0;
        let local_v_norm = 2.0 * local_v - 1.0;
        self.d2_local(local_u_norm, local_v_norm)
    }

    /// Evaluates point at pre-computed local parameters in [-1, 1] range
    pub fn d0_local(&self, local_u: f64, local_v: f64) -> Point {
        let basis_u = self.evaluate_basis_functions(local_u, self.degree_u);
        let basis_v = self.evaluate_basis_functions(local_v, self.degree_v);

        let stride = if self.is_rational { 4 } else { 3 };
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 1.0;

        let mut idx = 0;
        for i in 0..=(self.degree_u as usize) {
            for j in 0..=(self.degree_v as usize) {
                let b = basis_u[i] * basis_v[j];
                x += b * self.poles_weights[idx];
                y += b * self.poles_weights[idx + 1];
                z += b * self.poles_weights[idx + 2];
                if self.is_rational {
                    w += b * (self.poles_weights[idx + 3] - 1.0);
                }
                idx += stride;
            }
        }

        if self.is_rational && w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }

    /// Evaluates point and first derivatives at local parameters
    pub fn d1_local(&self, local_u: f64, local_v: f64) -> (Point, Vector3d, Vector3d) {
        let basis_u = self.evaluate_basis_functions(local_u, self.degree_u);
        let basis_v = self.evaluate_basis_functions(local_v, self.degree_v);
        let basis_du = self.evaluate_basis_derivatives(local_u, self.degree_u);
        let basis_dv = self.evaluate_basis_derivatives(local_v, self.degree_v);

        let stride = if self.is_rational { 4 } else { 3 };
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut dx_du = 0.0;
        let mut dy_du = 0.0;
        let mut dz_du = 0.0;
        let mut dx_dv = 0.0;
        let mut dy_dv = 0.0;
        let mut dz_dv = 0.0;
        let mut w = 1.0;
        let mut dw_du = 0.0;
        let mut dw_dv = 0.0;

        let mut idx = 0;
        for i in 0..=(self.degree_u as usize) {
            for j in 0..=(self.degree_v as usize) {
                let b = basis_u[i] * basis_v[j];
                let db_du = basis_du[i] * basis_v[j];
                let db_dv = basis_u[i] * basis_dv[j];

                x += b * self.poles_weights[idx];
                y += b * self.poles_weights[idx + 1];
                z += b * self.poles_weights[idx + 2];

                dx_du += db_du * self.poles_weights[idx];
                dy_du += db_du * self.poles_weights[idx + 1];
                dz_du += db_du * self.poles_weights[idx + 2];

                dx_dv += db_dv * self.poles_weights[idx];
                dy_dv += db_dv * self.poles_weights[idx + 1];
                dz_dv += db_dv * self.poles_weights[idx + 2];

                if self.is_rational {
                    let wt = self.poles_weights[idx + 3];
                    w += b * (wt - 1.0);
                    dw_du += db_du * (wt - 1.0);
                    dw_dv += db_dv * (wt - 1.0);
                }

                idx += stride;
            }
        }

        let point = if self.is_rational && w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        };

        let tan_u = if self.is_rational && w != 0.0 {
            Vector3d::new(
                (dx_du * w - x * dw_du) / (w * w),
                (dy_du * w - y * dw_du) / (w * w),
                (dz_du * w - z * dw_du) / (w * w),
            )
        } else {
            Vector3d::new(dx_du, dy_du, dz_du)
        };

        let tan_v = if self.is_rational && w != 0.0 {
            Vector3d::new(
                (dx_dv * w - x * dw_dv) / (w * w),
                (dy_dv * w - y * dw_dv) / (w * w),
                (dz_dv * w - z * dw_dv) / (w * w),
            )
        } else {
            Vector3d::new(dx_dv, dy_dv, dz_dv)
        };

        (point, tan_u, tan_v)
    }

    /// Evaluates point and derivatives up to second order
    pub fn d2_local(&self, local_u: f64, local_v: f64) -> (Point, Vector3d, Vector3d, Vector3d, Vector3d, Vector3d) {
        let basis_u = self.evaluate_basis_functions(local_u, self.degree_u);
        let basis_v = self.evaluate_basis_functions(local_v, self.degree_v);
        let basis_du = self.evaluate_basis_derivatives(local_u, self.degree_u);
        let basis_dv = self.evaluate_basis_derivatives(local_v, self.degree_v);
        let basis_d2u = self.evaluate_basis_2nd_derivatives(local_u, self.degree_u);
        let basis_d2v = self.evaluate_basis_2nd_derivatives(local_v, self.degree_v);

        let stride = if self.is_rational { 4 } else { 3 };
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut dx_du = 0.0;
        let mut dy_du = 0.0;
        let mut dz_du = 0.0;
        let mut dx_dv = 0.0;
        let mut dy_dv = 0.0;
        let mut dz_dv = 0.0;
        let mut d2x_du2 = 0.0;
        let mut d2y_du2 = 0.0;
        let mut d2z_du2 = 0.0;
        let mut d2x_dv2 = 0.0;
        let mut d2y_dv2 = 0.0;
        let mut d2z_dv2 = 0.0;
        let mut d2x_dudv = 0.0;
        let mut d2y_dudv = 0.0;
        let mut d2z_dudv = 0.0;

        let mut w = 1.0;
        let mut dw_du = 0.0;
        let mut dw_dv = 0.0;
        let mut d2w_du2 = 0.0;
        let mut d2w_dv2 = 0.0;
        let mut d2w_dudv = 0.0;

        let mut idx = 0;
        for i in 0..=(self.degree_u as usize) {
            for j in 0..=(self.degree_v as usize) {
                let b = basis_u[i] * basis_v[j];
                let db_du = basis_du[i] * basis_v[j];
                let db_dv = basis_u[i] * basis_dv[j];
                let d2b_du2 = basis_d2u[i] * basis_v[j];
                let d2b_dv2 = basis_u[i] * basis_d2v[j];
                let d2b_dudv = basis_du[i] * basis_dv[j];

                x += b * self.poles_weights[idx];
                y += b * self.poles_weights[idx + 1];
                z += b * self.poles_weights[idx + 2];

                dx_du += db_du * self.poles_weights[idx];
                dy_du += db_du * self.poles_weights[idx + 1];
                dz_du += db_du * self.poles_weights[idx + 2];

                dx_dv += db_dv * self.poles_weights[idx];
                dy_dv += db_dv * self.poles_weights[idx + 1];
                dz_dv += db_dv * self.poles_weights[idx + 2];

                d2x_du2 += d2b_du2 * self.poles_weights[idx];
                d2y_du2 += d2b_du2 * self.poles_weights[idx + 1];
                d2z_du2 += d2b_du2 * self.poles_weights[idx + 2];

                d2x_dv2 += d2b_dv2 * self.poles_weights[idx];
                d2y_dv2 += d2b_dv2 * self.poles_weights[idx + 1];
                d2z_dv2 += d2b_dv2 * self.poles_weights[idx + 2];

                d2x_dudv += d2b_dudv * self.poles_weights[idx];
                d2y_dudv += d2b_dudv * self.poles_weights[idx + 1];
                d2z_dudv += d2b_dudv * self.poles_weights[idx + 2];

                if self.is_rational {
                    let wt = self.poles_weights[idx + 3];
                    w += b * (wt - 1.0);
                    dw_du += db_du * (wt - 1.0);
                    dw_dv += db_dv * (wt - 1.0);
                    d2w_du2 += d2b_du2 * (wt - 1.0);
                    d2w_dv2 += d2b_dv2 * (wt - 1.0);
                    d2w_dudv += d2b_dudv * (wt - 1.0);
                }

                idx += stride;
            }
        }

        let point = if self.is_rational && w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        };

        if self.is_rational && w != 0.0 {
            let w2 = w * w;
            let w3 = w2 * w;

            let tan_u = Vector3d::new(
                (dx_du * w - x * dw_du) / w2,
                (dy_du * w - y * dw_du) / w2,
                (dz_du * w - z * dw_du) / w2,
            );

            let tan_v = Vector3d::new(
                (dx_dv * w - x * dw_dv) / w2,
                (dy_dv * w - y * dw_dv) / w2,
                (dz_dv * w - z * dw_dv) / w2,
            );

            let curv_u = Vector3d::new(
                (d2x_du2 * w2 - 2.0 * dx_du * w * dw_du + 2.0 * x * dw_du * dw_du - x * w * d2w_du2) / w3,
                (d2y_du2 * w2 - 2.0 * dy_du * w * dw_du + 2.0 * y * dw_du * dw_du - y * w * d2w_du2) / w3,
                (d2z_du2 * w2 - 2.0 * dz_du * w * dw_du + 2.0 * z * dw_du * dw_du - z * w * d2w_du2) / w3,
            );

            let curv_v = Vector3d::new(
                (d2x_dv2 * w2 - 2.0 * dx_dv * w * dw_dv + 2.0 * x * dw_dv * dw_dv - x * w * d2w_dv2) / w3,
                (d2y_dv2 * w2 - 2.0 * dy_dv * w * dw_dv + 2.0 * y * dw_dv * dw_dv - y * w * d2w_dv2) / w3,
                (d2z_dv2 * w2 - 2.0 * dz_dv * w * dw_dv + 2.0 * z * dw_dv * dw_dv - z * w * d2w_dv2) / w3,
            );

            let curv_uv = Vector3d::new(
                (d2x_dudv * w2 - dx_du * w * dw_dv - dx_dv * w * dw_du + x * dw_du * dw_dv - x * w * d2w_dudv) / w3,
                (d2y_dudv * w2 - dy_du * w * dw_dv - dy_dv * w * dw_du + y * dw_du * dw_dv - y * w * d2w_dudv) / w3,
                (d2z_dudv * w2 - dz_du * w * dw_dv - dz_dv * w * dw_du + z * dw_du * dw_dv - z * w * d2w_dudv) / w3,
            );

            (point, tan_u, tan_v, curv_u, curv_v, curv_uv)
        } else {
            (
                point,
                Vector3d::new(dx_du, dy_du, dz_du),
                Vector3d::new(dx_dv, dy_dv, dz_dv),
                Vector3d::new(d2x_du2, d2y_du2, d2z_du2),
                Vector3d::new(d2x_dv2, d2y_dv2, d2z_dv2),
                Vector3d::new(d2x_dudv, d2y_dudv, d2z_dudv),
            )
        }
    }

    // Private helper methods
    fn find_knot_span(&self, u: f64, flat_knots: &[f64], degree: i32) -> i32 {
        let n = flat_knots.len() - 1;
        if u >= flat_knots[n - 1] {
            return (n - 1) as i32;
        }
        for i in (degree as usize)..n {
            if u < flat_knots[i + 1] {
                return i as i32;
            }
        }
        (degree) as i32
    }

    fn evaluate_basis_functions(&self, u: f64, degree: i32) -> std::vec::Vec<f64> {
        let mut result = std::vec![0.0; (degree + 1) as usize];
        match degree {
            2 => {
                result[0] = (1.0 - u) * (1.0 - u) / 4.0;
                result[1] = (1.0 - u * u) / 2.0;
                result[2] = (1.0 + u) * (1.0 + u) / 4.0;
            }
            3 => {
                let u1 = 1.0 - u;
                result[0] = u1 * u1 * u1 / 8.0;
                result[1] = (3.0 * u1 * u1 * (1.0 + u)) / 8.0;
                result[2] = (3.0 * u1 * (1.0 + u) * (1.0 + u)) / 8.0;
                result[3] = (1.0 + u) * (1.0 + u) * (1.0 + u) / 8.0;
            }
            _ => {
                for i in 0..=(degree as usize) {
                    result[i] = self.bernstein(u, degree, i as i32);
                }
            }
        }
        result
    }

    fn evaluate_basis_derivatives(&self, u: f64, degree: i32) -> std::vec::Vec<f64> {
        let mut result = std::vec![0.0; (degree + 1) as usize];
        match degree {
            2 => {
                result[0] = -(1.0 - u) / 2.0;
                result[1] = -u;
                result[2] = (1.0 + u) / 2.0;
            }
            3 => {
                let u1 = 1.0 - u;
                result[0] = -3.0 * u1 * u1 / 8.0;
                result[1] = (9.0 * u1 * u1 - 3.0 * (1.0 + u) * (1.0 + u)) / 8.0;
                result[2] = (3.0 * (1.0 + u) * (1.0 + u) - 9.0 * u1 * u1) / 8.0;
                result[3] = 3.0 * (1.0 + u) * (1.0 + u) / 8.0;
            }
            _ => {
                for i in 0..=(degree as usize) {
                    result[i] = self.bernstein_derivative(u, degree, i as i32);
                }
            }
        }
        result
    }

    fn evaluate_basis_2nd_derivatives(&self, u: f64, degree: i32) -> std::vec::Vec<f64> {
        let mut result = std::vec![0.0; (degree + 1) as usize];
        match degree {
            2 => {
                result[0] = -0.5;
                result[1] = -1.0;
                result[2] = 0.5;
            }
            3 => {
                let u1 = 1.0 - u;
                result[0] = 3.0 * u1 / 4.0;
                result[1] = (9.0 * u1 - 3.0 * (1.0 + u)) / 4.0;
                result[2] = (3.0 * (1.0 + u) - 9.0 * u1) / 4.0;
                result[3] = 3.0 * (1.0 + u) / 4.0;
            }
            _ => {
                for i in 0..=(degree as usize) {
                    result[i] = self.bernstein_2nd_derivative(u, degree, i as i32);
                }
            }
        }
        result
    }

    fn bernstein(&self, u: f64, n: i32, i: i32) -> f64 {
        let mut result = 1.0;
        for j in 0..i {
            result *= (u + 1.0) / 2.0;
        }
        for j in i..n {
            result *= (1.0 - u) / 2.0;
        }
        result
    }

    fn bernstein_derivative(&self, u: f64, n: i32, i: i32) -> f64 {
        if n == 0 {
            return 0.0;
        }
        n as f64 * (self.bernstein(u, n - 1, i - 1) - self.bernstein(u, n - 1, i)) / 2.0
    }

    fn bernstein_2nd_derivative(&self, u: f64, n: i32, i: i32) -> f64 {
        if n < 2 {
            return 0.0;
        }
        (n as f64 * (n - 1) as f64 / 4.0)
            * (self.bernstein(u, n - 2, i - 2) - 2.0 * self.bernstein(u, n - 2, i - 1)
                + self.bernstein(u, n - 2, i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, None);
        assert!(!cache.is_rational);
        assert_eq!(cache.degree_u, 2);
        assert_eq!(cache.degree_v, 2);
    }

    #[test]
    fn test_cache_validity() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, None);

        assert!(!cache.is_cache_valid(0.5, 0.5));

        let mut poles = std::vec![std::vec![Point::new(0.0, 0.0, 0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                poles[i][j] = Point::new(i as f64, j as f64, 0.0);
            }
        }

        cache.build_cache(0.5, 0.5, &flat_knots_u, &flat_knots_v, &poles, None);
        assert!(cache.is_cache_valid(0.5, 0.5));
    }

    #[test]
    fn test_d0_simple() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, None);

        let mut poles = std::vec![std::vec![Point::new(0.0, 0.0, 0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                poles[i][j] = Point::new(i as f64, j as f64, 0.0);
            }
        }

        cache.build_cache(0.5, 0.5, &flat_knots_u, &flat_knots_v, &poles, None);
        let point = cache.d0(0.5, 0.5);

        assert!(point.x.is_finite());
        assert!(point.y.is_finite());
        assert!(point.z.is_finite());
    }

    #[test]
    fn test_d1_simple() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, None);

        let mut poles = std::vec![std::vec![Point::new(0.0, 0.0, 0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                poles[i][j] = Point::new(i as f64, j as f64, 0.0);
            }
        }

        cache.build_cache(0.5, 0.5, &flat_knots_u, &flat_knots_v, &poles, None);
        let (point, _tan_u, _tan_v) = cache.d1(0.5, 0.5);

        assert!(point.x.is_finite());
    }

    #[test]
    fn test_d2_simple() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, None);

        let mut poles = std::vec![std::vec![Point::new(0.0, 0.0, 0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                poles[i][j] = Point::new(i as f64, j as f64, 0.0);
            }
        }

        cache.build_cache(0.5, 0.5, &flat_knots_u, &flat_knots_v, &poles, None);
        let (point, _tan_u, _tan_v, _curv_u, _curv_v, _curv_uv) = cache.d2(0.5, 0.5);

        assert!(point.x.is_finite());
    }

    #[test]
    fn test_rational_surface() {
        let flat_knots_u = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let flat_knots_v = std::vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let weights_data = std::vec![std::vec![1.0; 3]; 3];
        let cache = BsplSlibCache::new(2, false, &flat_knots_u, 2, false, &flat_knots_v, Some(&weights_data));

        assert!(cache.is_rational);
    }
}
