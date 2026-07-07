// FILE: graphic3d_structure_manager.rs
// occt: Graphic3d_StructureManager

/// Manages graphic structures within a viewer, controlling their display,
/// highlighting, and transformations. This is the central manager for all
/// structures visible in the viewport.
#[derive(Debug)]
pub struct Graphic3dStructureManager {
    device_lost_flag: bool,
    num_displayed: usize,
}

impl Graphic3dStructureManager {
    /// Creates a new structure manager. In OCCT, this would initialize the
    /// ViewManager with a reference to the graphics driver.
    pub fn new() -> Self {
        Self {
            device_lost_flag: false,
            num_displayed: 0,
        }
    }

    /// Updates the bounding box of all structures, typically invalidating cached bounds.
    pub fn update(&mut self) {
        // Invalidates bounding boxes for viewport refresh
    }

    /// Erases all structures from the manager.
    pub fn erase(&mut self) {
        self.num_displayed = 0;
    }

    /// Returns whether the device has been lost (e.g., GPU context invalidated).
    pub fn is_device_lost(&self) -> bool {
        self.device_lost_flag
    }

    /// Sets the device lost flag, indicating that presentation data needs re-upload.
    pub fn set_device_lost(&mut self) {
        self.device_lost_flag = true;
    }

    /// Returns the number of structures currently displayed.
    pub fn number_of_displayed_structures(&self) -> usize {
        self.num_displayed
    }

    /// Increments the count of displayed structures.
    pub fn register_displayed_structure(&mut self) {
        self.num_displayed += 1;
    }

    /// Decrements the count of displayed structures.
    pub fn unregister_displayed_structure(&mut self) {
        if self.num_displayed > 0 {
            self.num_displayed -= 1;
        }
    }
}

impl Default for Graphic3dStructureManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let manager = Graphic3dStructureManager::new();
        assert!(!manager.is_device_lost());
        assert_eq!(manager.number_of_displayed_structures(), 0);
    }

    #[test]
    fn test_device_lost_flag() {
        let mut manager = Graphic3dStructureManager::new();
        assert!(!manager.is_device_lost());

        manager.set_device_lost();
        assert!(manager.is_device_lost());
    }

    #[test]
    fn test_displayed_structures_tracking() {
        let mut manager = Graphic3dStructureManager::new();

        manager.register_displayed_structure();
        assert_eq!(manager.number_of_displayed_structures(), 1);

        manager.register_displayed_structure();
        assert_eq!(manager.number_of_displayed_structures(), 2);

        manager.unregister_displayed_structure();
        assert_eq!(manager.number_of_displayed_structures(), 1);
    }

    #[test]
    fn test_erase_clears_structures() {
        let mut manager = Graphic3dStructureManager::new();
        manager.register_displayed_structure();
        manager.register_displayed_structure();

        manager.erase();
        assert_eq!(manager.number_of_displayed_structures(), 0);
    }

    #[test]
    fn test_unregister_does_not_go_negative() {
        let mut manager = Graphic3dStructureManager::new();
        manager.unregister_displayed_structure();
        assert_eq!(manager.number_of_displayed_structures(), 0);
    }

    #[test]
    fn test_default_creation() {
        let manager = Graphic3dStructureManager::default();
        assert!(!manager.is_device_lost());
        assert_eq!(manager.number_of_displayed_structures(), 0);
    }
}
