// FILE: g_prop.rs
// occt: GProp

/// Global properties computation utilities for geometric systems.
/// Computes properties like mass, center of mass, moments of inertia for curves, surfaces, and solids.
pub struct GProp;

impl GProp {
    /// Computes the Huyghens Operator for use with the inertia tensor theorem.
    /// The operator relates inertia at a point Q to inertia at center of mass G.
    ///
    /// # Arguments
    /// * `g` - Center of mass point [x, y, z]
    /// * `q` - Reference point [x, y, z]
    /// * `mass` - Total mass
    ///
    /// # Returns
    /// A 3x3 Huyghens operator matrix (stored as 9 values row-major)
    pub fn h_operator(g: [f64; 3], q: [f64; 3], mass: f64) -> [f64; 9] {
        let dx = q[0] - g[0];
        let dy = q[1] - g[1];
        let dz = q[2] - g[2];

        let r2 = dx * dx + dy * dy + dz * dz;

        // Huyghens operator matrix:
        // [r2 - dx*dx,    -dx*dy,    -dx*dz]
        // [   -dx*dy, r2 - dy*dy,    -dy*dz]
        // [   -dx*dz,    -dy*dz, r2 - dz*dz]
        // multiplied by mass
        [
            mass * (r2 - dx * dx),
            mass * (-dx * dy),
            mass * (-dx * dz),
            mass * (-dx * dy),
            mass * (r2 - dy * dy),
            mass * (-dy * dz),
            mass * (-dx * dz),
            mass * (-dy * dz),
            mass * (r2 - dz * dz),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_operator_zero_distance() {
        // When G = Q, operator should be zero matrix
        let g = [1.0, 2.0, 3.0];
        let q = [1.0, 2.0, 3.0];
        let mass = 5.0;
        let result = GProp::h_operator(g, q, mass);

        for i in 0..9 {
            assert!((result[i]).abs() < 1e-10, "Element {} should be ~0, got {}", i, result[i]);
        }
    }

    #[test]
    fn test_h_operator_mass_zero() {
        let g = [0.0, 0.0, 0.0];
        let q = [1.0, 1.0, 1.0];
        let mass = 0.0;
        let result = GProp::h_operator(g, q, mass);

        for i in 0..9 {
            assert!((result[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_h_operator_diagonal() {
        let g = [0.0, 0.0, 0.0];
        let q = [1.0, 0.0, 0.0];
        let mass = 1.0;
        let result = GProp::h_operator(g, q, mass);

        // r2 = 1, dx*dx = 1, dy*dy = 0, dz*dz = 0
        // result[0] = mass * (r2 - dx*dx) = 1 * (1 - 1) = 0
        assert!((result[0]).abs() < 1e-10);
        // result[4] = mass * (r2 - dy*dy) = 1 * (1 - 0) = 1
        assert!((result[4] - 1.0).abs() < 1e-10);
        // result[8] = mass * (r2 - dz*dz) = 1 * (1 - 0) = 1
        assert!((result[8] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_h_operator_mass_scaling() {
        let g = [0.0, 0.0, 0.0];
        let q = [1.0, 1.0, 0.0];
        let result1 = GProp::h_operator(g, q, 1.0);
        let result2 = GProp::h_operator(g, q, 2.0);

        for i in 0..9 {
            assert!((result2[i] - 2.0 * result1[i]).abs() < 1e-10);
        }
    }
}
