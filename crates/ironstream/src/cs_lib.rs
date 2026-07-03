// FILE: cs_lib.rs
//! Provides functions for basic geometric computation on curves and surfaces.
//! Ported from OpenCascade CSLib class.

// occt: CSLib

use std::f64::consts::PI;

/// Enumeration for derivative status (first overload)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivativeStatus {
    /// Normal computed successfully
    Done,
    /// First derivative in U is null
    D1uIsNull,
    /// First derivative in V is null
    D1vIsNull,
    /// Both derivatives are null
    D1IsNull,
    /// D1u and D1v are parallel
    D1uIsParallelD1v,
}

/// Enumeration for normal status (second/third overload)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalStatus {
    /// Normal computed successfully
    Defined,
    /// Singular point (derivatives null or too small)
    Singular,
    /// Infinity of solutions
    InfinityOfSolutions,
    /// First normal derivative in U is null
    D1NuIsNull,
    /// First normal derivative in V is null
    D1NvIsNull,
    /// Both normal derivatives are null
    D1NIsNull,
    /// Normal derivatives are parallel
    D1NuIsParallelD1Nv,
    /// Ratio of V to U is null
    D1NvNuRatioIsNull,
    /// Ratio of U to V is null
    D1NuNvRatioIsNull,
}

/// A 3D vector (simplified gp_Vec equivalent)
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Returns the magnitude squared of the vector
    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    /// Computes the cross product with another vector
    pub fn crossed(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes the dot product with another vector
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Adds another vector to this vector
    pub fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// Scales the vector by a scalar
    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    /// Returns the normalized vector
    pub fn normalized(&self) -> Vec3 {
        let mag = self.magnitude();
        if mag < 1e-15 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            Vec3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    /// Checks if this vector is parallel to another vector
    pub fn is_parallel(&self, other: &Vec3, tolerance: f64) -> bool {
        let cross = self.crossed(other);
        let cross_mag_sq = cross.square_magnitude();
        let self_mag_sq = self.square_magnitude();
        let other_mag_sq = other.square_magnitude();

        if self_mag_sq < 1e-15 || other_mag_sq < 1e-15 {
            return false;
        }

        cross_mag_sq / (self_mag_sq * other_mag_sq) < tolerance * tolerance
    }

    /// Checks if this vector is opposite to another vector
    pub fn is_opposite(&self, other: &Vec3, tolerance: f64) -> bool {
        let self_mag = self.magnitude();
        let other_mag = other.magnitude();

        if self_mag < 1e-15 || other_mag < 1e-15 {
            return false;
        }

        let normalized_self = self.normalized();
        let normalized_other = other.normalized();

        let dot = normalized_self.dot(&normalized_other);
        (dot + 1.0).abs() < tolerance
    }
}

/// A direction vector (normalized 3D vector)
#[derive(Debug, Clone, Copy)]
pub struct Dir {
    x: f64,
    y: f64,
    z: f64,
}

impl Dir {
    /// Creates a new direction from a vector (auto-normalizes)
    pub fn from_vec(vec: &Vec3) -> Option<Self> {
        let mag = vec.magnitude();
        if mag < 1e-15 {
            None
        } else {
            Some(Dir {
                x: vec.x / mag,
                y: vec.y / mag,
                z: vec.z / mag,
            })
        }
    }

    /// Creates a new direction from explicit components
    pub fn new(x: f64, y: f64, z: f64) -> Option<Self> {
        let vec = Vec3::new(x, y, z);
        Self::from_vec(&vec)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    /// Computes cross product with another direction
    pub fn crossed(&self, other: &Dir) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Negates the direction
    pub fn neg(&self) -> Dir {
        Dir {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Computes the normal direction of a surface as cross product D1U ^ D1V (first overload).
///
/// The normal is undefined if D1U or D1V is null, or if they are parallel.
/// To check parallelism, the sine of the angle is compared with the tolerance.
pub fn normal_from_derivatives(
    d1u: &Vec3,
    d1v: &Vec3,
    sin_tol: f64,
) -> (DerivativeStatus, Option<Dir>) {
    let d1u_mag_sq = d1u.square_magnitude();
    let d1v_mag_sq = d1v.square_magnitude();
    let cross = d1u.crossed(d1v);

    const RESOLUTION: f64 = 1e-7;

    let status = if d1u_mag_sq <= RESOLUTION && d1v_mag_sq <= RESOLUTION {
        DerivativeStatus::D1IsNull
    } else if d1u_mag_sq <= RESOLUTION {
        DerivativeStatus::D1uIsNull
    } else if d1v_mag_sq <= RESOLUTION {
        DerivativeStatus::D1vIsNull
    } else {
        let sin2 = cross.square_magnitude() / (d1u_mag_sq * d1v_mag_sq);
        if sin2 < sin_tol * sin_tol {
            DerivativeStatus::D1uIsParallelD1v
        } else {
            DerivativeStatus::Done
        }
    };

    let normal = if status == DerivativeStatus::Done {
        Dir::from_vec(&cross)
    } else {
        None
    };

    (status, normal)
}

/// Computes the normal from second derivatives at singular points (second overload).
///
/// Uses Taylor expansion when first derivatives produce null or parallel cross products.
pub fn normal_with_second_derivatives(
    d1u: &Vec3,
    d1v: &Vec3,
    d2u: &Vec3,
    d2v: &Vec3,
    d2uv: &Vec3,
    sin_tol: f64,
) -> (bool, NormalStatus, Option<Dir>) {
    // Compute approximate normal derivatives using Taylor expansion:
    // dN/du = D2U ^ D1V + D1U ^ D2UV
    // dN/dv = D2UV ^ D1V + D1U ^ D2V

    let mut d1nu = d2u.crossed(d1v);
    let crossed = d1u.crossed(d2uv);
    d1nu.add(&crossed);

    let mut d1nv = d2uv.crossed(d1v);
    let crossed2 = d1u.crossed(d2v);
    d1nv.add(&crossed2);

    let ld1nu = d1nu.square_magnitude();
    let ld1nv = d1nv.square_magnitude();

    const REAL_EPSILON: f64 = 1e-15;

    let (done, status, normal) = if ld1nu <= REAL_EPSILON && ld1nv <= REAL_EPSILON {
        (false, NormalStatus::D1NIsNull, None)
    } else if ld1nu < REAL_EPSILON {
        let dir = Dir::from_vec(&d1nv);
        (true, NormalStatus::D1NuIsNull, dir)
    } else if ld1nv < REAL_EPSILON {
        let dir = Dir::from_vec(&d1nu);
        (true, NormalStatus::D1NvIsNull, dir)
    } else if (ld1nv / ld1nu) <= REAL_EPSILON {
        (false, NormalStatus::D1NvNuRatioIsNull, None)
    } else if (ld1nu / ld1nv) <= REAL_EPSILON {
        (false, NormalStatus::D1NuNvRatioIsNull, None)
    } else {
        let cross_d1n = d1nu.crossed(&d1nv);
        let sin2 = cross_d1n.square_magnitude() / (ld1nu * ld1nv);

        if sin2 < sin_tol * sin_tol {
            let dir = Dir::from_vec(&d1nu);
            (true, NormalStatus::D1NuIsParallelD1Nv, dir)
        } else {
            (false, NormalStatus::InfinityOfSolutions, None)
        }
    };

    (done, status, normal)
}

/// Computes the normal with magnitude tolerance (third overload).
///
/// A simpler version that checks if the cross product magnitude
/// and derivative magnitudes exceed the given tolerance.
pub fn normal_with_magnitude_tolerance(
    d1u: &Vec3,
    d1v: &Vec3,
    mag_tol: f64,
) -> (NormalStatus, Option<Dir>) {
    let d1u_mag = d1u.magnitude();
    let d1v_mag = d1v.magnitude();
    let cross = d1u.crossed(d1v);
    let n_mag = cross.magnitude();

    let status = if n_mag <= mag_tol || d1u_mag <= mag_tol || d1v_mag <= mag_tol {
        NormalStatus::Singular
    } else {
        NormalStatus::Defined
    };

    let normal = if status == NormalStatus::Defined {
        let d1u_dir = Dir::from_vec(d1u);
        let d1v_dir = Dir::from_vec(d1v);
        if let (Some(u_dir), Some(v_dir)) = (d1u_dir, d1v_dir) {
            Dir::from_vec(&u_dir.crossed(&v_dir))
        } else {
            None
        }
    } else {
        None
    };

    (status, normal)
}

/// Computes the derivative of the non-normalized normal vector N = D1U ^ D1V.
///
/// Implements the Leibniz rule for cross products:
/// d^(Nu+Nv)N / (du^Nu dv^Nv) using binomial coefficients.
///
/// Follows OCCT: sum over i,j of C(Nu,i)*C(Nv,j) * dS/du^(i+1) ^ dS/dv^(Nv+1-j)
pub fn dnnuv(
    nu: usize,
    nv: usize,
    der_surf: &[[Vec3; 5]; 5],
) -> Vec3 {
    let mut result = Vec3::new(0.0, 0.0, 0.0);

    for i in 0..=nu {
        for j in 0..=nv {
            let vg = &der_surf[i + 1][j];
            let vd = &der_surf[nu - i][nv + 1 - j];
            let cross = vg.crossed(vd);

            let bincoef = binomial(nu, i) * binomial(nv, j);
            let mut scaled = cross;
            scaled.scale(bincoef);
            result.add(&scaled);
        }
    }

    result
}

/// Computes binomial coefficient C(n, k) = n! / (k! * (n-k)!)
fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }

    let mut result = 1.0;
    for i in 0..k {
        result = result * (n - i) as f64 / (i + 1) as f64;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_dir_equal(d1: Option<Dir>, d2: Option<Dir>, tolerance: f64) -> bool {
        match (d1, d2) {
            (Some(a), Some(b)) => {
                (a.x - b.x).abs() < tolerance
                    && (a.y - b.y).abs() < tolerance
                    && (a.z - b.z).abs() < tolerance
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn check_vec_equal(v1: &Vec3, v2: &Vec3, tolerance: f64) -> bool {
        (v1.x - v2.x).abs() < tolerance
            && (v1.y - v2.y).abs() < tolerance
            && (v1.z - v2.z).abs() < tolerance
    }

    #[test]
    fn test_normal_from_orthogonal_vectors() {
        let d1u = Vec3::new(1.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 1.0, 0.0);

        let (status, normal) = normal_from_derivatives(&d1u, &d1v, 1e-10);

        assert_eq!(status, DerivativeStatus::Done);
        assert!(check_dir_equal(normal, Dir::new(0.0, 0.0, 1.0), 1e-10));
    }

    #[test]
    fn test_normal_from_scaled_vectors() {
        let d1u = Vec3::new(3.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 5.0, 0.0);

        let (status, normal) = normal_from_derivatives(&d1u, &d1v, 1e-10);

        assert_eq!(status, DerivativeStatus::Done);
        assert!(check_dir_equal(normal, Dir::new(0.0, 0.0, 1.0), 1e-10));
    }

    #[test]
    fn test_normal_from_parallel_vectors() {
        let d1u = Vec3::new(1.0, 0.0, 0.0);
        let d1v = Vec3::new(2.0, 0.0, 0.0);

        let (status, _normal) = normal_from_derivatives(&d1u, &d1v, 1e-6);

        assert_eq!(status, DerivativeStatus::D1uIsParallelD1v);
    }

    #[test]
    fn test_normal_d1u_null() {
        let d1u = Vec3::new(0.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 1.0, 0.0);

        let (status, _normal) = normal_from_derivatives(&d1u, &d1v, 1e-10);

        assert_eq!(status, DerivativeStatus::D1uIsNull);
    }

    #[test]
    fn test_normal_d1v_null() {
        let d1u = Vec3::new(1.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 0.0, 0.0);

        let (status, _normal) = normal_from_derivatives(&d1u, &d1v, 1e-10);

        assert_eq!(status, DerivativeStatus::D1vIsNull);
    }

    #[test]
    fn test_normal_both_null() {
        let d1u = Vec3::new(0.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 0.0, 0.0);

        let (status, _normal) = normal_from_derivatives(&d1u, &d1v, 1e-10);

        assert_eq!(status, DerivativeStatus::D1IsNull);
    }

    #[test]
    fn test_normal_with_mag_tol_defined() {
        let d1u = Vec3::new(1.0, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 1.0, 0.0);

        let (status, normal) = normal_with_magnitude_tolerance(&d1u, &d1v, 1e-10);

        assert_eq!(status, NormalStatus::Defined);
        assert!(check_dir_equal(normal, Dir::new(0.0, 0.0, 1.0), 1e-10));
    }

    #[test]
    fn test_normal_with_mag_tol_singular() {
        let d1u = Vec3::new(1e-12, 0.0, 0.0);
        let d1v = Vec3::new(0.0, 1e-12, 0.0);

        let (status, _normal) = normal_with_magnitude_tolerance(&d1u, &d1v, 1e-10);

        assert_eq!(status, NormalStatus::Singular);
    }

    #[test]
    fn test_dnnuv_zero_order() {
        let mut der_surf = [[Vec3::new(0.0, 0.0, 0.0); 5]; 5];
        der_surf[0][0] = Vec3::new(0.0, 0.0, 0.0); // Position (not used)
        der_surf[1][0] = Vec3::new(1.0, 0.0, 0.0); // D1U
        der_surf[0][1] = Vec3::new(0.0, 1.0, 0.0); // D1V

        // DNNUV(0,0) should give D1U ^ D1V (with binomial C(0,0)*C(0,0)=1)
        let result = dnnuv(0, 0, &der_surf);

        // D1U x D1V = (1,0,0) x (0,1,0) = (0,0,1)
        assert!(check_vec_equal(&result, &Vec3::new(0.0, 0.0, 1.0), 1e-10));
    }

    #[test]
    fn test_vec3_magnitude() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert!((v1.dot(&v2) - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let cross = v1.crossed(&v2);
        assert!(check_vec_equal(&cross, &Vec3::new(0.0, 0.0, 1.0), 1e-10));
    }

    #[test]
    fn test_binomial_coefficients() {
        assert_eq!(binomial(0, 0), 1.0);
        assert_eq!(binomial(5, 0), 1.0);
        assert_eq!(binomial(5, 5), 1.0);
        assert_eq!(binomial(5, 2), 10.0);
        assert_eq!(binomial(4, 2), 6.0);
    }
}
