// FILE: geom_convert_func_cylinder_ls_dist.rs
// occt: GeomConvert_FuncCylinderLSDist

/// Function for searching cylinder canonical parameters from points using least squares.
pub struct GeomConvertFuncCylinderLSDist {
    points: Vec<(f64, f64, f64)>,
}

impl GeomConvertFuncCylinderLSDist {
    /// Create a new cylinder LS distance function.
    pub fn new(points: Vec<(f64, f64, f64)>) -> Self {
        GeomConvertFuncCylinderLSDist { points }
    }

    /// Number of variables: 6 (axis point, axis direction, radius).
    pub fn nb_variables(&self) -> usize {
        6
    }

    /// Compute function value.
    /// Minimizes sum of squared distances from points to cylinder surface.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 6 {
            return f64::INFINITY;
        }

        let ax = x[0];
        let ay = x[1];
        let az = x[2];
        let dx = x[3];
        let dy = x[4];
        let dz = x[5];

        // Normalize direction
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        if len < 1e-10 {
            return f64::INFINITY;
        }

        let dx = dx / len;
        let dy = dy / len;
        let dz = dz / len;

        // Radius is implicit; we compute it from data
        let mut sum_radius_sq = 0.0;
        let mut count = 0;

        for (px, py, pz) in &self.points {
            let vx = px - ax;
            let vy = py - ay;
            let vz = pz - az;

            // Project onto axis
            let proj = vx * dx + vy * dy + vz * dz;

            // Perpendicular component
            let perp_x = vx - proj * dx;
            let perp_y = vy - proj * dy;
            let perp_z = vz - proj * dz;

            let radius_sq = perp_x * perp_x + perp_y * perp_y + perp_z * perp_z;
            sum_radius_sq += radius_sq;
            count += 1;
        }

        if count == 0 {
            return f64::INFINITY;
        }

        let avg_radius = (sum_radius_sq / (count as f64)).sqrt();

        // Now compute error with average radius
        let mut sum = 0.0;
        for (px, py, pz) in &self.points {
            let vx = px - ax;
            let vy = py - ay;
            let vz = pz - az;

            let proj = vx * dx + vy * dy + vz * dz;

            let perp_x = vx - proj * dx;
            let perp_y = vy - proj * dy;
            let perp_z = vz - proj * dz;

            let radius = (perp_x * perp_x + perp_y * perp_y + perp_z * perp_z).sqrt();
            let error = radius - avg_radius;
            sum += error * error;
        }

        sum
    }

    /// Compute the gradient (numerical).
    pub fn gradient(&self, x: &[f64]) -> Vec<f64> {
        if x.len() < 6 {
            return vec![0.0; 6];
        }

        let h = 1e-8;
        let mut grad = vec![0.0; 6];

        let f0 = self.value(x);
        for i in 0..6 {
            let mut x_plus = x.to_vec();
            x_plus[i] += h;
            let f_plus = self.value(&x_plus);
            grad[i] = (f_plus - f0) / h;
        }

        grad
    }

    /// Set the points to fit.
    pub fn set_points(&mut self, points: Vec<(f64, f64, f64)>) {
        self.points = points;
    }

    /// Get the number of points.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_fit() {
        // Create points on a cylinder: axis along Z, radius 2
        let mut points = vec![];
        let radius = 2.0;
        for z in 0..5 {
            for i in 0..8 {
                let theta = (i as f64) * std::f64::consts::PI / 4.0;
                let x = radius * theta.cos();
                let y = radius * theta.sin();
                points.push((x, y, z as f64));
            }
        }

        let func = GeomConvertFuncCylinderLSDist::new(points);
        assert_eq!(func.nb_variables(), 6);

        // At ideal fit: axis at (0, 0, z), direction (0, 0, 1)
        let x = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let val = func.value(&x);
        // Should be very close to zero
        assert!(val < 1e-3);
    }

    #[test]
    fn test_nb_variables() {
        let points = vec![(0.0, 0.0, 0.0)];
        let func = GeomConvertFuncCylinderLSDist::new(points);
        assert_eq!(func.nb_variables(), 6);
    }

    #[test]
    fn test_gradient() {
        let points = vec![(2.0, 0.0, 0.0), (0.0, 2.0, 0.0), (-2.0, 0.0, 0.0)];
        let func = GeomConvertFuncCylinderLSDist::new(points);

        let x = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let grad = func.gradient(&x);
        assert_eq!(grad.len(), 6);
    }

    #[test]
    fn test_set_points() {
        let points1 = vec![(0.0, 0.0, 0.0)];
        let mut func = GeomConvertFuncCylinderLSDist::new(points1);
        assert_eq!(func.nb_points(), 1);

        let points2 = vec![(1.0, 1.0, 1.0), (2.0, 2.0, 2.0), (3.0, 3.0, 3.0)];
        func.set_points(points2);
        assert_eq!(func.nb_points(), 3);
    }

    #[test]
    fn test_invalid_axis() {
        let points = vec![(1.0, 1.0, 1.0)];
        let func = GeomConvertFuncCylinderLSDist::new(points);

        // Zero direction vector should cause infinite value
        let x = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(func.value(&x), f64::INFINITY);
    }
}
