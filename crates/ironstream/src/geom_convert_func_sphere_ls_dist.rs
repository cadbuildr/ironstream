// FILE: geom_convert_func_sphere_ls_dist.rs
// occt: GeomConvert_FuncSphereLSDist

//! Least-squares function for sphere fitting with gradient support.
//! F(x0, y0, z0, R) = Sum[(x(i) - x0)^2 + (y(i) - y0)^2 + (z(i) - z0)^2 - R^2]^2

#[derive(Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Least-squares sphere fitting function with gradient
pub struct GeomConvertFuncSphereLSDist {
    points: Vec<Point3d>,
}

impl GeomConvertFuncSphereLSDist {
    /// Creates empty sphere fitting function
    pub fn new() -> Self {
        GeomConvertFuncSphereLSDist {
            points: Vec::new(),
        }
    }

    /// Creates sphere fitting function with point set
    pub fn new_with_points(_points: &[Point3d]) -> Self {
        GeomConvertFuncSphereLSDist {
            points: _points.to_vec(),
        }
    }

    /// Sets points for fitting
    pub fn set_points(&mut self, points: &[Point3d]) {
        self.points = points.to_vec();
    }

    /// Returns number of variables: center (3) + radius = 4
    pub fn nb_variables(&self) -> i32 {
        4
    }

    /// Evaluates F(x0, y0, z0, R) = Sum[(x(i) - x0)^2 + (y(i) - y0)^2 + (z(i) - z0)^2 - R^2]^2
    /// x[0] = x0 (center x coordinate)
    /// x[1] = y0 (center y coordinate)
    /// x[2] = z0 (center z coordinate)
    /// x[3] = R (radius)
    pub fn value(&self, x: &[f64]) -> Result<f64, String> {
        if x.len() < 4 {
            return Err("Insufficient variables".to_string());
        }

        let x0 = x[0];
        let y0 = x[1];
        let z0 = x[2];
        let r = x[3];

        let mut sum = 0.0;
        for point in &self.points {
            let dx = point.x - x0;
            let dy = point.y - y0;
            let dz = point.z - z0;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let term = dist_sq - r * r;
            sum += term * term;
        }
        Ok(sum)
    }

    /// Computes gradient vector
    /// G1 = -4*Sum{[...]*(x(i) - x0)}
    /// G2 = -4*Sum{[...]*(y(i) - y0)}
    /// G3 = -4*Sum{[...]*(z(i) - z0)}
    /// G4 = -4*R*Sum{[...]}
    pub fn gradient(&self, x: &[f64]) -> Result<Vec<f64>, String> {
        if x.len() < 4 {
            return Err("Insufficient variables".to_string());
        }

        let x0 = x[0];
        let y0 = x[1];
        let z0 = x[2];
        let r = x[3];

        let mut g1 = 0.0;
        let mut g2 = 0.0;
        let mut g3 = 0.0;
        let mut g4 = 0.0;

        for point in &self.points {
            let dx = point.x - x0;
            let dy = point.y - y0;
            let dz = point.z - z0;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let term = dist_sq - r * r;

            g1 += term * dx;
            g2 += term * dy;
            g3 += term * dz;
            g4 += term * r;
        }

        Ok(vec![-4.0 * g1, -4.0 * g2, -4.0 * g3, -4.0 * g4])
    }

    /// Evaluates both function and gradient
    pub fn value_and_gradient(
        &self,
        x: &[f64],
    ) -> Result<(f64, Vec<f64>), String> {
        let f = self.value(x)?;
        let g = self.gradient(x)?;
        Ok((f, g))
    }
}

impl Default for GeomConvertFuncSphereLSDist {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GeomConvertFuncSphereLSDist {
    fn clone(&self) -> Self {
        GeomConvertFuncSphereLSDist {
            points: self.points.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_ls_dist_new() {
        let func = GeomConvertFuncSphereLSDist::new();
        assert_eq!(func.nb_variables(), 4);
    }

    #[test]
    fn test_sphere_ls_dist_nb_variables() {
        let func = GeomConvertFuncSphereLSDist::new();
        assert_eq!(func.nb_variables(), 4);
    }

    #[test]
    fn test_sphere_ls_dist_value() {
        let func = GeomConvertFuncSphereLSDist::new();
        let x = vec![0.0, 0.0, 0.0, 1.0];
        let result = func.value(&x);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_sphere_ls_dist_gradient() {
        let func = GeomConvertFuncSphereLSDist::new();
        let x = vec![0.0, 0.0, 0.0, 1.0];
        let result = func.gradient(&x);
        assert!(result.is_ok());
        let g = result.unwrap();
        assert_eq!(g.len(), 4);
    }

    #[test]
    fn test_sphere_ls_dist_with_points() {
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
        let func = GeomConvertFuncSphereLSDist::new_with_points(&points);
        let x = vec![0.0, 0.0, 0.0, 1.0];
        let result = func.value(&x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sphere_ls_dist_value_and_gradient() {
        let func = GeomConvertFuncSphereLSDist::new();
        let x = vec![0.0, 0.0, 0.0, 1.0];
        let result = func.value_and_gradient(&x);
        assert!(result.is_ok());
        let (f, g) = result.unwrap();
        assert_eq!(f, 0.0);
        assert_eq!(g.len(), 4);
    }
}
