// FILE: bopds_vector_of_pave.rs
// occt: BOPDS_VectorOfPave

//! Deprecated NCollection alias: Vector<Pave>

/// Pave (point on edge) data (stub).
#[derive(Clone, Debug)]
pub struct Pave {
    pub param: f64,
    pub vertex_id: u32,
}

/// Vector of paves.
pub type BopdsVectorOfPave = Vec<Pave>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let mut vec: BopdsVectorOfPave = Vec::new();
        vec.push(Pave { param: 0.5, vertex_id: 1 });
        assert_eq!(vec.len(), 1);
    }
}
