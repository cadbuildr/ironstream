// FILE: vrml_data_indexed_face_set.rs
// occt: VrmlData_IndexedFaceSet

#[derive(Clone, Debug)]
pub struct VrmlDataIndexedFaceSet {
    face_indices: Vec<Vec<usize>>,
}

impl VrmlDataIndexedFaceSet {
    pub fn new() -> Self {
        VrmlDataIndexedFaceSet {
            face_indices: Vec::new(),
        }
    }

    pub fn add_face(&mut self, indices: Vec<usize>) {
        self.face_indices.push(indices);
    }

    pub fn face_count(&self) -> usize {
        self.face_indices.len()
    }

    pub fn get_face(&self, idx: usize) -> Option<&Vec<usize>> {
        self.face_indices.get(idx)
    }
}

impl Default for VrmlDataIndexedFaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let faces = VrmlDataIndexedFaceSet::new();
        assert_eq!(faces.face_count(), 0);
    }

    #[test]
    fn test_add_face() {
        let mut faces = VrmlDataIndexedFaceSet::new();
        faces.add_face(vec![0, 1, 2]);
        assert_eq!(faces.face_count(), 1);
        assert_eq!(faces.get_face(0), Some(&vec![0, 1, 2]));
    }
}
