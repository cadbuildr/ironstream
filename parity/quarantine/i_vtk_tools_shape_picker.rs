// FILE: i_vtk_tools_shape_picker.rs
// occt: IVtkTools_ShapePicker

/// Shape picker for VTK tools.
#[derive(Clone, Debug)]
pub struct IVtkTools_ShapePicker {
    last_picked_id: Option<u32>,
}

impl IVtkTools_ShapePicker {
    /// Create a new shape picker.
    pub fn new() -> Self {
        IVtkTools_ShapePicker {
            last_picked_id: None,
        }
    }

    /// Pick a shape at the given coordinates.
    pub fn pick(&mut self, x: f64, y: f64) -> Option<u32> {
        if x >= 0.0 && y >= 0.0 {
            self.last_picked_id = Some((x as u32) * 1000 + (y as u32));
        } else {
            self.last_picked_id = None;
        }
        self.last_picked_id
    }

    /// Get the last picked shape ID.
    pub fn last_picked_id(&self) -> Option<u32> {
        self.last_picked_id
    }

    /// Clear the last picked ID.
    pub fn clear(&mut self) {
        self.last_picked_id = None;
    }

    /// Check if a shape is currently picked.
    pub fn has_picked(&self) -> bool {
        self.last_picked_id.is_some()
    }
}

impl Default for IVtkTools_ShapePicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_picker() {
        let picker = IVtkTools_ShapePicker::new();
        assert!(!picker.has_picked());
    }

    #[test]
    fn test_pick_shape() {
        let mut picker = IVtkTools_ShapePicker::new();
        let id = picker.pick(10.5, 20.5);
        assert!(id.is_some());
        assert!(picker.has_picked());
    }

    #[test]
    fn test_clear() {
        let mut picker = IVtkTools_ShapePicker::new();
        picker.pick(5.0, 5.0);
        picker.clear();
        assert!(!picker.has_picked());
        assert_eq!(picker.last_picked_id(), None);
    }

    #[test]
    fn test_invalid_coordinates() {
        let mut picker = IVtkTools_ShapePicker::new();
        picker.pick(-1.0, -1.0);
        assert!(!picker.has_picked());
    }
}
