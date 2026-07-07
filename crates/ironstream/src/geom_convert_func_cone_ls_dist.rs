// FILE: geom_convert_func_cone_ls_dist.rs
// occt: GeomConvert_FuncConeLSDist

//! Least-squares function for cone fitting from point sets.
//! Implements objective function for minimizing distance from points to cone surface.

#[derive(Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
pub struct Direction {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

/// Least-squares cone fitting function
pub struct GeomConvertFuncConeLSDist {
    points: Vec<Point3d>,
    direction: Direction,
}

impl GeomConvertFuncConeLSDist {
    /// Creates empty cone fitting function
    pub fn new() -> Self {
        GeomConvertFuncConeLSDist {
            points: Vec::new(),
            direction: Direction {
                dx: 0.0,
                dy: 0.0,
                dz: 1.0,
            },
        }
    }

    /// Creates cone fitting function with points and axis direction
    pub fn new_with_points_dir(_points: &[Point3d], _dir: &Direction) -> Self {
        GeomConvertFuncConeLSDist {
            points: _points.to_vec(),
            direction: _dir.clone(),
        }
    }

    /// Sets point set for fitting
    pub fn set_points(&mut self, points: &[Point3d]) {
        self.points = points.to_vec();
    }

    /// Sets axis direction
    pub fn set_dir(&mut self, dir: &Direction) {
        self.direction = dir.clone();
    }

    /// Returns number of variables: center (3) + radius + semi-angle = 5
    pub fn nb_variables(&self) -> i32 {
        5
    }

    /// Evaluates least-squares function for cone parameters
    /// x[0..2] = cone center coordinates
    /// x[3] = cone radius
    /// x[4] = cone semi-angle
    /// Returns F = sum of squared distances from points to cone surface
    pub fn value(&self, x: &[f64]) -> Result<f64, String> {
        if x.len() < 5 {
            return Err("Insufficient variables".to_string());
        }

        // TODO: Implement cone distance computation
        // F = sum[(distance(point_i, cone)]^2
        let mut sum = 0.0;
        for _point in &self.points {
            // Compute distance from point to cone surface
            // For a cone with apex at origin, axis along z, semi-angle alpha:
            // cone surface: x^2 + y^2 = (z * tan(alpha))^2
            let _dist = 0.0; // TODO: actual distance computation
            sum += _dist * _dist;
        }
        Ok(sum)
    }
}

impl Default for GeomConvertFuncConeLSDist {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GeomConvertFuncConeLSDist {
    fn clone(&self) -> Self {
        GeomConvertFuncConeLSDist {
            points: self.points.clone(),
            direction: self.direction.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cone_ls_dist_new() {
        let func = GeomConvertFuncConeLSDist::new();
        assert_eq!(func.nb_variables(), 5);
    }

    #[test]
    fn test_cone_ls_dist_nb_variables() {
        let func = GeomConvertFuncConeLSDist::new();
        assert_eq!(func.nb_variables(), 5);
    }

    #[test]
    fn test_cone_ls_dist_value() {
        let func = GeomConvertFuncConeLSDist::new();
        let x = vec![0.0, 0.0, 0.0, 1.0, 0.5];
        let result = func.value(&x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cone_ls_dist_set_points() {
        let mut func = GeomConvertFuncConeLSDist::new();
        let points = vec![
            Point3d {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Point3d {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        func.set_points(&points);
        assert_eq!(func.points.len(), 2);
    }

    #[test]
    fn test_cone_ls_dist_set_dir() {
        let mut func = GeomConvertFuncConeLSDist::new();
        let dir = Direction {
            dx: 0.0,
            dy: 0.0,
            dz: 1.0,
        };
        func.set_dir(&dir);
        assert_eq!(func.direction.dz, 1.0);
    }
}
