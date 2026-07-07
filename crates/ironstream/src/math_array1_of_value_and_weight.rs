// FILE: math_array1_of_value_and_weight.rs
// occt: math_Array1OfValueAndWeight

/// 1D array of pairs (value, weight) for weighted mathematical operations.
/// Used in interpolation, approximation, and curve fitting algorithms.
#[derive(Debug, Clone)]
pub struct MathArray1OfValueAndWeight {
    values: Vec<f64>,
    weights: Vec<f64>,
    lower_index: usize,
}

impl MathArray1OfValueAndWeight {
    /// Create an array with given capacity and lower index.
    /// All values and weights are initialized to 0.0.
    pub fn new(lower: usize, upper: usize) -> Result<Self, String> {
        if lower > upper {
            return Err("Lower index must be <= upper index".to_string());
        }
        let size = upper - lower + 1;
        Ok(MathArray1OfValueAndWeight {
            values: vec![0.0; size],
            weights: vec![0.0; size],
            lower_index: lower,
        })
    }

    /// Get the lower bound index.
    pub fn lower_bound(&self) -> usize {
        self.lower_index
    }

    /// Get the upper bound index.
    pub fn upper_bound(&self) -> usize {
        self.lower_index + self.values.len() - 1
    }

    /// Get the number of elements in the array.
    pub fn length(&self) -> usize {
        self.values.len()
    }

    /// Set a value and weight at the given index.
    pub fn set(&mut self, index: usize, value: f64, weight: f64) -> Result<(), String> {
        let local_index = self.to_local_index(index)?;
        self.values[local_index] = value;
        self.weights[local_index] = weight;
        Ok(())
    }

    /// Get the value at the given index.
    pub fn value(&self, index: usize) -> Result<f64, String> {
        let local_index = self.to_local_index(index)?;
        Ok(self.values[local_index])
    }

    /// Get the weight at the given index.
    pub fn weight(&self, index: usize) -> Result<f64, String> {
        let local_index = self.to_local_index(index)?;
        Ok(self.weights[local_index])
    }

    /// Get both value and weight at the given index.
    pub fn value_and_weight(&self, index: usize) -> Result<(f64, f64), String> {
        let local_index = self.to_local_index(index)?;
        Ok((self.values[local_index], self.weights[local_index]))
    }

    /// Calculate the weighted sum of values.
    pub fn weighted_sum(&self) -> f64 {
        self.values.iter().zip(self.weights.iter())
            .map(|(v, w)| v * w)
            .sum()
    }

    /// Calculate the total weight sum.
    pub fn total_weight(&self) -> f64 {
        self.weights.iter().sum()
    }

    /// Calculate the weighted average.
    /// Returns 0.0 if total weight is near-zero.
    pub fn weighted_average(&self) -> f64 {
        let total_weight = self.total_weight();
        if total_weight < 1e-10 {
            0.0
        } else {
            self.weighted_sum() / total_weight
        }
    }

    /// Normalize weights so they sum to 1.0.
    pub fn normalize_weights(&mut self) -> Result<(), String> {
        let total = self.total_weight();
        if total < 1e-10 {
            return Err("Cannot normalize: total weight is near-zero".to_string());
        }
        for w in &mut self.weights {
            *w /= total;
        }
        Ok(())
    }

    /// Check if all weights are non-negative.
    pub fn all_weights_positive(&self) -> bool {
        self.weights.iter().all(|w| *w >= 0.0)
    }

    /// Find the index with maximum weight.
    pub fn max_weight_index(&self) -> Result<usize, String> {
        if self.weights.is_empty() {
            return Err("Empty array".to_string());
        }
        let max_idx = self.weights.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .ok_or("No maximum found")?;
        Ok(self.lower_index + max_idx)
    }

    /// Find the index with minimum weight.
    pub fn min_weight_index(&self) -> Result<usize, String> {
        if self.weights.is_empty() {
            return Err("Empty array".to_string());
        }
        let min_idx = self.weights.iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .ok_or("No minimum found")?;
        Ok(self.lower_index + min_idx)
    }

    /// Convert external index to local vector index.
    fn to_local_index(&self, index: usize) -> Result<usize, String> {
        if index < self.lower_index || index > self.upper_bound() {
            return Err(format!(
                "Index {} out of bounds [{}, {}]",
                index,
                self.lower_index,
                self.upper_bound()
            ));
        }
        Ok(index - self.lower_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let arr = MathArray1OfValueAndWeight::new(0, 9);
        assert!(arr.is_ok());
        let arr = arr.unwrap();
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_new_invalid() {
        let arr = MathArray1OfValueAndWeight::new(10, 5);
        assert!(arr.is_err());
    }

    #[test]
    fn test_bounds() {
        let arr = MathArray1OfValueAndWeight::new(5, 14).unwrap();
        assert_eq!(arr.lower_bound(), 5);
        assert_eq!(arr.upper_bound(), 14);
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_set_and_value() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 4).unwrap();
        assert!(arr.set(2, 3.5, 2.0).is_ok());
        assert_eq!(arr.value(2).unwrap(), 3.5);
        assert_eq!(arr.weight(2).unwrap(), 2.0);
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 4).unwrap();
        assert!(arr.set(10, 1.0, 1.0).is_err());
    }

