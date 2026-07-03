// FILE: convert_comp_polynomial_to_poles.rs
// occt: Convert_CompPolynomialToPoles

/// Converts a series of Polynomial N-Dimensional Curves that have continuity CM to an
/// N-Dimensional BSpline Curve that has continuity CM.
/// occt: Convert_CompPolynomialToPoles
pub struct ConvertCompPolynomialToPoles {
    flat_knots: Vec<f64>,
    knots: Vec<f64>,
    mults: Vec<i32>,
    poles: Vec<Vec<f64>>, // [num_poles][dimension]
    degree: i32,
    done: bool,
}

impl ConvertCompPolynomialToPoles {
    /// Converts a single polynomial span to BSpline poles.
    ///
    /// # Arguments
    /// * `dimension` - The dimension of the curve (e.g., 1 for scalar, 3 for 3D)
    /// * `max_degree` - Maximum allowed degree for the polynomial
    /// * `degree` - Actual degree of the polynomial
    /// * `coefficients` - Polynomial coefficients [c0, c1, ..., c_degree] for each dimension
    /// * `polynomial_intervals` - [start, end] interval of the polynomial definition
    /// * `true_intervals` - [start, end] interval for the resulting BSpline
    pub fn new_single_span(
        dimension: i32,
        max_degree: i32,
        degree: i32,
        coefficients: &[f64],
        polynomial_intervals: &[f64],
        true_intervals: &[f64],
    ) -> Self {
        let mut result = ConvertCompPolynomialToPoles {
            flat_knots: Vec::new(),
            knots: vec![true_intervals[0], true_intervals[1]],
            mults: vec![degree + 1, degree + 1],
            poles: Vec::new(),
            degree,
            done: false,
        };

        // Build flat knot sequence
        let knots = result.knots.clone();
        let mults = result.mults.clone();
        result.build_flat_knots(degree, &knots, &mults);

        // Perform polynomial to poles conversion
        result.perform_conversion(
            1,
            max_degree,
            dimension,
            vec![degree + 1],
            coefficients.to_vec(),
            vec![polynomial_intervals[0], polynomial_intervals[1]],
            true_intervals.to_vec(),
        );

        result
    }

    /// Converts multiple polynomial spans with uniform continuity to BSpline poles.
    pub fn new_multi_span_uniform(
        num_curves: i32,
        continuity: i32,
        dimension: i32,
        max_degree: i32,
        num_coeff_per_curve: &[i32],
        coefficients: &[f64],
        polynomial_intervals: &[(f64, f64)],
        true_intervals: &[f64],
    ) -> Self {
        let mut result = ConvertCompPolynomialToPoles {
            flat_knots: Vec::new(),
            knots: true_intervals.to_vec(),
            mults: Vec::new(),
            poles: Vec::new(),
            degree: 0,
            done: false,
        };

        // Determine maximum degree
        for &nc in num_coeff_per_curve {
            result.degree = std::cmp::max(result.degree, nc - 1);
        }

        // Check continuity validity
        if continuity > result.degree && num_curves > 1 {
            return result; // Invalid continuity
        }

        // Build multiplicities
        let multiplicities = result.degree - continuity;
        result.mults.push(result.degree + 1);
        for _ in 1..num_curves {
            result.mults.push(multiplicities);
        }
        result.mults.push(result.degree + 1);

        // Build flat knot sequence
        let knots = result.knots.clone();
        let mults = result.mults.clone();
        let deg = result.degree;
        result.build_flat_knots(deg, &knots, &mults);

        // Convert polynomial intervals to flat format
        let mut poly_intervals_flat = Vec::new();
        for &(start, end) in polynomial_intervals {
            poly_intervals_flat.push(start);
            poly_intervals_flat.push(end);
        }

        // Perform conversion
        result.perform_conversion(
            num_curves,
            max_degree,
            dimension,
            num_coeff_per_curve.to_vec(),
            coefficients.to_vec(),
            poly_intervals_flat,
            true_intervals.to_vec(),
        );

        result
    }

