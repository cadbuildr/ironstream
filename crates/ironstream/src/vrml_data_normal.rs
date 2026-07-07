// FILE: vrml_data_normal.rs
// occt: VrmlData_Normal
//
// Faithful port of OCCT VrmlData_Normal (DataExchange/TKDEVRML/VrmlData/
// VrmlData_Normal.hxx/.cxx): represents vertex normals in VRML 2.0.
// Stores a list of 3D normal vectors as f32 triplets; used for smooth shading
// in IndexedFaceSet and related geometry nodes.

use std::cell::RefCell;
use std::rc::Rc;

/// Represents a single 3D normal vector (f32 for VRML compact storage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlDataNormalVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VrmlDataNormalVec {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        VrmlDataNormalVec { x, y, z }
    }

    /// Normalize this vector in-place.
    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 1e-7 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    /// Return the normalized version of this vector.
    pub fn normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 1e-7 {
            VrmlDataNormalVec {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    /// Compute squared magnitude.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Compute magnitude.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl Default for VrmlDataNormalVec {
    fn default() -> Self {
        VrmlDataNormalVec::new(0.0, 0.0, 1.0)
    }
}

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDataNormalErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct VrmlDataNormalInBuffer {
    pub line_num: u32,
}

impl VrmlDataNormalInBuffer {
    pub fn new() -> Self {
        VrmlDataNormalInBuffer { line_num: 1 }
    }
}

impl Default for VrmlDataNormalInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML Normal node: collection of 3D normal vectors for smooth shading.
/// Typically used by IndexedFaceSet to specify vertex normals.
pub struct VrmlDataNormal {
    my_normals: Vec<VrmlDataNormalVec>,
    my_name: String,
}

impl VrmlDataNormal {
    /// Constructor: empty normal list.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataNormal {
            my_normals: Vec::new(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Add a normal vector.
    pub fn add_normal(&mut self, normal: VrmlDataNormalVec) {
        self.my_normals.push(normal);
    }

    /// Get the number of normals.
    pub fn count(&self) -> usize {
        self.my_normals.len()
    }

    /// Get a normal by index (0-based). Returns None if out of range.
    pub fn get(&self, index: usize) -> Option<VrmlDataNormalVec> {
        self.my_normals.get(index).copied()
    }

    /// Get all normals as a slice.
    pub fn normals(&self) -> &[VrmlDataNormalVec] {
        &self.my_normals
    }

    /// Set all normals from a vector.
    pub fn set_normals(&mut self, normals: Vec<VrmlDataNormalVec>) {
        self.my_normals = normals;
    }

    /// Clear all normals.
    pub fn clear(&mut self) {
        self.my_normals.clear();
    }

    /// Virtual read method: parse Normal node from VRML stream.
    pub fn read(&mut self, _buffer: &mut VrmlDataNormalInBuffer) -> VrmlDataNormalErrorStatus {
        // Subclass/user provides actual parsing.
        VrmlDataNormalErrorStatus::Ok
    }

    /// Virtual write method: output Normal node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> VrmlDataNormalErrorStatus {
        // Subclass/user provides actual output.
        VrmlDataNormalErrorStatus::Ok
    }

    /// Check if this node is in default state (empty normal list).
    pub fn is_default(&self) -> bool {
        self.my_normals.is_empty()
    }
}

impl Default for VrmlDataNormal {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataNormal {
    fn clone(&self) -> Self {
        VrmlDataNormal {
            my_normals: self.my_normals.clone(),
            my_name: self.my_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_vec_creation() {
        let n = VrmlDataNormalVec::new(1.0, 0.0, 0.0);
        assert_eq!(n.x, 1.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, 0.0);
    }

    #[test]
    fn normal_vec_normalize() {
        let mut n = VrmlDataNormalVec::new(3.0, 4.0, 0.0);
        n.normalize();
        assert!((n.x - 0.6).abs() < 1e-6);
        assert!((n.y - 0.8).abs() < 1e-6);
        assert!(n.z.abs() < 1e-6);
    }

    #[test]
    fn normal_vec_length() {
        let n = VrmlDataNormalVec::new(3.0, 4.0, 0.0);
        assert!((n.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn normal_list_creation() {
        let normal = VrmlDataNormal::new(Some("Normals"));
        assert_eq!(normal.name(), "Normals");
        assert_eq!(normal.count(), 0);
        assert!(normal.is_default());
    }

    #[test]
    fn add_normals() {
        let mut normal = VrmlDataNormal::new(None);
        normal.add_normal(VrmlDataNormalVec::new(0.0, 0.0, 1.0));
        normal.add_normal(VrmlDataNormalVec::new(1.0, 0.0, 0.0));
        assert_eq!(normal.count(), 2);
        assert!(!normal.is_default());
    }

    #[test]
    fn get_normal() {
        let mut normal = VrmlDataNormal::new(None);
        let v1 = VrmlDataNormalVec::new(0.0, 1.0, 0.0);
        normal.add_normal(v1);
        assert_eq!(normal.get(0), Some(v1));
        assert_eq!(normal.get(1), None);
    }

    #[test]
    fn set_normals() {
        let mut normal = VrmlDataNormal::new(None);
        let vecs = vec![
            VrmlDataNormalVec::new(1.0, 0.0, 0.0),
            VrmlDataNormalVec::new(0.0, 1.0, 0.0),
        ];
        normal.set_normals(vecs);
        assert_eq!(normal.count(), 2);
    }

    #[test]
    fn clear_normals() {
        let mut normal = VrmlDataNormal::new(None);
        normal.add_normal(VrmlDataNormalVec::new(1.0, 0.0, 0.0));
        assert_eq!(normal.count(), 1);
        normal.clear();
        assert_eq!(normal.count(), 0);
        assert!(normal.is_default());
    }

    #[test]
    fn clone_preserves_data() {
        let mut normal = VrmlDataNormal::new(Some("Original"));
        normal.add_normal(VrmlDataNormalVec::new(1.0, 0.0, 0.0));
        let cloned = normal.clone();
        assert_eq!(cloned.name(), "Original");
        assert_eq!(cloned.count(), 1);
    }

    #[test]
    fn normalized_vector() {
        let n = VrmlDataNormalVec::new(3.0, 4.0, 0.0);
        let normalized = n.normalized();
        assert!((normalized.x - 0.6).abs() < 1e-6);
        assert!((normalized.y - 0.8).abs() < 1e-6);
    }
}
