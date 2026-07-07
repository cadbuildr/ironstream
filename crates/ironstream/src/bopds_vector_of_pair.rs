// FILE: bopds_vector_of_pair.rs
// occt: BOPDS_VectorOfPair

//! Deprecated NCollection alias: Vector<Pair>

/// Pair data structure (stub).
#[derive(Clone, Debug)]
pub struct Pair {
    pub first: u32,
    pub second: u32,
}

/// Vector of pairs.
pub type BopdsVectorOfPair = Vec<Pair>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let mut vec: BopdsVectorOfPair = Vec::new();
        vec.push(Pair { first: 1, second: 2 });
        assert_eq!(vec.len(), 1);
    }
}
