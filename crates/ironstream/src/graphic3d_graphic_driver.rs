// FILE: graphic3d_graphic_driver.rs
// occt: Graphic3d_GraphicDriver

/// Type limit enumeration for graphic resources
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TypeOfLimit {
    MaxNbLights = 0,
    MaxNbClipPlanes = 1,
    MaxNbViews = 2,
}

/// Abstract graphic driver for 3D graphics interface
pub struct GraphicDriver {
    /// Display connection handle
    display_connection: Option<String>,
    /// Structure identifier generator
    struct_gen_id: u32,
}

impl GraphicDriver {
    /// Creates a new graphic driver
    pub fn new(display_connection: Option<String>) -> Self {
        GraphicDriver {
            display_connection,
            struct_gen_id: 0,
        }
    }

    /// Request limit of graphic resource of specific type (abstract, override in subclasses)
    pub fn inquire_limit(&self, _the_type: TypeOfLimit) -> i32 {
        // This would be implemented by subclasses
        0
    }

    /// Request maximum number of active light sources supported
    pub fn inquire_light_limit(&self) -> i32 {
        self.inquire_limit(TypeOfLimit::MaxNbLights)
    }

    /// Request maximum number of active clipping planes supported
    pub fn inquire_plane_limit(&self) -> i32 {
        self.inquire_limit(TypeOfLimit::MaxNbClipPlanes)
    }

    /// Request maximum number of views supported
    pub fn inquire_view_limit(&self) -> i32 {
        self.inquire_limit(TypeOfLimit::MaxNbViews)
    }

    /// Get display connection
    pub fn get_display_connection(&self) -> Option<&str> {
        self.display_connection.as_deref()
    }

    /// Returns a new identification number for a new structure
    pub fn new_identification(&mut self) -> u32 {
        let id = self.struct_gen_id;
        self.struct_gen_id += 1;
        id
    }

    /// Frees the identifier of a structure
    pub fn remove_identification(&mut self, _the_id: u32) {
        // In the real implementation this would track free IDs
    }
}

impl Default for GraphicDriver {
    fn default() -> Self {
        GraphicDriver::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphic_driver_creation() {
        let driver = GraphicDriver::new(Some("display".to_string()));
        assert_eq!(driver.get_display_connection(), Some("display"));
    }

    #[test]
    fn test_graphic_driver_default() {
        let driver = GraphicDriver::default();
        assert_eq!(driver.get_display_connection(), None);
    }

    #[test]
    fn test_new_identification() {
        let mut driver = GraphicDriver::new(None);
        let id1 = driver.new_identification();
        let id2 = driver.new_identification();
        let id3 = driver.new_identification();

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_inquire_limits() {
        let driver = GraphicDriver::new(None);
        assert_eq!(driver.inquire_light_limit(), driver.inquire_limit(TypeOfLimit::MaxNbLights));
        assert_eq!(driver.inquire_plane_limit(), driver.inquire_limit(TypeOfLimit::MaxNbClipPlanes));
        assert_eq!(driver.inquire_view_limit(), driver.inquire_limit(TypeOfLimit::MaxNbViews));
    }

    #[test]
    fn test_type_of_limit_enum() {
        assert_eq!(TypeOfLimit::MaxNbLights as u32, 0);
        assert_eq!(TypeOfLimit::MaxNbClipPlanes as u32, 1);
        assert_eq!(TypeOfLimit::MaxNbViews as u32, 2);
    }
}
