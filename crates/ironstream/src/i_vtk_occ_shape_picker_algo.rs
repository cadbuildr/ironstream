// FILE: i_vtk_occ_shape_picker_algo.rs
// occt: IVtkOCC_ShapePickerAlgo

/// VTK-OCC algorithm for picking shapes in the viewport.
#[derive(Clone, Debug)]
pub struct IVtkOCC_ShapePickerAlgo {
    picked_id: Option<u32>,
}

impl IVtkOCC_ShapePickerAlgo {
    /// Create a new shape picker algorithm.
    pub fn new() -> Self {
        IVtkOCC_ShapePickerAlgo { picked_id: None }
    }

    /// Pick a shape at given screen coordinates.
    pub fn pick(&mut self, x: i32, y: i32) -> Option<u32> {
        // Simulate picking based on coordinates
        if x >= 0 && y >= 0 {
            self.picked_id = Some((x * 1000 + y) as u32);
        } else {
            self.picked_id = None;
        }
        self.picked_id
    }

    /// Get the ID of the last picked shape.
    pub fn picked_id(&self) -> Option<u32> {
        self.picked_id
    }

    /// Clear the picked ID.
    pub fn clear(&mut self) {
        self.picked_id = None;
    }
}

impl Default for IVtkOCC_ShapePickerAlgo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_picker() {
        let picker = IVtkOCC_ShapePickerAlgo::new();
        assert_eq!(picker.picked_id(), None);
    }

    #[test]
    fn test_pick_shape() {
        let mut picker = IVtkOCC_ShapePickerAlgo::new();
        let id = picker.pick(10, 20);
        assert_eq!(id, Some(10020));
        assert_eq!(picker.picked_id(), Some(10020));
    }

    #[test]
    fn test_clear() {
        let mut picker = IVtkOCC_ShapePickerAlgo::new();
        picker.pick(5, 5);
        picker.clear();
        assert_eq!(picker.picked_id(), None);
    }
}
