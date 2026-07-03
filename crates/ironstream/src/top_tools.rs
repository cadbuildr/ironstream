// FILE: top_tools.rs
// occt: TopTools

use std::collections::HashMap;

/// Global tools for topology processing.
pub struct TopTools;

impl TopTools {
    /// Create a topology hash map.
    pub fn create_shape_map() -> HashMap<u32, Vec<u32>> {
        HashMap::new()
    }

    /// Check if a shape is valid.
    pub fn is_valid_shape(shape_id: u32) -> bool {
        shape_id > 0
    }

    /// Get shape type as string.
    pub fn shape_type_str(type_id: u8) -> &'static str {
        match type_id {
            0 => "COMPOUND",
            1 => "COMPSOLID",
            2 => "SOLID",
            3 => "SHELL",
            4 => "FACE",
            5 => "WIRE",
            6 => "EDGE",
            7 => "VERTEX",
            _ => "UNKNOWN",
        }
    }

    /// Get the parent type of a shape.
    pub fn parent_type(type_id: u8) -> Option<u8> {
        match type_id {
            7 => Some(6), // VERTEX -> EDGE
            6 => Some(5), // EDGE -> WIRE
            5 => Some(4), // WIRE -> FACE
            4 => Some(3), // FACE -> SHELL
            3 => Some(2), // SHELL -> SOLID
            2 => Some(1), // SOLID -> COMPSOLID
            1 => Some(0), // COMPSOLID -> COMPOUND
            _ => None,
        }
    }

    /// Check if a type is a sub-type of another.
    pub fn is_subtype(child: u8, parent: u8) -> bool {
        let mut current = child;
        loop {
            if current == parent {
                return true;
            }
            match Self::parent_type(current) {
                Some(p) => current = p,
                None => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shape_map() {
        let map = TopTools::create_shape_map();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_is_valid_shape() {
        assert!(TopTools::is_valid_shape(1));
        assert!(!TopTools::is_valid_shape(0));
    }

    #[test]
    fn test_shape_type_str() {
        assert_eq!(TopTools::shape_type_str(0), "COMPOUND");
        assert_eq!(TopTools::shape_type_str(7), "VERTEX");
        assert_eq!(TopTools::shape_type_str(99), "UNKNOWN");
    }

    #[test]
    fn test_parent_type() {
        assert_eq!(TopTools::parent_type(7), Some(6)); // VERTEX -> EDGE
        assert_eq!(TopTools::parent_type(6), Some(5)); // EDGE -> WIRE
        assert_eq!(TopTools::parent_type(0), None);
    }

    #[test]
    fn test_is_subtype() {
        assert!(TopTools::is_subtype(7, 6)); // VERTEX is subtype of EDGE
        assert!(TopTools::is_subtype(7, 5)); // VERTEX is subtype of WIRE
        assert!(TopTools::is_subtype(7, 7)); // VERTEX is subtype of itself
        assert!(!TopTools::is_subtype(6, 7)); // EDGE is not subtype of VERTEX
    }
}
