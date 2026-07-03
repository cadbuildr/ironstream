// FILE: n_collection_map_algo.rs
// occt: NCollection_MapAlgo

/// Algorithms and utilities for managing map operations.
/// Provides set operations like union, intersection, subtraction, difference.
pub mod map_algo {
    /// Check if two collections have the same elements.
    pub fn is_equal<T: PartialEq>(set1: &[T], set2: &[T]) -> bool {
        if set1.len() != set2.len() {
            return false;
        }
        set1.iter().all(|x| set2.contains(x))
    }

    /// Check if first set contains all elements of second set.
    pub fn contains<T: PartialEq>(set1: &[T], set2: &[T]) -> bool {
        set2.iter().all(|x| set1.contains(x))
    }

    /// Union of two sets (all elements from both).
    pub fn union<T: Clone + PartialEq>(set1: &[T], set2: &[T]) -> Vec<T> {
        let mut result = set1.to_vec();
        for item in set2 {
            if !result.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }

    /// Intersection of two sets (only common elements).
    pub fn intersection<T: Clone + PartialEq>(set1: &[T], set2: &[T]) -> Vec<T> {
        set1.iter()
            .filter(|x| set2.contains(x))
            .cloned()
            .collect()
    }

    /// Subtraction (elements in set1 but not in set2).
    pub fn subtract<T: Clone + PartialEq>(set1: &[T], set2: &[T]) -> Vec<T> {
        set1.iter()
            .filter(|x| !set2.contains(x))
            .cloned()
            .collect()
    }

    /// Symmetric difference (elements in either but not both).
    pub fn difference<T: Clone + PartialEq>(set1: &[T], set2: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        for item in set1 {
            if !set2.contains(item) {
                result.push(item.clone());
            }
        }
        for item in set2 {
            if !set1.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::map_algo::*;

    #[test]
    fn test_is_equal() {
        assert!(is_equal(&[1, 2, 3], &[1, 2, 3]));
        assert!(!is_equal(&[1, 2], &[1, 2, 3]));
    }

    #[test]
    fn test_contains() {
        assert!(contains(&[1, 2, 3], &[2, 3]));
        assert!(!contains(&[1, 2], &[1, 3]));
    }

    #[test]
    fn test_union() {
        let result = union(&[1, 2], &[2, 3]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_intersection() {
        let result = intersection(&[1, 2, 3], &[2, 3, 4]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_difference() {
        let result = difference(&[1, 2, 3], &[2, 3, 4]);
        assert!(result.contains(&1));
        assert!(result.contains(&4));
        assert_eq!(result.len(), 2);
    }
}
