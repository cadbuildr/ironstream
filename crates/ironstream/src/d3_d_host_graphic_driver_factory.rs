// FILE: d3_d_host_graphic_driver_factory.rs
// occt: D3DHost_GraphicDriverFactory

//! Factory for creating D3D Host graphic drivers.
//! This factory extends OpenGL graphic driver factory with D3D interoperability support.

use std::sync::Arc;

/// Placeholder for display connection.
#[derive(Clone, Debug)]
pub struct AspectDisplayConnection {
    id: u32,
}

impl AspectDisplayConnection {
    pub fn new(id: u32) -> Self {
        AspectDisplayConnection { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Placeholder for a generic graphic driver.
#[derive(Clone, Debug)]
pub enum Graphic3dGraphicDriver {
    OpenGl(Arc<OpenGlGraphicDriver>),
    D3DHost(Arc<D3DHostGraphicDriver>),
}

/// Placeholder for OpenGL graphic driver factory.
#[derive(Clone, Debug)]
pub struct OpenGlGraphicDriverFactory;

impl OpenGlGraphicDriverFactory {
    pub fn new() -> Self {
        OpenGlGraphicDriverFactory
    }
}

impl Default for OpenGlGraphicDriverFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder for OpenGL graphic driver.
#[derive(Clone, Debug)]
pub struct OpenGlGraphicDriver {
    id: u32,
}

impl OpenGlGraphicDriver {
    pub fn new() -> Self {
        OpenGlGraphicDriver { id: 1 }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Default for OpenGlGraphicDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder for D3D Host graphic driver.
#[derive(Clone, Debug)]
pub struct D3DHostGraphicDriver {
    id: u32,
    is_initialized: bool,
}

impl D3DHostGraphicDriver {
    pub fn new() -> Self {
        D3DHostGraphicDriver {
            id: 2,
            is_initialized: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn initialize(&mut self) {
        self.is_initialized = true;
    }
}

impl Default for D3DHostGraphicDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// D3DHost_GraphicDriverFactory: creates D3D host graphic drivers.
/// Extends OpenGL driver factory with D3D interoperability.
#[derive(Clone, Debug)]
pub struct D3DHostGraphicDriverFactory {
    base_factory: OpenGlGraphicDriverFactory,
    driver_counter: u32,
}

impl D3DHostGraphicDriverFactory {
    /// Create a new D3D Host graphic driver factory.
    pub fn new() -> Self {
        D3DHostGraphicDriverFactory {
            base_factory: OpenGlGraphicDriverFactory::new(),
            driver_counter: 0,
        }
    }

    /// Create a new empty graphic driver.
    /// This factory creates D3D Host drivers configured for D3D/OpenGL interoperability.
    pub fn create_driver(
        &mut self,
        _disp: &Arc<AspectDisplayConnection>,
    ) -> Arc<Graphic3dGraphicDriver> {
        self.driver_counter += 1;

        let mut driver = D3DHostGraphicDriver::new();
        driver.initialize();

        Arc::new(Graphic3dGraphicDriver::D3DHost(Arc::new(driver)))
    }

    /// Get the number of drivers created by this factory.
    pub fn driver_count(&self) -> u32 {
        self.driver_counter
    }

    /// Reset the factory to initial state.
    pub fn reset(&mut self) {
        self.driver_counter = 0;
    }
}

impl Default for D3DHostGraphicDriverFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = D3DHostGraphicDriverFactory::new();
        assert_eq!(factory.driver_count(), 0);
    }

    #[test]
    fn test_create_driver() {
        let mut factory = D3DHostGraphicDriverFactory::new();
        let disp = Arc::new(AspectDisplayConnection::new(1));

        let driver = factory.create_driver(&disp);
        assert_eq!(factory.driver_count(), 1);

        match driver.as_ref() {
            Graphic3dGraphicDriver::D3DHost(d3d) => {
                assert!(d3d.is_initialized());
            }
            _ => panic!("Expected D3DHost driver"),
        }
    }

    #[test]
    fn test_create_multiple_drivers() {
        let mut factory = D3DHostGraphicDriverFactory::new();
        let disp = Arc::new(AspectDisplayConnection::new(2));

        let driver1 = factory.create_driver(&disp);
        let driver2 = factory.create_driver(&disp);
        let driver3 = factory.create_driver(&disp);

        assert_eq!(factory.driver_count(), 3);

        // Verify all drivers are D3DHost type
        match driver1.as_ref() {
            Graphic3dGraphicDriver::D3DHost(_) => (),
            _ => panic!("Expected D3DHost driver"),
        }
        match driver2.as_ref() {
            Graphic3dGraphicDriver::D3DHost(_) => (),
            _ => panic!("Expected D3DHost driver"),
        }
        match driver3.as_ref() {
            Graphic3dGraphicDriver::D3DHost(_) => (),
            _ => panic!("Expected D3DHost driver"),
        }
    }

    #[test]
    fn test_factory_reset() {
        let mut factory = D3DHostGraphicDriverFactory::new();
        let disp = Arc::new(AspectDisplayConnection::new(3));

        factory.create_driver(&disp);
        factory.create_driver(&disp);
        assert_eq!(factory.driver_count(), 2);

        factory.reset();
        assert_eq!(factory.driver_count(), 0);
    }

    #[test]
    fn test_display_connection() {
        let disp = AspectDisplayConnection::new(99);
        assert_eq!(disp.id(), 99);
    }

    #[test]
    fn test_opengl_graphic_driver() {
        let driver = OpenGlGraphicDriver::new();
        assert_eq!(driver.id(), 1);
    }

    #[test]
    fn test_d3d_host_graphic_driver() {
        let mut driver = D3DHostGraphicDriver::new();
        assert_eq!(driver.id(), 2);
        assert!(!driver.is_initialized());

        driver.initialize();
        assert!(driver.is_initialized());
    }

    #[test]
    fn test_factory_default() {
        let factory = D3DHostGraphicDriverFactory::default();
        assert_eq!(factory.driver_count(), 0);
    }
}
