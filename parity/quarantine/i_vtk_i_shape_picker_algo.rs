// FILE: i_vtk_i_shape_picker_algo.rs
// occt: IVtk_IShapePickerAlgo

/// Interface for shape picking algorithm.
pub trait IVtk_IShapePickerAlgo {
    /// Pick a shape at the given coordinates.
    fn pick(&mut self, x: f64, y: f64) -> Option<u32>;

    /// Get the last picked shape ID.
    fn last_picked_id(&self) -> Option<u32>;

    /// Clear the picked ID.
    fn clear(&mut self);
}

/// Default implementation of IVtk_IShapePickerAlgo.
#[derive(Clone, Debug)]
pub struct DefaultShapePickerAlgo {
    picked_id: Option<u32>,
}

impl DefaultShapePickerAlgo {
    /// Create a new shape picker algorithm.
    pub fn new() -> Self {
        DefaultShapePickerAlgo { picked_id: None }
    }
}

impl Default for DefaultShapePickerAlgo {
    fn default() -> Self {
        Self::new()
    }
}

impl IVtk_IShapePickerAlgo for DefaultShapePickerAlgo {
    fn pick(&mut self, x: f64, y: f64) -> Option<u32> {
        if x >= 0.0 && y >= 0.0 {
            self.picked_id = Some((x as u32) * 1000 + (y as u32));
        } else {
            self.picked_id = None;
        }
        self.picked_id
    }

    fn last_picked_id(&self) -> Option<u32> {
        self.picked_id
    }

    fn clear(&mut self) {
        self.picked_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_picker() {
        let picker = DefaultShapePickerAlgo::new();
        assert_eq!(picker.last_picked_id(), None);
    }

    #[test]
    fn test_pick_shape() {
        let mut picker = DefaultShapePickerAlgo::new();
        let id = picker.pick(5.0, 10.0);
        assert_eq!(id, Some(5010));
    }

    #[test]
    fn test_clear() {
        let mut picker = DefaultShapePickerAlgo::new();
        picker.pick(3.0, 7.0);
        picker.clear();
        assert_eq!(picker.last_picked_id(), None);
    }

    #[test]
    fn test_picker_trait() {
        let mut picker: Box<dyn IVtk_IShapePickerAlgo> = Box::new(DefaultShapePickerAlgo::new());
        picker.pick(2.0, 3.0);
        assert!(picker.last_picked_id().is_some());
    }
}
