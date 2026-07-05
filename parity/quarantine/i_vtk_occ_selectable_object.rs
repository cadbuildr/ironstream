// FILE: i_vtk_occ_selectable_object.rs
// occt: IVtkOCC_SelectableObject

/// VTK-OCC bridge for selectable objects in visualization.
#[derive(Clone, Debug)]
pub struct IVtkOCC_SelectableObject {
    id: u32,
    is_selected: bool,
}

impl IVtkOCC_SelectableObject {
    /// Create a new selectable object.
    pub fn new(id: u32) -> Self {
        IVtkOCC_SelectableObject {
            id,
            is_selected: false,
        }
    }

    /// Get object ID.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Select this object.
    pub fn select(&mut self) {
        self.is_selected = true;
    }

    /// Deselect this object.
    pub fn deselect(&mut self) {
        self.is_selected = false;
    }

    /// Check if object is selected.
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_object() {
        let obj = IVtkOCC_SelectableObject::new(42);
        assert_eq!(obj.id(), 42);
        assert!(!obj.is_selected());
    }

    #[test]
    fn test_select() {
        let mut obj = IVtkOCC_SelectableObject::new(1);
        obj.select();
        assert!(obj.is_selected());
    }

    #[test]
    fn test_deselect() {
        let mut obj = IVtkOCC_SelectableObject::new(2);
        obj.select();
        obj.deselect();
        assert!(!obj.is_selected());
    }
}
