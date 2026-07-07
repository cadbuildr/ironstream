// FILE: mesh_vs_vector_prs_builder.rs
// occt: MeshVS_VectorPrsBuilder

use std::collections::HashMap;

/// 3D Point structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

/// 3D Vector structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    /// Returns the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized unit vector
    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        if mag < 1e-10 {
            Vector::new(0.0, 0.0, 0.0)
        } else {
            Vector {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    /// Computes the cross product with another vector
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes the dot product with another vector
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Scales the vector by a scalar
    pub fn scale(&self, factor: f64) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

/// Builder for vector presentation in mesh visualization.
/// Stores and manages vector data for nodes and elements.
pub struct VectorPrsBuilder {
    /// Maximum length for vector visualization
    max_length: f64,

    /// Color for vectors (R, G, B components)
    vector_color: (u8, u8, u8),

    /// Map of vectors assigned to nodes (node_id -> vector)
    node_vectors: HashMap<i32, Vector>,

    /// Map of vectors assigned to elements (element_id -> vector)
    element_vectors: HashMap<i32, Vector>,

    /// Flag for simple vector arrow presentation
    is_simple_prs: bool,

    /// Line width parameter for simple presentation (default: 2.5)
    simple_width_prm: f64,

    /// Start parameter for simple presentation (default: 0.85)
    simple_start_prm: f64,

    /// End parameter for simple presentation (default: 0.95)
    simple_end_prm: f64,
}

impl VectorPrsBuilder {
    /// Creates a new VectorPrsBuilder with the specified parameters
    pub fn new(
        max_length: f64,
        vector_color: (u8, u8, u8),
        is_simple_prs: bool,
    ) -> Self {
        VectorPrsBuilder {
            max_length,
            vector_color,
            node_vectors: HashMap::new(),
            element_vectors: HashMap::new(),
            is_simple_prs,
            simple_width_prm: 2.5,
            simple_start_prm: 0.85,
            simple_end_prm: 0.95,
        }
    }

    /// Returns a reference to the node vectors map
    pub fn get_node_vectors(&self) -> &HashMap<i32, Vector> {
        &self.node_vectors
    }

    /// Returns a reference to the element vectors map
    pub fn get_element_vectors(&self) -> &HashMap<i32, Vector> {
        &self.element_vectors
    }

    /// Sets the node vectors map
    pub fn set_node_vectors(&mut self, vectors: HashMap<i32, Vector>) {
        self.node_vectors = vectors;
    }

    /// Sets the element vectors map
    pub fn set_element_vectors(&mut self, vectors: HashMap<i32, Vector>) {
        self.element_vectors = vectors;
    }

    /// Returns true if there are any node vectors
    pub fn has_node_vectors(&self) -> bool {
        !self.node_vectors.is_empty()
    }

    /// Returns true if there are any element vectors
    pub fn has_element_vectors(&self) -> bool {
        !self.element_vectors.is_empty()
    }

    /// Gets a vector assigned to a node
    pub fn get_node_vector(&self, id: i32) -> Option<Vector> {
        self.node_vectors.get(&id).copied()
    }

    /// Gets a vector assigned to an element
    pub fn get_element_vector(&self, id: i32) -> Option<Vector> {
        self.element_vectors.get(&id).copied()
    }

    /// Sets a vector for a node
    pub fn set_node_vector(&mut self, id: i32, vector: Vector) {
        self.node_vectors.insert(id, vector);
    }

    /// Sets a vector for an element
    pub fn set_element_vector(&mut self, id: i32, vector: Vector) {
        self.element_vectors.insert(id, vector);
    }

    /// Calculates min and max magnitude values for node vectors
    pub fn get_node_vector_range(&self) -> Option<(f64, f64)> {
        if self.node_vectors.is_empty() {
            return None;
        }

        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for vector in self.node_vectors.values() {
            let mag = vector.magnitude();
            min_val = min_val.min(mag);
            max_val = max_val.max(mag);
        }

        Some((min_val, max_val))
    }

    /// Calculates min and max magnitude values for element vectors
    pub fn get_element_vector_range(&self) -> Option<(f64, f64)> {
        if self.element_vectors.is_empty() {
            return None;
        }

        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for vector in self.element_vectors.values() {
            let mag = vector.magnitude();
            min_val = min_val.min(mag);
            max_val = max_val.max(mag);
        }

        Some((min_val, max_val))
    }

    /// Sets the simple presentation mode
    pub fn set_simple_prs_mode(&mut self, is_simple: bool) {
        self.is_simple_prs = is_simple;
    }

    /// Gets the simple presentation mode
    pub fn is_simple_prs_mode(&self) -> bool {
        self.is_simple_prs
    }

    /// Sets the simple presentation parameters
    pub fn set_simple_prs_params(&mut self, width_param: f64, start_param: f64, end_param: f64) {
        self.simple_width_prm = width_param;
        self.simple_start_prm = start_param;
        self.simple_end_prm = end_param;
    }

    /// Gets the simple presentation parameters
    pub fn get_simple_prs_params(&self) -> (f64, f64, f64) {
        (self.simple_width_prm, self.simple_start_prm, self.simple_end_prm)
    }

    /// Gets the maximum vector length
    pub fn get_max_length(&self) -> f64 {
        self.max_length
    }

    /// Sets the maximum vector length
    pub fn set_max_length(&mut self, max_length: f64) {
        self.max_length = max_length;
    }

    /// Gets the vector color
    pub fn get_vector_color(&self) -> (u8, u8, u8) {
        self.vector_color
    }

    /// Sets the vector color
    pub fn set_vector_color(&mut self, color: (u8, u8, u8)) {
        self.vector_color = color;
    }

    /// Calculates arrow presentation points from a vector length and arrow part ratio.
    /// Returns the calculated arrow head height.
    pub fn calculate_arrow(&self, vector_length: f64, arrow_part: f64) -> f64 {
        let actual_arrow_part = arrow_part.min(1.0).max(0.0);
        let arrow_height = vector_length * actual_arrow_part;

        // Ensure minimum arrow height
        if arrow_height < 0.01 * vector_length {
            0.01 * vector_length
        } else {
            arrow_height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_magnitude() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert!((cross.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 0.0, 0.0);
        assert!((v1.dot(&v2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_scale() {
        let v = Vector::new(2.0, 3.0, 4.0);
        let scaled = v.scale(2.0);
        assert!((scaled.x - 4.0).abs() < 1e-10);
        assert!((scaled.y - 6.0).abs() < 1e-10);
        assert!((scaled.z - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_builder_creation() {
        let builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        assert!(!builder.has_node_vectors());
        assert!(!builder.has_element_vectors());
        assert_eq!(builder.get_max_length(), 100.0);
        assert_eq!(builder.get_vector_color(), (255, 0, 0));
    }

    #[test]
    fn test_node_vector_operations() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        let v = Vector::new(1.0, 2.0, 3.0);

        builder.set_node_vector(1, v);
        assert!(builder.has_node_vectors());
        assert_eq!(builder.get_node_vector(1), Some(v));
        assert_eq!(builder.get_node_vector(2), None);
    }

    #[test]
    fn test_element_vector_operations() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        let v = Vector::new(1.0, 2.0, 3.0);

        builder.set_element_vector(5, v);
        assert!(builder.has_element_vectors());
        assert_eq!(builder.get_element_vector(5), Some(v));
    }

    #[test]
    fn test_node_vector_range() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        builder.set_node_vector(1, Vector::new(3.0, 4.0, 0.0)); // magnitude 5.0
        builder.set_node_vector(2, Vector::new(0.0, 0.0, 10.0)); // magnitude 10.0
        builder.set_node_vector(3, Vector::new(1.0, 0.0, 0.0)); // magnitude 1.0

        let range = builder.get_node_vector_range();
        assert!(range.is_some());
        let (min, max) = range.unwrap();
        assert!((min - 1.0).abs() < 1e-10);
        assert!((max - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_vector_range() {
        let builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        assert!(builder.get_node_vector_range().is_none());
        assert!(builder.get_element_vector_range().is_none());
    }

    #[test]
    fn test_simple_prs_mode() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        assert!(!builder.is_simple_prs_mode());

        builder.set_simple_prs_mode(true);
        assert!(builder.is_simple_prs_mode());
    }

    #[test]
    fn test_simple_prs_params() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        builder.set_simple_prs_params(3.0, 0.8, 0.9);

        let (w, s, e) = builder.get_simple_prs_params();
        assert!((w - 3.0).abs() < 1e-10);
        assert!((s - 0.8).abs() < 1e-10);
        assert!((e - 0.9).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_arrow() {
        let builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        let arrow_height = builder.calculate_arrow(10.0, 0.1);
        assert!((arrow_height - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_arrow_clamping() {
        let builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);

        // Test arrow part > 1.0 gets clamped
        let arrow_height = builder.calculate_arrow(10.0, 1.5);
        assert!(arrow_height <= 10.0);

        // Test arrow part < 0.0 gets clamped
        let arrow_height = builder.calculate_arrow(10.0, -0.5);
        assert!(arrow_height >= 0.0);
    }

    #[test]
    fn test_set_vectors_from_map() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        let mut vectors = HashMap::new();
        vectors.insert(1, Vector::new(1.0, 2.0, 3.0));
        vectors.insert(2, Vector::new(4.0, 5.0, 6.0));

        builder.set_node_vectors(vectors);
        assert!(builder.has_node_vectors());
        assert_eq!(builder.get_node_vectors().len(), 2);
    }

    #[test]
    fn test_get_vectors_map() {
        let mut builder = VectorPrsBuilder::new(100.0, (255, 0, 0), false);
        builder.set_node_vector(1, Vector::new(1.0, 2.0, 3.0));

        let vecs = builder.get_node_vectors();
        assert_eq!(vecs.len(), 1);
        assert_eq!(vecs.get(&1).copied(), Some(Vector::new(1.0, 2.0, 3.0)));
    }
}
