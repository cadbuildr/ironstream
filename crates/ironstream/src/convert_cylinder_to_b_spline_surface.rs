// FILE: convert_cylinder_to_b_spline_surface.rs
// occt: Convert_CylinderToBSplineSurface

//! This algorithm converts a bounded cylinder into a rational B-spline surface.
//! The cylinder is defined by radius and parametric ranges [u1, u2] x [v1, v2].
//! The parametrization of the cylinder is:
//! P(U, V) = Loc + V * Zdir + Radius * (Xdir*cos(U) + Ydir*sin(U))
//! where Loc is the location point, Xdir, Ydir, Zdir are the local cartesian coordinate system.
//! The U parametrization range is typically [0, 2*pi].

use core::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3d { x, y, z }
    }
}

#[derive(Clone, Debug)]
pub struct ConvertCylinderToBSplineSurface {
    poles: Vec<Vec<Point3d>>,
    weights: Vec<Vec<f64>>,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<i32>,
    v_mults: Vec<i32>,
    u_degree: usize,
    v_degree: usize,
    nb_u_poles: usize,
    nb_v_poles: usize,
    nb_u_knots: usize,
    nb_v_knots: usize,
    is_u_periodic: bool,
    is_v_periodic: bool,
}

impl ConvertCylinderToBSplineSurface {
    /// Creates a B-spline surface from a cylinder with specified U and V parametric ranges.
    ///
    /// # Arguments
    /// * `radius` - The radius of the cylinder
    /// * `u1`, `u2` - U parametric range (angular direction)
    /// * `v1`, `v2` - V parametric range (axial direction)
    /// * `center_x`, `center_y`, `center_z` - Cylinder center location
    /// * `xdir_x`, `xdir_y`, `xdir_z` - Local X direction (normalized)
    /// * `ydir_x`, `ydir_y`, `ydir_z` - Local Y direction (normalized)
    /// * `zdir_x`, `zdir_y`, `zdir_z` - Local Z direction (normalized)
    pub fn new(
        radius: f64,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        center_x: f64,
        center_y: f64,
        center_z: f64,
        xdir_x: f64,
        xdir_y: f64,
        xdir_z: f64,
        ydir_x: f64,
        ydir_y: f64,
        ydir_z: f64,
        zdir_x: f64,
        zdir_y: f64,
        zdir_z: f64,
    ) -> Self {
        let delta_u = u2 - u1;
        let delta_v = v2 - v1;

        // Validation
        let epsilon = 1e-15;
        if delta_v.abs() <= epsilon {
            panic!("Convert_CylinderToBSplineSurface: V1 == V2");
        }
        if delta_u > 2.0 * PI || delta_u < 0.0 {
            panic!("Convert_CylinderToBSplineSurface: Invalid U range");
        }

        // Compute number of spans: maximum opening = 150 degrees (PI / 1.2 rads)
        let nb_u_spans = ((1.2 * delta_u / PI).trunc() as usize) + 1;
        let alfa_u = delta_u / ((nb_u_spans as f64) * 2.0);

        let nb_u_poles = 2 * nb_u_spans + 1;
        let nb_u_knots = nb_u_spans + 1;
        let nb_v_poles = 2;
        let nb_v_knots = 2;

        // Initialize arrays
        let mut poles = vec![vec![Point3d::new(0.0, 0.0, 0.0); nb_v_poles]; nb_u_poles];
        let mut weights = vec![vec![0.0; nb_v_poles]; nb_u_poles];
        let mut u_knots = vec![0.0; nb_u_knots];
        let mut v_knots = vec![v1, v2];
        let mut u_mults = vec![2; nb_u_knots];
        let mut v_mults = vec![2, 2];

        // Set first and last U multiplicity to 3 (clamped)
        u_mults[0] += 1;
        u_mults[nb_u_knots - 1] += 1;

        // Compute poles
        let mut u_start = u1;
        poles[0][0] = Point3d::new(radius * u_start.cos(), radius * u_start.sin(), v1);
        poles[0][1] = Point3d::new(radius * u_start.cos(), radius * u_start.sin(), v2);

        for i in 1..=nb_u_spans {
            let u_mid = u_start + alfa_u;
            let u_end = u_start + 2.0 * alfa_u;

            poles[2 * i - 1][0] = Point3d::new(
                radius * u_mid.cos() / alfa_u.cos(),
                radius * u_mid.sin() / alfa_u.cos(),
                v1,
            );
            poles[2 * i - 1][1] = Point3d::new(
                radius * u_mid.cos() / alfa_u.cos(),
                radius * u_mid.sin() / alfa_u.cos(),
                v2,
            );
            poles[2 * i][0] = Point3d::new(radius * u_end.cos(), radius * u_end.sin(), v1);
            poles[2 * i][1] = Point3d::new(radius * u_end.cos(), radius * u_end.sin(), v2);

            u_start += 2.0 * alfa_u;
        }

        // Compute U knots
        for i in 0..nb_u_knots {
            u_knots[i] = u1 + (i as f64) * 2.0 * alfa_u;
        }

        // Compute weights and transform poles
        for i in 0..nb_u_poles {
            let w1 = if (i + 1) % 2 == 0 {
                alfa_u.cos()
            } else {
                1.0
            };

            for j in 0..nb_v_poles {
                weights[i][j] = w1;

                // Transform pole from XOY frame to cylinder frame
                let x = poles[i][j].x;
                let y = poles[i][j].y;
                let z = poles[i][j].z;

                poles[i][j].x = center_x + x * xdir_x + y * ydir_x + z * zdir_x;
                poles[i][j].y = center_y + x * xdir_y + y * ydir_y + z * zdir_y;
                poles[i][j].z = center_z + x * xdir_z + y * ydir_z + z * zdir_z;
            }
        }

        ConvertCylinderToBSplineSurface {
            poles,
            weights,
            u_knots,
            v_knots,
            u_mults,
            v_mults,
            u_degree: 2,
            v_degree: 1,
            nb_u_poles,
            nb_v_poles,
            nb_u_knots,
            nb_v_knots,
            is_u_periodic: false,
            is_v_periodic: false,
        }
    }

