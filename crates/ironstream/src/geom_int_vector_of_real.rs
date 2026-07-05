// FILE: geom_int_vector_of_real.rs
// occt: GeomInt_VectorOfReal

//! Deprecated: Use Vec<f64> directly.
//! Alias for backward compatibility with OCCT.

pub type GeomIntVectorOfReal = Vec<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let mut vec: GeomIntVectorOfReal = Vec::new();
        assert_eq!(vec.len(), 0);

        vec.push(1.5);
        vec.push(2.5);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1.5);
        assert_eq!(vec[1], 2.5);
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: GeomIntVectorOfReal = Vec::new();
        vec.extend_from_slice(&[1.0, 2.0, 3.0]);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.iter().sum::<f64>(), 6.0);
    }

    #[test]
    fn test_vector_append() {
        let mut vec: GeomIntVectorOfReal = vec![1.0, 2.0];
        vec.push(3.0);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.last(), Some(&3.0));
    }
}
