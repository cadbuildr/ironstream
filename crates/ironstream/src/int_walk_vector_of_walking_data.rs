// FILE: int_walk_vector_of_walking_data.rs
// occt: IntWalk_VectorOfWalkingData

/// Vector of walking data structures
#[derive(Clone, Debug)]
pub struct VectorOfWalkingData {
    data: Vec<WalkingData>,
}

/// Single walking data entry
#[derive(Clone, Debug, Copy)]
pub struct WalkingData {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
    step_size: f64,
}

impl WalkingData {
    pub fn new() -> Self {
        WalkingData {
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
            step_size: 0.01,
        }
    }

    pub fn with_params(u1: f64, v1: f64, u2: f64, v2: f64, step: f64) -> Self {
        WalkingData { u1, v1, u2, v2, step_size: step }
    }
}

impl Default for WalkingData {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorOfWalkingData {
    /// Create new vector
    pub fn new() -> Self {
        VectorOfWalkingData { data: Vec::new() }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        VectorOfWalkingData {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Append data
    pub fn append(&mut self, data: WalkingData) {
        self.data.push(data);
    }

    /// Get value at index
    pub fn value(&self, index: usize) -> Option<WalkingData> {
        self.data.get(index).copied()
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for VectorOfWalkingData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = VectorOfWalkingData::new();
        assert!(v.is_empty());
    }

    #[test]
    fn test_walking_data() {
        let wd = WalkingData::with_params(0.5, 0.6, 0.7, 0.8, 0.01);
        assert_eq!(wd.u1, 0.5);
        assert_eq!(wd.step_size, 0.01);
    }

    #[test]
    fn test_append() {
        let mut v = VectorOfWalkingData::new();
        let wd = WalkingData::new();
        v.append(wd);
        assert_eq!(v.len(), 1);
    }
}