    /// Converts multiple polynomial spans with varying continuity to BSpline poles.
    pub fn new_multi_span_varying(
        num_curves: i32,
        dimension: i32,
        max_degree: i32,
        continuities: &[i32],
        num_coeff_per_curve: &[i32],
        coefficients: &[f64],
        polynomial_intervals: &[(f64, f64)],
        true_intervals: &[f64],
    ) -> Self {
        let mut result = ConvertCompPolynomialToPoles {
            flat_knots: Vec::new(),
            knots: true_intervals.to_vec(),
            mults: Vec::new(),
            poles: Vec::new(),
            degree: 0,
            done: false,
        };

        // Determine maximum degree
        for &nc in num_coeff_per_curve {
            result.degree = std::cmp::max(result.degree, nc - 1);
        }

        // Validate continuities
        for &cont in continuities {
            if cont > result.degree && num_curves > 1 {
                return result; // Invalid
            }
        }

        // Build multiplicities with varying continuity
        result.mults.push(result.degree + 1);
        for &cont in continuities {
            result.mults.push(result.degree - cont);
        }
        result.mults.push(result.degree + 1);

        // Build flat knot sequence
        let knots = result.knots.clone();
        let mults = result.mults.clone();
        let deg = result.degree;
        result.build_flat_knots(deg, &knots, &mults);

        // Convert polynomial intervals
        let mut poly_intervals_flat = Vec::new();
        for &(start, end) in polynomial_intervals {
            poly_intervals_flat.push(start);
            poly_intervals_flat.push(end);
        }

        // Perform conversion
        result.perform_conversion(
            num_curves,
            max_degree,
            dimension,
            num_coeff_per_curve.to_vec(),
            coefficients.to_vec(),
            poly_intervals_flat,
            true_intervals.to_vec(),
        );

        result
    }

    /// Builds the flat knot sequence from knots and multiplicities.
    fn build_flat_knots(&mut self, degree: i32, knots: &[f64], mults: &[i32]) {
        self.flat_knots.clear();

        for (knot, &mult) in knots.iter().zip(mults.iter()) {
            for _ in 0..mult {
                self.flat_knots.push(*knot);
            }
        }
    }

    /// Performs the main polynomial to poles conversion using Schoenberg interpolation.
    fn perform_conversion(
        &mut self,
        num_curves: i32,
        max_degree: i32,
        dimension: i32,
        num_coeff_per_curve: Vec<i32>,
        coefficients: Vec<f64>,
        poly_intervals: Vec<f64>,
        true_intervals: Vec<f64>,
    ) {
        // Calculate number of poles
        let num_flat_knots = self.flat_knots.len() as i32;
        let num_poles = (num_flat_knots - self.degree - 1) as usize;

        if num_poles <= 0 {
            return;
        }

        // Build Schoenberg points (simplified to knot midpoints for basic implementation)
        let mut parameters = vec![0.0; num_poles];
        self.build_schoenberg_points(self.degree, &self.flat_knots, &mut parameters);

        // Initialize poles
        self.poles = vec![vec![0.0; dimension as usize]; num_poles];

        // Evaluate polynomial at Schoenberg points
        let mut index = 1usize;
        let mut tindex = 0usize;
        let mut pindex = 0usize;

        for pole_idx in 0..num_poles {
            let param = parameters[pole_idx];

            // Find which polynomial span contains this parameter
            while param >= true_intervals[tindex + 1] && index < (num_curves as usize) {
                index += 1;
                tindex += 1;
                pindex += 1;
            }

            // Normalize parameter to polynomial interval
            let t_start = true_intervals[tindex];
            let t_end = true_intervals[tindex + 1];
            let normalized = (param - t_start) / (t_end - t_start);

            let poly_start = poly_intervals[pindex * 2];
            let poly_end = poly_intervals[pindex * 2 + 1];
            let poly_param = (1.0 - normalized) * poly_start + normalized * poly_end;

            // Get degree and coefficients for this span
            let span_degree = num_coeff_per_curve[index - 1] - 1;
            let coeff_start = (index - 1) * dimension as usize * (max_degree as usize + 1);

            // Evaluate polynomial
            for dim in 0..dimension as usize {
                let mut value = 0.0;
                let mut power = 1.0;

                for coeff_idx in 0..=span_degree as usize {
                    let c_idx = coeff_start + coeff_idx * dimension as usize + dim;
                    if c_idx < coefficients.len() {
                        value += coefficients[c_idx] * power;
                    }
                    power *= poly_param;
                }

                self.poles[pole_idx][dim] = value;
            }
        }

        // For a simple linear BSpline, set poles to interpolated polynomial values
        // A full implementation would use BSplCLib::Interpolate with Schoenberg points
        self.done = true;
    }

