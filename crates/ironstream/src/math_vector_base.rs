// FILE: math_vector_base.rs
// occt: math_VectorBase

/// Base class for mathematical vectors.
pub struct VectorBase {
    data: Vec<f64>,
}

impl VectorBase {
    pub fn new(size: usize) -> Self {
        VectorBase {
            data: vec![0.0; size],
        }
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        VectorBase { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, i: usize) -> Option<f64> {
        self.data.get(i).copied()
    }

    pub fn set(&mut self, i: usize, val: f64) {
        if i < self.data.len() {
            self.data[i] = val;
        }
    }

    pub fn norm(&self) -> f64 {
        let mut sum = 0.0;
        for &x in &self.data {
            sum += x * x;
        }
        sum.sqrt()
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        if n > 1e-15 {
            for x in &mut self.data {
                *x /= n;
            }
        }
    }

    pub fn dot(&self, other: &VectorBase) -> f64 {
        let mut sum = 0.0;
        for (a, b) in self.data.iter().zip(other.data.iter()) {
            sum += a * b;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_base_new() {
        let v = VectorBase::new(3);
        assert_eq!(v.size(), 3);
    }

    #[test]
    fn test_vector_base_get_set() {
        let mut v = VectorBase::new(3);
        v.set(0, 1.0);
        assert_eq!(v.get(0), Some(1.0));
    }

    #[test]
    fn test_vector_base_norm() {
        let v = VectorBase::from_vec(vec![3.0, 4.0]);
        assert!((v.norm() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_base_dot() {
        let v1 = VectorBase::from_vec(vec![1.0, 2.0]);
        let v2 = VectorBase::from_vec(vec![3.0, 4.0]);
        assert_eq!(v1.dot(&v2), 11.0);
    }
}
