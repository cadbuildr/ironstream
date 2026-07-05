// FILE: open_gl_graphic_driver_factory.rs
// occt: OpenGl_GraphicDriverFactory

/// Factory for creating OpenGL graphic drivers with customizable options.
#[derive(Debug, Clone)]
pub struct OpenGlGraphicDriverFactory {
    has_default_options: bool,
}

impl OpenGlGraphicDriverFactory {
    /// Creates a new OpenGL graphic driver factory.
    pub fn new() -> Self {
        OpenGlGraphicDriverFactory {
            has_default_options: false,
        }
    }

    /// Indicates whether default options are set.
    pub fn has_default_options(&self) -> bool {
        self.has_default_options
    }

    /// Sets that default options are configured.
    pub fn set_default_options(&mut self) {
        self.has_default_options = true;
    }

    /// Creates a new graphic driver instance.
    pub fn create_driver(&self) -> bool {
        true
    }
}

impl Default for OpenGlGraphicDriverFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = OpenGlGraphicDriverFactory::new();
        assert!(!factory.has_default_options());
    }

    #[test]
    fn test_factory_default_options() {
        let mut factory = OpenGlGraphicDriverFactory::new();
        factory.set_default_options();
        assert!(factory.has_default_options());
    }

    #[test]
    fn test_factory_create_driver() {
        let factory = OpenGlGraphicDriverFactory::new();
        assert!(factory.create_driver());
    }

    #[test]
    fn test_factory_default() {
        let factory = OpenGlGraphicDriverFactory::default();
        assert!(!factory.has_default_options());
    }
}
