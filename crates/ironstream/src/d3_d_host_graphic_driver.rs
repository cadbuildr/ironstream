// FILE: d3_d_host_graphic_driver.rs
// occt: D3DHost_GraphicDriver

//! Defines a D3D host for an OpenGL graphic driver.
//! This driver enables Direct3D interoperability with OpenGL rendering.

use std::sync::Arc;

/// A placeholder for a graphics structure manager.
#[derive(Clone, Debug)]
pub struct Graphic3dStructureManager {
    id: u32,
}

impl Graphic3dStructureManager {
    pub fn new(id: u32) -> Self {
        Graphic3dStructureManager { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// A placeholder for a graphics view.
#[derive(Clone, Debug)]
pub struct Graphic3dCView {
    id: u32,
    manager_id: u32,
    is_d3d_host: bool,
}

impl Graphic3dCView {
    pub fn new(id: u32, manager_id: u32) -> Self {
        Graphic3dCView {
            id,
            manager_id,
            is_d3d_host: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn manager_id(&self) -> u32 {
        self.manager_id
    }

    pub fn is_d3d_host(&self) -> bool {
        self.is_d3d_host
    }

    fn set_d3d_host(&mut self, is_d3d_host: bool) {
        self.is_d3d_host = is_d3d_host;
    }
}

/// A placeholder for an OpenGL graphic driver.
#[derive(Clone, Debug)]
pub struct OpenGlGraphicDriver {
    id: u32,
    view_counter: usize,
}

impl OpenGlGraphicDriver {
    pub fn new() -> Self {
        OpenGlGraphicDriver {
            id: 1,
            view_counter: 0,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn view_counter(&self) -> usize {
        self.view_counter
    }
}

impl Default for OpenGlGraphicDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// D3DHost_GraphicDriver: extends OpenGL driver with D3D host interoperability.
#[derive(Clone, Debug)]
pub struct D3DHostGraphicDriver {
    base_driver: OpenGlGraphicDriver,
}

impl D3DHostGraphicDriver {
    /// Create a new D3D host graphic driver.
    pub fn new() -> Self {
        D3DHostGraphicDriver {
            base_driver: OpenGlGraphicDriver::new(),
        }
    }

    /// Create an instance of a D3D host view.
    /// This returns a view that is bound to the given structure manager
    /// and is configured for D3D interoperability.
    pub fn create_view(
        &mut self,
        manager: &Arc<Graphic3dStructureManager>,
    ) -> Arc<Graphic3dCView> {
        let view_id = (self.base_driver.view_counter + 1) as u32;
        self.base_driver.view_counter += 1;

        let mut view = Graphic3dCView::new(view_id, manager.id());
        view.set_d3d_host(true);

        Arc::new(view)
    }

    /// Get the base OpenGL driver.
    pub fn base_driver(&self) -> &OpenGlGraphicDriver {
        &self.base_driver
    }

    /// Get the number of views created by this driver.
    pub fn view_count(&self) -> usize {
        self.base_driver.view_counter
    }
}

impl Default for D3DHostGraphicDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = D3DHostGraphicDriver::new();
        assert_eq!(driver.view_count(), 0);
    }

    #[test]
    fn test_create_view() {
        let mut driver = D3DHostGraphicDriver::new();
        let manager = Arc::new(Graphic3dStructureManager::new(1));

        let view = driver.create_view(&manager);
        assert_eq!(view.id(), 1);
        assert_eq!(view.manager_id(), 1);
        assert!(view.is_d3d_host());
    }

    #[test]
    fn test_create_multiple_views() {
        let mut driver = D3DHostGraphicDriver::new();
        let manager1 = Arc::new(Graphic3dStructureManager::new(10));
        let manager2 = Arc::new(Graphic3dStructureManager::new(20));

        let view1 = driver.create_view(&manager1);
        let view2 = driver.create_view(&manager2);

        assert_eq!(view1.id(), 1);
        assert_eq!(view2.id(), 2);
        assert_eq!(view1.manager_id(), 10);
        assert_eq!(view2.manager_id(), 20);
        assert_eq!(driver.view_count(), 2);
    }

    #[test]
    fn test_view_is_d3d_host() {
        let mut driver = D3DHostGraphicDriver::new();
        let manager = Arc::new(Graphic3dStructureManager::new(5));

        let view = driver.create_view(&manager);
        assert!(view.is_d3d_host());
    }

    #[test]
    fn test_base_driver_access() {
        let driver = D3DHostGraphicDriver::new();
        let base = driver.base_driver();
        assert_eq!(base.view_counter(), 0);
    }

    #[test]
    fn test_driver_default() {
        let driver = D3DHostGraphicDriver::default();
        assert_eq!(driver.view_count(), 0);
    }

    #[test]
    fn test_structure_manager() {
        let manager = Graphic3dStructureManager::new(42);
        assert_eq!(manager.id(), 42);
    }

    #[test]
    fn test_graphic3d_cview_creation() {
        let view = Graphic3dCView::new(7, 3);
        assert_eq!(view.id(), 7);
        assert_eq!(view.manager_id(), 3);
        assert!(!view.is_d3d_host());
    }
}
