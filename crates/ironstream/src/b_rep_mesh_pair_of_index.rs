// FILE: b_rep_mesh_pair_of_index.rs
// occt: BRepMesh_PairOfIndex

/// Represents a pair of integer indices to store element indices.
/// Restricted to store at most two indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairOfIndex {
    indices: [i32; 2],
}

impl PairOfIndex {
    /// Default constructor.
    pub fn new() -> Self {
        PairOfIndex { indices: [-1, -1] }
    }

    /// Clears indices.
    pub fn clear(&mut self) {
        self.indices[0] = -1;
        self.indices[1] = -1;
    }

    /// Appends index to the pair.
    /// Raises an error if trying to append more than two indices.
    pub fn append(&mut self, index: i32) -> Result<(), String> {
        if self.indices[0] < 0 {
            self.indices[0] = index;
            Ok(())
        } else if self.indices[1] < 0 {
            self.indices[1] = index;
            Ok(())
        } else {
            Err("BRepMesh_PairOfIndex::Append, more than two index to store".to_string())
        }
    }

    /// Prepends index to the pair.
    /// Raises an error if trying to store more than two indices.
    pub fn prepend(&mut self, index: i32) -> Result<(), String> {
        if self.indices[1] >= 0 {
            Err("BRepMesh_PairOfIndex::Prepend, more than two index to store".to_string())
        } else {
            self.indices[1] = self.indices[0];
            self.indices[0] = index;
            Ok(())
        }
    }

    /// Returns true if pair is empty.
    pub fn is_empty(&self) -> bool {
        self.indices[0] < 0
    }

    /// Returns number of initialized indices.
    pub fn extent(&self) -> i32 {
        if self.indices[0] < 0 {
            0
        } else if self.indices[1] < 0 {
            1
        } else {
            2
        }
    }

    /// Returns first index of pair.
    pub fn first_index(&self) -> i32 {
        self.indices[0]
    }

    /// Returns last index of pair.
    pub fn last_index(&self) -> i32 {
        if self.indices[1] < 0 {
            self.indices[0]
        } else {
            self.indices[1]
        }
    }

    /// Returns index corresponding to the given position (1 or 2).
    pub fn index(&self, pair_pos: i32) -> Result<i32, String> {
        match pair_pos {
            1 => Ok(self.indices[0]),
            2 => Ok(self.indices[1]),
            _ => Err("BRepMesh_PairOfIndex::Index, requested index is out of range".to_string()),
        }
    }

    /// Sets index at the given position (1 or 2).
    pub fn set_index(&mut self, pair_pos: i32, index: i32) -> Result<(), String> {
        match pair_pos {
            1 => {
                self.indices[0] = index;
                Ok(())
            }
            2 => {
                self.indices[1] = index;
                Ok(())
            }
            _ => Err("BRepMesh_PairOfIndex::SetIndex, requested index is out of range".to_string()),
        }
    }

    /// Removes index from the given position (1 or 2).
    pub fn remove_index(&mut self, pair_pos: i32) -> Result<(), String> {
        match pair_pos {
            1 => {
                self.indices[0] = self.indices[1];
                self.indices[1] = -1;
                Ok(())
            }
            2 => {
                self.indices[1] = -1;
                Ok(())
            }
            _ => Err(
                "BRepMesh_PairOfIndex::RemoveIndex, requested index is out of range".to_string(),
            ),
        }
    }
}

impl Default for PairOfIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_of_index_new() {
        let p = PairOfIndex::new();
        assert!(p.is_empty());
        assert_eq!(p.extent(), 0);
    }

    #[test]
    fn test_pair_of_index_append() {
        let mut p = PairOfIndex::new();
        assert!(p.append(10).is_ok());
        assert!(!p.is_empty());
        assert_eq!(p.extent(), 1);
        assert_eq!(p.first_index(), 10);

        assert!(p.append(20).is_ok());
        assert_eq!(p.extent(), 2);
        assert_eq!(p.first_index(), 10);
        assert_eq!(p.last_index(), 20);

        // Should fail when appending third index
        assert!(p.append(30).is_err());
    }

    #[test]
    fn test_pair_of_index_prepend() {
        let mut p = PairOfIndex::new();
        assert!(p.prepend(10).is_ok());
        assert_eq!(p.first_index(), 10);

        assert!(p.prepend(5).is_ok());
        assert_eq!(p.first_index(), 5);
        assert_eq!(p.last_index(), 10);

        assert!(p.prepend(1).is_err());
    }

    #[test]
    fn test_pair_of_index_clear() {
        let mut p = PairOfIndex::new();
        let _ = p.append(10);
        let _ = p.append(20);
        assert_eq!(p.extent(), 2);

        p.clear();
        assert!(p.is_empty());
        assert_eq!(p.extent(), 0);
    }

    #[test]
    fn test_pair_of_index_remove() {
        let mut p = PairOfIndex::new();
        let _ = p.append(10);
        let _ = p.append(20);

        assert!(p.remove_index(1).is_ok());
        assert_eq!(p.extent(), 1);
        assert_eq!(p.first_index(), 20);
    }

    #[test]
    fn test_pair_of_index_get_set() {
        let mut p = PairOfIndex::new();
        assert!(p.set_index(1, 100).is_ok());
        assert!(p.set_index(2, 200).is_ok());

        assert_eq!(p.index(1).unwrap(), 100);
        assert_eq!(p.index(2).unwrap(), 200);
        assert!(p.index(3).is_err());
    }
}
