// FILE: vrml_data_array_vec3d.rs
// occt: VrmlData_ArrayVec3d

#[derive(Clone, Debug)]
pub struct VrmlDataArrayVec3d {
    data: Vec<(f64, f64, f64)>,
}

impl VrmlDataArrayVec3d {
    pub fn new() -> Self {
        VrmlDataArrayVec3d { data: Vec::new() }
    }

    pub fn append(&mut self, x: f64, y: f64, z: f64) {
        self.data.push((x, y, z));
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, idx: usize) -> Option<(f64, f64, f64)> {
        self.data.get(idx).copied()
    }
}

impl Default for VrmlDataArrayVec3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = VrmlDataArrayVec3d::new();
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_append() {
        let mut arr = VrmlDataArrayVec3d::new();
        arr.append(1.0, 2.0, 3.0);
        assert_eq!(arr.size(), 1);
        assert_eq!(arr.get(0), Some((1.0, 2.0, 3.0)));
    }
}
