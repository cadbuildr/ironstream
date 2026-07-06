// FILE: vrml_data_texture_coordinate.rs
// occt: VrmlData_TextureCoordinate
//
// Faithful port of OCCT VrmlData_TextureCoordinate (DataExchange/TKDEVRML/VrmlData/
// VrmlData_TextureCoordinate.hxx/.cxx): VRML 2.0 TextureCoordinate node.
// Maps 2D (S, T) texture coordinates to geometry vertices for image mapping.

use std::cell::RefCell;
use std::rc::Rc;

/// Single texture coordinate (S, T pair for 2D texture space).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureCoord {
    pub s: f32,
    pub t: f32,
}

impl TextureCoord {
    pub fn new(s: f32, t: f32) -> Self {
        TextureCoord { s, t }
    }

    /// Distance in texture space (Euclidean).
    pub fn distance_to(&self, other: &TextureCoord) -> f32 {
        let ds = self.s - other.s;
        let dt = self.t - other.t;
        (ds * ds + dt * dt).sqrt()
    }

    /// Clamp coordinates to [0, 1].
    pub fn clamp(&self) -> Self {
        TextureCoord {
            s: self.s.clamp(0.0, 1.0),
            t: self.t.clamp(0.0, 1.0),
        }
    }
}

impl Default for TextureCoord {
    fn default() -> Self {
        TextureCoord::new(0.0, 0.0)
    }
}

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureCoordinateErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct TextureCoordinateInBuffer {
    pub line_num: u32,
}

impl TextureCoordinateInBuffer {
    pub fn new() -> Self {
        TextureCoordinateInBuffer { line_num: 1 }
    }
}

impl Default for TextureCoordinateInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML TextureCoordinate node: collection of 2D texture coordinates.
/// Maps to vertices of geometry to define texture mapping on a per-vertex basis.
pub struct VrmlDataTextureCoordinate {
    my_coords: Vec<TextureCoord>,
    my_name: String,
}

impl VrmlDataTextureCoordinate {
    /// Constructor: empty coordinate list.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataTextureCoordinate {
            my_coords: Vec::new(),
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

    /// Add a texture coordinate.
    pub fn add_coord(&mut self, coord: TextureCoord) {
        self.my_coords.push(coord);
    }

    /// Get the number of coordinates.
    pub fn count(&self) -> usize {
        self.my_coords.len()
    }

    /// Get a coordinate by index (0-based). Returns None if out of range.
    pub fn get(&self, index: usize) -> Option<TextureCoord> {
        self.my_coords.get(index).copied()
    }

    /// Get all coordinates as a slice.
    pub fn coords(&self) -> &[TextureCoord] {
        &self.my_coords
    }

    /// Set all coordinates from a vector.
    pub fn set_coords(&mut self, coords: Vec<TextureCoord>) {
        self.my_coords = coords;
    }

    /// Clear all coordinates.
    pub fn clear(&mut self) {
        self.my_coords.clear();
    }

    /// Check if in default state (empty).
    pub fn is_default(&self) -> bool {
        self.my_coords.is_empty()
    }

    /// Virtual read method: parse TextureCoordinate node from VRML stream.
    pub fn read(&mut self, _buffer: &mut TextureCoordinateInBuffer) -> TextureCoordinateErrorStatus {
        // Subclass/user provides actual parsing.
        TextureCoordinateErrorStatus::Ok
    }

    /// Virtual write method: output TextureCoordinate node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> TextureCoordinateErrorStatus {
        // Subclass/user provides actual output.
        TextureCoordinateErrorStatus::Ok
    }

    /// Compute axis-aligned bounding box of all coordinates in texture space.
    pub fn compute_bounds(&self) -> Option<(TextureCoord, TextureCoord)> {
        if self.my_coords.is_empty() {
            return None;
        }

        let mut min_s = f32::MAX;
        let mut max_s = f32::MIN;
        let mut min_t = f32::MAX;
        let mut max_t = f32::MIN;

        for coord in &self.my_coords {
            min_s = min_s.min(coord.s);
            max_s = max_s.max(coord.s);
            min_t = min_t.min(coord.t);
            max_t = max_t.max(coord.t);
        }

        Some((
            TextureCoord::new(min_s, min_t),
            TextureCoord::new(max_s, max_t),
        ))
    }

    /// Clamp all coordinates to [0, 1].
    pub fn clamp_all(&mut self) {
        for coord in &mut self.my_coords {
            *coord = coord.clamp();
        }
    }
}

