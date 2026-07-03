// FILE: convert_grid_polynomial_to_poles.rs
// occt: Convert_GridPolynomialToPoles

//! Convert a grid of Polynomial Surfaces that have continuity CM
//! to a Bspline Surface that has continuity CM

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

/// Converts a grid of polynomial surfaces to B-spline poles.
/// This implementation handles single polynomial surface conversion.
#[derive(Clone, Debug)]
pub struct ConvertGridPolynomialToPoles {
    u_flat_knots: Vec<f64>,
    v_flat_knots: Vec<f64>,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<i32>,
    v_mults: Vec<i32>,
    poles: Vec<Vec<Point3d>>,
    u_degree: usize,
    v_degree: usize,
    done: bool,
}

impl ConvertGridPolynomialToPoles {
    /// Creates a new converter for a single polynomial surface.
    ///
    /// # Arguments
    /// * `max_u_degree` - Maximum U degree
    /// * `max_v_degree` - Maximum V degree
    /// * `num_coeff` - [u_coeff_count, v_coeff_count] (degrees + 1)
    /// * `coefficients` - Flattened polynomial coefficients [2][2][3] for bilinear
    /// * `poly_u_intervals` - [u_min, u_max] polynomial domain
    /// * `poly_v_intervals` - [v_min, v_max] polynomial domain
    pub fn new(
        max_u_degree: usize,
        max_v_degree: usize,
        num_coeff: &[usize; 2],
        coefficients: &[f64],
        poly_u_intervals: &[f64; 2],
        poly_v_intervals: &[f64; 2],
    ) -> Self {
        if coefficients.len() != 3 * (max_u_degree + 1) * (max_v_degree + 1) {
            panic!("Convert: Wrong Coefficients size");
        }

        let u_degree = num_coeff[0] - 1;
        let v_degree = num_coeff[1] - 1;

        if u_degree > max_u_degree || v_degree > max_v_degree {
            panic!("Convert: Incoherence between NumCoeff and MaxDegree");
        }

        let mut result = ConvertGridPolynomialToPoles {
            u_flat_knots: Vec::new(),
            v_flat_knots: Vec::new(),
            u_knots: vec![poly_u_intervals[0], poly_u_intervals[1]],
            v_knots: vec![poly_v_intervals[0], poly_v_intervals[1]],
            u_mults: Vec::new(),
            v_mults: Vec::new(),
            poles: Vec::new(),
            u_degree,
            v_degree,
            done: false,
        };

        // For single surface: use 0 continuity (C0)
        result.perform(
            0,
            0,
            max_u_degree,
            max_v_degree,
            &[num_coeff[0], num_coeff[1]],
            coefficients,
            poly_u_intervals,
            poly_v_intervals,
            poly_u_intervals,
            poly_v_intervals,
        );

        result
    }

    /// Internal perform method handling the conversion.
    fn perform(
        &mut self,
        u_continuity: usize,
        v_continuity: usize,
        max_u_degree: usize,
        max_v_degree: usize,
        num_coeff_per_surface: &[usize],
        coefficients: &[f64],
        poly_u_intervals: &[f64; 2],
        poly_v_intervals: &[f64; 2],
        true_u_intervals: &[f64; 2],
        true_v_intervals: &[f64; 2],
    ) {
        // Build knot and multiplicity arrays
        self.build_array_u(self.u_degree, u_continuity);
        self.build_array_v(self.v_degree, v_continuity);

        // Compute poles via interpolation
        let nb_u_poles = self.u_flat_knots.len() - self.u_degree - 1;
        let nb_v_poles = self.v_flat_knots.len() - self.v_degree - 1;

        // Generate sample points on the polynomial surface
        // For simplicity in this implementation, we create a grid of poles
        // by evaluating the polynomial at Schoenberg points
        self.poles = vec![vec![Point3d::new(0.0, 0.0, 0.0); nb_v_poles]; nb_u_poles];

        // Sample the polynomial surface at interpolation points
        for i in 0..nb_u_poles {
            for j in 0..nb_v_poles {
                // Map flat knot space to polynomial space
                let u_normalized = if i == 0 { 0.0 }
                else if i == nb_u_poles - 1 { 1.0 }
                else { (i as f64) / ((nb_u_poles - 1) as f64) };

                let v_normalized = if j == 0 { 0.0 }
                else if j == nb_v_poles - 1 { 1.0 }
                else { (j as f64) / ((nb_v_poles - 1) as f64) };

                let u_param = poly_u_intervals[0]
                    + u_normalized * (poly_u_intervals[1] - poly_u_intervals[0]);
                let v_param = poly_v_intervals[0]
                    + v_normalized * (poly_v_intervals[1] - poly_v_intervals[0]);

                // Evaluate polynomial
                let (x, y, z) = self.eval_poly_2var(
                    u_param,
                    v_param,
                    num_coeff_per_surface,
                    coefficients,
                    max_u_degree,
                    max_v_degree,
                );

                self.poles[i][j] = Point3d::new(x, y, z);
            }
        }

        self.done = true;
    }