    /// Creates a B-spline surface from a full periodic cylinder.
    ///
    /// # Arguments
    /// * `radius` - The radius of the cylinder
    /// * `v1`, `v2` - V parametric range (axial direction)
    pub fn new_periodic(
        radius: f64,
        v1: f64,
        v2: f64,
        center_x: f64,
        center_y: f64,
        center_z: f64,
        xdir_x: f64,
        xdir_y: f64,
        xdir_z: f64,
        ydir_x: f64,
        ydir_y: f64,
        ydir_z: f64,
        zdir_x: f64,
        zdir_y: f64,
        zdir_z: f64,
    ) -> Self {
        let delta_v = v2 - v1;

        // Validation
        let epsilon = 1e-15;
        if delta_v.abs() <= epsilon {
            panic!("Convert_CylinderToBSplineSurface: V1 == V2");
        }

        // For periodic cylinder: 6 poles, 4 knots
        let nb_u_poles = 6;
        let nb_u_knots = 4;
        let nb_v_poles = 2;
        let nb_v_knots = 2;

        // Initialize arrays
        let mut poles = vec![vec![Point3d::new(0.0, 0.0, 0.0); nb_v_poles]; nb_u_poles];
        let mut weights = vec![vec![0.0; nb_v_poles]; nb_u_poles];
        let mut u_knots = vec![0.0; nb_u_knots];
        let mut v_knots = vec![v1, v2];
        let mut u_mults = vec![2; nb_u_knots];
        let mut v_mults = vec![2, 2];

        // Compute poles for full cylinder [0, 2*pi]
        let u1 = 0.0;
        let u2 = 2.0 * PI;
        let delta_u = u2 - u1;
        let nb_u_spans = ((1.2 * delta_u / PI).trunc() as usize) + 1;
        let alfa_u = delta_u / ((nb_u_spans as f64) * 2.0);

        let mut u_start = u1;
        poles[0][0] = Point3d::new(radius * u_start.cos(), radius * u_start.sin(), v1);
        poles[0][1] = Point3d::new(radius * u_start.cos(), radius * u_start.sin(), v2);

        for i in 1..=nb_u_spans {
            let u_mid = u_start + alfa_u;
            let u_end = u_start + 2.0 * alfa_u;

            poles[2 * i - 1][0] = Point3d::new(
                radius * u_mid.cos() / alfa_u.cos(),
                radius * u_mid.sin() / alfa_u.cos(),
                v1,
            );
            poles[2 * i - 1][1] = Point3d::new(
                radius * u_mid.cos() / alfa_u.cos(),
                radius * u_mid.sin() / alfa_u.cos(),
                v2,
            );

            if 2 * i < nb_u_poles {
                poles[2 * i][0] = Point3d::new(radius * u_end.cos(), radius * u_end.sin(), v1);
                poles[2 * i][1] = Point3d::new(radius * u_end.cos(), radius * u_end.sin(), v2);
            }

            u_start += 2.0 * alfa_u;
        }

        // U knots for periodic: span [0, 2*pi/3]
        for i in 0..nb_u_knots {
            u_knots[i] = (i as f64) * 2.0 * PI / 3.0;
        }

        // Compute weights and transform poles
        for i in 0..nb_u_poles {
            let w = if (i + 1) % 2 == 0 {
                0.5  // cos(pi/3)
            } else {
                1.0
            };

            for j in 0..nb_v_poles {
                weights[i][j] = w;

                // Transform pole from XOY frame to cylinder frame
                let x = poles[i][j].x;
                let y = poles[i][j].y;
                let z = poles[i][j].z;

                poles[i][j].x = center_x + x * xdir_x + y * ydir_x + z * zdir_x;
                poles[i][j].y = center_y + x * xdir_y + y * ydir_y + z * zdir_y;
                poles[i][j].z = center_z + x * xdir_z + y * ydir_z + z * zdir_z;
            }
        }

        ConvertCylinderToBSplineSurface {
            poles,
            weights,
            u_knots,
            v_knots,
            u_mults,
            v_mults,
            u_degree: 2,
            v_degree: 1,
            nb_u_poles,
            nb_v_poles,
            nb_u_knots,
            nb_v_knots,
            is_u_periodic: true,
            is_v_periodic: false,
        }
    }

