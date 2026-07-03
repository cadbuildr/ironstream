// FILE: int_polyh_array.rs
// occt: IntPolyh_Array

/// Implementation of IntPolyh_Array
pub struct IntPolyh_Array;

impl IntPolyh_Array {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPolyh_Array
    }
}

impl Default for IntPolyh_Array {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPolyh_Array::new();
    }
}