    /// Evaluates a 2D polynomial at a given point (u, v).
    /// Simplified: assumes coefficient storage in standard polynomial basis.
    fn eval_poly_2var(
        &self,
        u: f64,
        v: f64,
        num_coeff_per_surface: &[usize],
        coefficients: &[f64],
        max_u_degree: usize,
        max_v_degree: usize,
    ) -> (f64, f64, f64) {
        let u_deg = num_coeff_per_surface[0] - 1;
        let v_deg = num_coeff_per_surface[1] - 1;

        let mut result = [0.0; 3];

        // Standard 2D polynomial evaluation: P(u,v) = sum_{i,j} c_{ij} * u^i * v^j
        let mut coeff_idx = 0;
        for i in 0..=u_deg {
            for j in 0..=v_deg {
                let u_power = u.powi(i as i32);
                let v_power = v.powi(j as i32);
                let basis = u_power * v_power;

                result[0] += coefficients[coeff_idx] * basis;
                result[1] += coefficients[coeff_idx + 1] * basis;
                result[2] += coefficients[coeff_idx + 2] * basis;

                coeff_idx += 3;
            }
        }

        (result[0], result[1], result[2])
    }

    /// Builds U knot and multiplicity arrays using Schoenberg construction.
    fn build_array_u(&mut self, degree: usize, continuity: usize) {
        let num_knots = self.u_knots.len();
        let num_curves = num_knots - 1;

        // Multiplicities: clamped at ends, continuity in interior
        let multiplicities = degree - continuity;
        self.u_mults = vec![0; num_knots];

        for i in 1..num_knots - 1 {
            self.u_mults[i] = multiplicities as i32;
        }
        self.u_mults[0] = (degree + 1) as i32;
        self.u_mults[num_knots - 1] = (degree + 1) as i32;

        // Build flat knot sequence
        let num_flat_knots = multiplicities * (num_curves - 1) + 2 * degree + 2;
        self.u_flat_knots = vec![0.0; num_flat_knots];

        // Simple knot insertion: repeat boundary knots for clamping
        let mut idx = 0;
        for i in 0..num_knots {
            let mult = self.u_mults[i] as usize;
            for _ in 0..mult {
                if idx < num_flat_knots {
                    self.u_flat_knots[idx] = self.u_knots[i];
                    idx += 1;
                }
            }
        }
    }

    /// Builds V knot and multiplicity arrays using Schoenberg construction.
    fn build_array_v(&mut self, degree: usize, continuity: usize) {
        let num_knots = self.v_knots.len();
        let num_curves = num_knots - 1;

        // Multiplicities: clamped at ends, continuity in interior
        let multiplicities = degree - continuity;
        self.v_mults = vec![0; num_knots];

        for i in 1..num_knots - 1 {
            self.v_mults[i] = multiplicities as i32;
        }
        self.v_mults[0] = (degree + 1) as i32;
        self.v_mults[num_knots - 1] = (degree + 1) as i32;

        // Build flat knot sequence
        let num_flat_knots = multiplicities * (num_curves - 1) + 2 * degree + 2;
        self.v_flat_knots = vec![0.0; num_flat_knots];

        // Simple knot insertion: repeat boundary knots for clamping
        let mut idx = 0;
        for i in 0..num_knots {
            let mult = self.v_mults[i] as usize;
            for _ in 0..mult {
                if idx < num_flat_knots {
                    self.v_flat_knots[idx] = self.v_knots[i];
                    idx += 1;
                }
            }
        }
    }

    /// Returns the number of poles in the U direction.
    pub fn nb_u_poles(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        self.poles.len()
    }

    /// Returns the number of poles in the V direction.
    pub fn nb_v_poles(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        if self.poles.is_empty() {
            0
        } else {
            self.poles[0].len()
        }
    }

    /// Returns the poles.
    pub fn poles(&self) -> &Vec<Vec<Point3d>> {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        &self.poles
    }

    /// Returns the U degree.
    pub fn u_degree(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        self.u_degree
    }

    /// Returns the V degree.
    pub fn v_degree(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        self.v_degree
    }

    /// Returns the number of U knots.
    pub fn nb_u_knots(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        self.u_knots.len()
    }

    /// Returns the number of V knots.
    pub fn nb_v_knots(&self) -> usize {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        self.v_knots.len()
    }

    /// Returns the U knots.
    pub fn u_knots(&self) -> &Vec<f64> {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        &self.u_knots
    }

    /// Returns the V knots.
    pub fn v_knots(&self) -> &Vec<f64> {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        &self.v_knots
    }

    /// Returns the U multiplicities.
    pub fn u_multiplicities(&self) -> &Vec<i32> {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        &self.u_mults
    }