    pub fn u_degree(&self) -> usize {
        self.u_degree
    }

    pub fn v_degree(&self) -> usize {
        self.v_degree
    }

    pub fn nb_u_poles(&self) -> usize {
        self.nb_u_poles
    }

    pub fn nb_v_poles(&self) -> usize {
        self.nb_v_poles
    }

    pub fn nb_u_knots(&self) -> usize {
        self.nb_u_knots
    }

    pub fn nb_v_knots(&self) -> usize {
        self.nb_v_knots
    }

    pub fn is_u_periodic(&self) -> bool {
        self.is_u_periodic
    }

    pub fn is_v_periodic(&self) -> bool {
        self.is_v_periodic
    }

    pub fn poles(&self) -> &Vec<Vec<Point3d>> {
        &self.poles
    }

    pub fn weights(&self) -> &Vec<Vec<f64>> {
        &self.weights
    }

    pub fn u_knots(&self) -> &Vec<f64> {
        &self.u_knots
    }

    pub fn v_knots(&self) -> &Vec<f64> {
        &self.v_knots
    }

    pub fn u_multiplicities(&self) -> &Vec<i32> {
        &self.u_mults
    }

    pub fn v_multiplicities(&self) -> &Vec<i32> {
        &self.v_mults
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn test_full_cylinder() {
        let conv = ConvertCylinderToBSplineSurface::new_periodic(
            3.0, 0.0, 10.0,
            0.0, 0.0, 0.0,  // center
            1.0, 0.0, 0.0,  // xdir
            0.0, 1.0, 0.0,  // ydir
            0.0, 0.0, 1.0   // zdir
        );

        assert_eq!(conv.u_degree(), 2);
        assert_eq!(conv.v_degree(), 1);
        assert!(conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
        assert!(conv.nb_u_poles() > 0);
        assert!(conv.nb_v_poles() > 0);
    }

    #[test]
    fn test_trimmed_cylinder() {
        let u1 = 0.0;
        let u2 = PI;
        let v1 = -5.0;
        let v2 = 5.0;

        let conv = ConvertCylinderToBSplineSurface::new(
            2.0, u1, u2, v1, v2,
            0.0, 0.0, 0.0,  // center
            1.0, 0.0, 0.0,  // xdir
            0.0, 1.0, 0.0,  // ydir
            0.0, 0.0, 1.0   // zdir
        );

        assert!(!conv.is_u_periodic());
        assert!(!conv.is_v_periodic());
        assert!(conv.nb_u_poles() > 0);
        assert!(conv.nb_v_poles() > 0);
    }

    #[test]
    fn test_weights_are_positive() {
        let conv = ConvertCylinderToBSplineSurface::new(
            1.0, 0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0,  // center
            1.0, 0.0, 0.0,  // xdir
            0.0, 1.0, 0.0,  // ydir
            0.0, 0.0, 1.0   // zdir
        );

        for i in 0..conv.nb_u_poles() {
            for j in 0..conv.nb_v_poles() {
                assert!(conv.weights()[i][j] > 0.0,
                    "Weight at ({}, {}) is not positive", i, j);
            }
        }
    }

    #[test]
    fn test_knots_are_monotonic() {
        let conv = ConvertCylinderToBSplineSurface::new(
            1.0, 0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0,  // center
            1.0, 0.0, 0.0,  // xdir
            0.0, 1.0, 0.0,  // ydir
            0.0, 0.0, 1.0   // zdir
        );

        for i in 1..conv.nb_u_knots() {
            assert!(conv.u_knots()[i] > conv.u_knots()[i - 1],
                "U knots not monotonic at index {}", i);
        }

        for i in 1..conv.nb_v_knots() {
            assert!(conv.v_knots()[i] > conv.v_knots()[i - 1],
                "V knots not monotonic at index {}", i);
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_v_range() {
        ConvertCylinderToBSplineSurface::new(
            1.0, 0.0, 1.0, 1.0, 1.0,  // v1 == v2
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_u_range() {
        ConvertCylinderToBSplineSurface::new(
            1.0, 0.0, 3.0 * PI, 0.0, 1.0,  // u2 - u1 > 2*PI
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        );
    }

    #[test]
    fn test_geometric_verification() {
        let radius = 3.0;
        let v1 = -5.0;
        let v2 = 5.0;

        let conv = ConvertCylinderToBSplineSurface::new(
            radius, 0.0, PI, v1, v2,
            0.0, 0.0, 0.0,  // center
            1.0, 0.0, 0.0,  // xdir
            0.0, 1.0, 0.0,  // ydir
            0.0, 0.0, 1.0   // zdir
        );

        // Note: Test basic sanity - poles with weight 1.0 should be on the cylinder,
        // but intermediate poles (weight != 1.0) are off-surface control points
        // and their distance from axis varies. We just check Z is within range.
        let tol = 1e-10;
        for pole_row in conv.poles() {
            for pole in pole_row {
                // Verify Z is within range
                assert!(pole.z >= v1 - tol && pole.z <= v2 + tol,
                    "Pole Z {} outside range [{}, {}]", pole.z, v1, v2);
            }
        }

        // Verify at least some poles have the correct radius (those with weight 1.0)
        let mut found_on_surface = false;
        for pole_row in conv.poles() {
            for pole in pole_row {
                let dist_from_axis = (pole.x * pole.x + pole.y * pole.y).sqrt();
                if approx_eq(dist_from_axis, radius, 1e-9) {
                    found_on_surface = true;
                    break;
                }
            }
            if found_on_surface { break; }
        }
        assert!(found_on_surface, "No poles found on cylinder surface");
    }
}