    #[test]
    fn test_value_and_weight() {
        let mut arr = MathArray1OfValueAndWeight::new(1, 5).unwrap();
        arr.set(3, 7.5, 1.5).unwrap();
        let (v, w) = arr.value_and_weight(3).unwrap();
        assert_eq!(v, 7.5);
        assert_eq!(w, 1.5);
    }

    #[test]
    fn test_weighted_sum() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 10.0, 2.0).unwrap();
        arr.set(1, 20.0, 3.0).unwrap();
        arr.set(2, 30.0, 1.0).unwrap();
        let sum = arr.weighted_sum();
        assert_eq!(sum, 10.0 * 2.0 + 20.0 * 3.0 + 30.0 * 1.0);
    }

    #[test]
    fn test_total_weight() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 10.0, 2.0).unwrap();
        arr.set(1, 20.0, 3.0).unwrap();
        arr.set(2, 30.0, 1.0).unwrap();
        assert_eq!(arr.total_weight(), 6.0);
    }

    #[test]
    fn test_weighted_average() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 10.0, 2.0).unwrap();
        arr.set(1, 20.0, 3.0).unwrap();
        arr.set(2, 40.0, 1.0).unwrap();
        let avg = arr.weighted_average();
        // (10*2 + 20*3 + 40*1) / (2+3+1) = 120 / 6 = 20
        assert!((avg - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_weighted_average_zero_weight() {
        let arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        assert_eq!(arr.weighted_average(), 0.0);
    }

    #[test]
    fn test_normalize_weights() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 1.0, 10.0).unwrap();
        arr.set(1, 1.0, 20.0).unwrap();
        arr.set(2, 1.0, 70.0).unwrap();

        assert!(arr.normalize_weights().is_ok());
        assert!((arr.weight(0).unwrap() - 0.1).abs() < 1e-6);
        assert!((arr.weight(1).unwrap() - 0.2).abs() < 1e-6);
        assert!((arr.weight(2).unwrap() - 0.7).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_weights_zero_total() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        let result = arr.normalize_weights();
        assert!(result.is_err());
    }

    #[test]
    fn test_all_weights_positive_true() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 1.0, 1.0).unwrap();
        arr.set(1, 1.0, 2.0).unwrap();
        arr.set(2, 1.0, 3.0).unwrap();
        assert!(arr.all_weights_positive());
    }

    #[test]
    fn test_all_weights_positive_false() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 2).unwrap();
        arr.set(0, 1.0, 1.0).unwrap();
        arr.set(1, 1.0, -2.0).unwrap();
        arr.set(2, 1.0, 3.0).unwrap();
        assert!(!arr.all_weights_positive());
    }

    #[test]
    fn test_max_weight_index() {
        let mut arr = MathArray1OfValueAndWeight::new(5, 9).unwrap();
        arr.set(5, 1.0, 1.0).unwrap();
        arr.set(6, 1.0, 5.0).unwrap();
        arr.set(7, 1.0, 2.0).unwrap();
        arr.set(8, 1.0, 3.0).unwrap();
        arr.set(9, 1.0, 0.5).unwrap();

        assert_eq!(arr.max_weight_index().unwrap(), 6);
    }

    #[test]
    fn test_min_weight_index() {
        let mut arr = MathArray1OfValueAndWeight::new(0, 4).unwrap();
        arr.set(0, 1.0, 5.0).unwrap();
        arr.set(1, 1.0, 2.0).unwrap();
        arr.set(2, 1.0, 0.5).unwrap();
        arr.set(3, 1.0, 3.0).unwrap();
        arr.set(4, 1.0, 1.0).unwrap();

        assert_eq!(arr.min_weight_index().unwrap(), 2);
    }

    #[test]
    fn test_custom_lower_bound() {
        let mut arr = MathArray1OfValueAndWeight::new(100, 102).unwrap();
        assert!(arr.set(100, 1.5, 0.5).is_ok());
        assert!(arr.set(101, 2.5, 0.5).is_ok());
        assert!(arr.set(102, 3.5, 0.5).is_ok());
        assert!(arr.set(99, 1.0, 1.0).is_err());
        assert!(arr.set(103, 1.0, 1.0).is_err());
    }

    #[test]
    fn test_empty_array() {
        let arr = MathArray1OfValueAndWeight::new(5, 4);
        assert!(arr.is_err());
    }
}