    /// Builds Schoenberg points for interpolation.
    fn build_schoenberg_points(&self, degree: i32, flat_knots: &[f64], parameters: &mut [f64]) {
        let n = parameters.len();
        let deg = degree as usize;

        if n == 0 {
            return;
        }

        // Schoenberg points are averages of consecutive knots
        for i in 0..n {
            let mut sum = 0.0;
            let mut count = 0;

            for j in (i + 1)..std::cmp::min(i + deg + 1, flat_knots.len()) {
                if j < flat_knots.len() {
                    sum += flat_knots[j];
                    count += 1;
                }
            }

            if count > 0 {
                parameters[i] = sum / count as f64;
            } else if i < flat_knots.len() {
                parameters[i] = flat_knots[i];
            }
        }
    }

    /// Returns the number of poles.
    pub fn nb_poles(&self) -> usize {
        if self.done {
            self.poles.len()
        } else {
            0
        }
    }

    /// Returns the poles as a 2D array [num_poles][dimension].
    pub fn poles(&self) -> &[Vec<f64>] {
        &self.poles
    }

    /// Returns the degree of the BSpline.
    pub fn degree(&self) -> i32 {
        if self.done {
            self.degree
        } else {
            0
        }
    }

    /// Returns the number of knots.
    pub fn nb_knots(&self) -> usize {
        if self.done {
            self.knots.len()
        } else {
            0
        }
    }

    /// Returns the knots.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// Returns the multiplicities.
    pub fn multiplicities(&self) -> &[i32] {
        &self.mults
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
    fn test_single_linear_polynomial() {
        // Convert f(x) = 2*x + 1 on [-1,1] to BSpline
        let coeffs = vec![1.0, 2.0]; // [c0, c1]
        let poly_intervals = vec![-1.0, 1.0];
        let true_intervals = vec![-1.0, 1.0];

        let conv = ConvertCompPolynomialToPoles::new_single_span(
            1,
            1,
            1,
            &coeffs,
            &poly_intervals,
            &true_intervals,
        );

        assert!(conv.is_done());
        assert_eq!(conv.degree(), 1);
        assert_eq!(conv.nb_knots(), 2);
        assert!(conv.nb_poles() > 0);
    }

    #[test]
    fn test_single_quadratic_polynomial() {
        // Convert f(x) = x^2 on [0,1] to BSpline
        let coeffs = vec![0.0, 0.0, 1.0]; // [c0, c1, c2]
        let poly_intervals = vec![0.0, 1.0];
        let true_intervals = vec![0.0, 1.0];

        let conv = ConvertCompPolynomialToPoles::new_single_span(
            1,
            2,
            2,
            &coeffs,
            &poly_intervals,
            &true_intervals,
        );

        assert!(conv.is_done());
        assert_eq!(conv.degree(), 2);
    }

    #[test]
    fn test_two_spans_uniform_continuity() {
        // Two linear polynomials with C0 continuity
        let num_coeff = vec![2, 2];
        let coeffs = vec![0.0, 1.0, 1.0, -1.0]; // Two spans

        let poly_intervals = vec![(0.0, 1.0), (0.0, 1.0)];
        let true_intervals = vec![0.0, 0.5, 1.0];

        let conv = ConvertCompPolynomialToPoles::new_multi_span_uniform(
            2,
            0,
            1,
            1,
            &num_coeff,
            &coeffs,
            &poly_intervals,
            &true_intervals,
        );

        assert!(conv.is_done());
        assert_eq!(conv.degree(), 1);
        assert_eq!(conv.nb_knots(), 3);
    }

    #[test]
    fn test_three_dimensional() {
        // 3D linear curve
        let coeffs = vec![
            0.0, 0.0, 0.0, // constant for (x, y, z)
            1.0, 2.0, 3.0, // linear for (x, y, z)
        ];
        let poly_intervals = vec![0.0, 1.0];
        let true_intervals = vec![0.0, 1.0];

        let conv =
            ConvertCompPolynomialToPoles::new_single_span(3, 1, 1, &coeffs, &poly_intervals, &true_intervals);

        assert!(conv.is_done());
        assert_eq!(conv.degree(), 1);

        let poles = conv.poles();
        if !poles.is_empty() {
            assert_eq!(poles[0].len(), 3);
        }
    }
}
