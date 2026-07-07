// FILE: aspect_touch_map.rs
// occt: Aspect_TouchMap

//! Deprecated NCollection alias: DataMap<TouchID, TouchData>

use std::collections::HashMap;

/// Touch data (stub).
#[derive(Clone, Debug)]
pub struct TouchData {
    pub id: u32,
    pub x: f64,
    pub y: f64,
}

/// Map of touch points.
pub type AspectTouchMap = HashMap<u32, TouchData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_map() {
        let mut map: AspectTouchMap = HashMap::new();
        map.insert(1, TouchData { id: 1, x: 100.0, y: 200.0 });
        assert_eq!(map.get(&1).map(|t| t.x), Some(100.0));
    }
}
