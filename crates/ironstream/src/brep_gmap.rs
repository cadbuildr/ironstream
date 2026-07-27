// FILE: rust/ironstream/crates/ironstream/src/brep_gmap.rs

use std::collections::HashMap;

// occt: BRepAlgo_Image
/// Maps a set of original shapes to their transformed/generated images.
/// Tracks root shapes and the set of image shapes associated with each.
#[derive(Clone, Debug, Default)]
pub struct ShapeImage {
    pub roots: Vec<String>,
    pub images: HashMap<String, Vec<String>>,
}

impl ShapeImage {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            images: HashMap::new(),
        }
    }

    /// Bind `shape` to the given list of image shapes.
    /// If `shape` is not already a root, it is added to the roots list.
    pub fn bind(&mut self, shape: &str, images: Vec<String>) {
        if !self.images.contains_key(shape) {
            self.roots.push(shape.to_string());
        }
        self.images.insert(shape.to_string(), images);
    }

    /// Returns true if `shape` has at least one image bound to it.
    pub fn has_image(&self, shape: &str) -> bool {
        self.images.contains_key(shape)
    }

    /// Returns the list of images bound to `shape`, or `None` if unbound.
    pub fn image(&self, shape: &str) -> Option<&Vec<String>> {
        self.images.get(shape)
    }

    /// Returns the ordered slice of root shapes.
    pub fn roots(&self) -> &[String] {
        &self.roots
    }
}

// occt-ref: TopTools_DataMapOfShapeShape
/// A one-to-one mapping from a source shape key to a single target shape.
#[derive(Clone, Debug, Default)]
pub struct ShapeMap {
    pub map: HashMap<String, String>,
}

impl ShapeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Bind `from` to `to`, overwriting any existing binding.
    pub fn bind(&mut self, from: &str, to: &str) {
        self.map.insert(from.to_string(), to.to_string());
    }

    /// Return the shape bound to `from`, or `None` if not bound.
    pub fn find(&self, from: &str) -> Option<&str> {
        self.map.get(from).map(String::as_str)
    }

    /// Return true if `from` has a binding in this map.
    pub fn is_bound(&self, from: &str) -> bool {
        self.map.contains_key(from)
    }

    /// Remove the binding for `from` if it exists.
    pub fn unbind(&mut self, from: &str) {
        self.map.remove(from);
    }
}

/// Build a `ShapeMap` where each shape in `shapes` maps to itself.
/// Useful as an identity initialisation before applying transformations.
pub fn build_shape_map(shapes: &[String]) -> ShapeMap {
    let mut m = ShapeMap::new();
    for s in shapes {
        m.bind(s, s);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_image_bind_and_query() {
        let mut img = ShapeImage::new();
        assert!(!img.has_image("face_1"));

        img.bind("face_1", vec!["face_1a".to_string(), "face_1b".to_string()]);
        assert!(img.has_image("face_1"));
        assert_eq!(img.image("face_1").unwrap().len(), 2);
        assert_eq!(img.roots(), &["face_1".to_string()]);
    }

    #[test]
    fn shape_image_rebind_does_not_duplicate_root() {
        let mut img = ShapeImage::new();
        img.bind("e1", vec!["e1_copy".to_string()]);
        img.bind("e1", vec!["e1_new".to_string()]);
        // root should appear only once
        assert_eq!(img.roots().len(), 1);
        assert_eq!(img.image("e1").unwrap(), &["e1_new".to_string()]);
    }

    #[test]
    fn shape_map_bind_find_unbind() {
        let mut sm = ShapeMap::new();
        assert!(!sm.is_bound("s1"));

        sm.bind("s1", "s2");
        assert!(sm.is_bound("s1"));
        assert_eq!(sm.find("s1"), Some("s2"));

        sm.unbind("s1");
        assert!(!sm.is_bound("s1"));
        assert_eq!(sm.find("s1"), None);
    }

    #[test]
    fn build_shape_map_identity() {
        let shapes: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let sm = build_shape_map(&shapes);
        for s in &shapes {
            assert_eq!(sm.find(s), Some(s.as_str()));
        }
    }
}