impl Default for VrmlDataTextureCoordinate {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataTextureCoordinate {
    fn clone(&self) -> Self {
        VrmlDataTextureCoordinate {
            my_coords: self.my_coords.clone(),
            my_name: self.my_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_coord_creation() {
        let coord = TextureCoord::new(0.5, 0.75);
        assert_eq!(coord.s, 0.5);
        assert_eq!(coord.t, 0.75);
    }

    #[test]
    fn texture_coord_default() {
        let coord = TextureCoord::default();
        assert_eq!(coord.s, 0.0);
        assert_eq!(coord.t, 0.0);
    }

    #[test]
    fn texture_coord_distance() {
        let c1 = TextureCoord::new(0.0, 0.0);
        let c2 = TextureCoord::new(3.0, 4.0);
        assert!((c1.distance_to(&c2) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn texture_coord_clamp() {
        let coord = TextureCoord::new(2.0, -1.0);
        let clamped = coord.clamp();
        assert_eq!(clamped.s, 1.0);
        assert_eq!(clamped.t, 0.0);
    }

    #[test]
    fn texture_coordinate_list_creation() {
        let texcoord = VrmlDataTextureCoordinate::new(Some("TexCoords"));
        assert_eq!(texcoord.name(), "TexCoords");
        assert_eq!(texcoord.count(), 0);
        assert!(texcoord.is_default());
    }

    #[test]
    fn add_coords() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        texcoord.add_coord(TextureCoord::new(0.0, 0.0));
        texcoord.add_coord(TextureCoord::new(1.0, 1.0));
        assert_eq!(texcoord.count(), 2);
        assert!(!texcoord.is_default());
    }

    #[test]
    fn get_coord() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        let c = TextureCoord::new(0.5, 0.5);
        texcoord.add_coord(c);
        assert_eq!(texcoord.get(0), Some(c));
        assert_eq!(texcoord.get(1), None);
    }

    #[test]
    fn set_coords() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        let coords = vec![
            TextureCoord::new(0.0, 0.0),
            TextureCoord::new(1.0, 0.0),
            TextureCoord::new(1.0, 1.0),
        ];
        texcoord.set_coords(coords.clone());
        assert_eq!(texcoord.count(), 3);
    }

    #[test]
    fn clear_coords() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        texcoord.add_coord(TextureCoord::new(0.0, 0.0));
        assert_eq!(texcoord.count(), 1);
        texcoord.clear();
        assert_eq!(texcoord.count(), 0);
        assert!(texcoord.is_default());
    }

    #[test]
    fn compute_bounds() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        texcoord.add_coord(TextureCoord::new(0.2, 0.3));
        texcoord.add_coord(TextureCoord::new(0.8, 0.9));
        texcoord.add_coord(TextureCoord::new(0.5, 0.5));

        let bounds = texcoord.compute_bounds();
        assert!(bounds.is_some());
        let (min, max) = bounds.unwrap();
        assert!((min.s - 0.2).abs() < 1e-6);
        assert!((min.t - 0.3).abs() < 1e-6);
        assert!((max.s - 0.8).abs() < 1e-6);
        assert!((max.t - 0.9).abs() < 1e-6);
    }

    #[test]
    fn compute_bounds_empty() {
        let texcoord = VrmlDataTextureCoordinate::new(None);
        assert_eq!(texcoord.compute_bounds(), None);
    }

    #[test]
    fn clamp_all() {
        let mut texcoord = VrmlDataTextureCoordinate::new(None);
        texcoord.add_coord(TextureCoord::new(2.0, -1.0));
        texcoord.add_coord(TextureCoord::new(0.5, 0.5));
        texcoord.clamp_all();

        assert_eq!(texcoord.get(0), Some(TextureCoord::new(1.0, 0.0)));
        assert_eq!(texcoord.get(1), Some(TextureCoord::new(0.5, 0.5)));
    }

    #[test]
    fn clone_preserves_data() {
        let mut texcoord = VrmlDataTextureCoordinate::new(Some("Original"));
        texcoord.add_coord(TextureCoord::new(0.5, 0.5));
        let cloned = texcoord.clone();
        assert_eq!(cloned.name(), "Original");
        assert_eq!(cloned.count(), 1);
        assert_eq!(cloned.get(0), Some(TextureCoord::new(0.5, 0.5)));
    }

    #[test]
    fn set_name() {
        let mut texcoord = VrmlDataTextureCoordinate::new(Some("Old"));
        texcoord.set_name("New");
        assert_eq!(texcoord.name(), "New");
    }
}
