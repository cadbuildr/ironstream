// FILE: graphic3d_graphic_driver_factory.rs
// occt: Graphic3d_GraphicDriverFactory

use core::fmt;

/// Factory for creating graphic drivers
pub struct GraphicDriverFactory {
    name: String,
}

impl GraphicDriverFactory {
    /// Creates a new graphic driver factory
    pub fn new(name: &str) -> Self {
        GraphicDriverFactory {
            name: name.to_string(),
        }
    }

    /// Return driver factory name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Debug for GraphicDriverFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphicDriverFactory")
            .field("name", &self.name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_factory_creation() {
        let factory = GraphicDriverFactory::new("OpenGL");
        assert_eq!(factory.name(), "OpenGL");
    }

    #[test]
    fn test_driver_factory_name_persistence() {
        let factory = GraphicDriverFactory::new("TestFactory");
        assert_eq!(factory.name(), "TestFactory");

        let factory2 = GraphicDriverFactory::new("AnotherFactory");
        assert_eq!(factory2.name(), "AnotherFactory");

        // Original factory name should not change
        assert_eq!(factory.name(), "TestFactory");
    }

    #[test]
    fn test_driver_factory_debug() {
        let factory = GraphicDriverFactory::new("DebugFactory");
        let debug_str = format!("{:?}", factory);
        assert!(debug_str.contains("GraphicDriverFactory"));
        assert!(debug_str.contains("DebugFactory"));
    }

    #[test]
    fn test_driver_factory_empty_name() {
        let factory = GraphicDriverFactory::new("");
        assert_eq!(factory.name(), "");
    }
}