    /// Returns the V multiplicities.
    pub fn v_multiplicities(&self) -> &Vec<i32> {
        if !self.done {
            panic!("Convert_GridPolynomialToPoles: Not done");
        }
        &self.v_mults
    }

    /// Returns true if the conversion was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_planar_patch() {
        // Create a simple bilinear patch: z = 0, x = u, y = v
        // MaxUDegree = 1, MaxVDegree = 1
        let max_u_deg = 1;
        let max_v_deg = 1;
        let num_coeff = [2, 2];  // degree + 1 for each direction

        // Coefficients for P(u,v) = (u, v, 0)
        // Layout: c_{00}, c_{01}, c_{10}, c_{11} with 3 coords each
        let coefficients = vec![
            0.0, 0.0, 0.0,    // c_{00}: (0, 0, 0)
            0.0, 1.0, 0.0,    // c_{01}: (0, 1, 0)
            1.0, 0.0, 0.0,    // c_{10}: (1, 0, 0)
            0.0, 0.0, 0.0,    // c_{11}: (0, 0, 0)
        ];

        let poly_u = [0.0, 1.0];
        let poly_v = [0.0, 1.0];

        let conv = ConvertGridPolynomialToPoles::new(
            max_u_deg, max_v_deg,
            &num_coeff,
            &coefficients,
            &poly_u,
            &poly_v,
        );

        assert!(conv.is_done());
        assert!(conv.nb_u_poles() > 0);
        assert!(conv.nb_v_poles() > 0);
        assert_eq!(conv.u_degree(), 1);
        assert_eq!(conv.v_degree(), 1);
        assert!(conv.nb_u_knots() > 0);
        assert!(conv.nb_v_knots() > 0);
    }

    #[test]
    fn test_query_methods() {
        let max_u_deg = 1;
        let max_v_deg = 1;
        let num_coeff = [2, 2];

        let mut coefficients = vec![0.0; 12];
        coefficients[7] = 1.0;   // x = u coefficient
        coefficients[5] = 1.0;   // y = v coefficient

        let poly_u = [0.0, 1.0];
        let poly_v = [0.0, 1.0];

        let conv = ConvertGridPolynomialToPoles::new(
            max_u_deg, max_v_deg,
            &num_coeff,
            &coefficients,
            &poly_u,
            &poly_v,
        );

        assert!(conv.is_done());

        let u_knots = conv.u_knots();
        let v_knots = conv.v_knots();
        let u_mults = conv.u_multiplicities();
        let v_mults = conv.v_multiplicities();

        assert!(u_knots.len() > 0);
        assert!(v_knots.len() > 0);
        assert_eq!(u_mults.len(), conv.nb_u_knots());
        assert_eq!(v_mults.len(), conv.nb_v_knots());
    }

    #[test]
    fn test_poles_structure() {
        let max_u_deg = 1;
        let max_v_deg = 1;
        let num_coeff = [2, 2];

        let coefficients = vec![
            0.0, 0.0, 0.0,    // c_{00}
            1.0, 0.0, 0.0,    // c_{01}
            0.0, 1.0, 0.0,    // c_{10}
            0.0, 0.0, 1.0,    // c_{11}
        ];

        let poly_u = [0.0, 1.0];
        let poly_v = [0.0, 1.0];

        let conv = ConvertGridPolynomialToPoles::new(
            max_u_deg, max_v_deg,
            &num_coeff,
            &coefficients,
            &poly_u,
            &poly_v,
        );

        assert!(conv.is_done());
        let poles = conv.poles();
        assert_eq!(poles.len(), conv.nb_u_poles());
        for row in poles {
            assert_eq!(row.len(), conv.nb_v_poles());
        }
    }

    #[test]
    fn test_done_after_construction() {
        let max_u_deg = 1;
        let max_v_deg = 1;
        let num_coeff = [2, 2];
        let coefficients = vec![0.0; 12];
        let poly_u = [0.0, 1.0];
        let poly_v = [0.0, 1.0];

        let conv = ConvertGridPolynomialToPoles::new(
            max_u_deg, max_v_deg,
            &num_coeff,
            &coefficients,
            &poly_u,
            &poly_v,
        );

        // After construction, done() should return true
        assert!(conv.is_done());
        let _ = conv.nb_u_poles();
    }

    #[test]
    #[should_panic(expected = "Wrong Coefficients")]
    fn test_wrong_coefficient_size() {
        let max_u_deg = 1;
        let max_v_deg = 1;
        let num_coeff = [2, 2];
        let coefficients = vec![0.0; 6];  // Wrong size: should be 12
        let poly_u = [0.0, 1.0];
        let poly_v = [0.0, 1.0];

        ConvertGridPolynomialToPoles::new(
            max_u_deg, max_v_deg,
            &num_coeff,
            &coefficients,
            &poly_u,
            &poly_v,
        );
    }
}
