// FILE: d3_d_host_test.rs
// occt: D3DHostTest

//! This module provides Draw commands for testing the D3D Host library.
//! It registers test commands with the Draw interpreter.

/// A simple placeholder type to represent the Draw interpreter.
/// In real OCCT, Draw_Interpretor is a complex command-processing system.
#[derive(Clone, Debug)]
pub struct DrawInterpretor {
    name: String,
}

impl DrawInterpretor {
    /// Create a new Draw interpreter instance.
    pub fn new(name: &str) -> Self {
        DrawInterpretor {
            name: name.to_string(),
        }
    }

    /// Get the name of the interpreter.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// D3DHostTest: defines Draw commands for testing the D3D Host library.
pub struct D3DHostTest;

impl D3DHostTest {
    /// Adds Draw commands to the draw interpreter.
    /// In a real implementation, this would register various test commands
    /// for D3D Host visualization functionality.
    pub fn commands(interpreter: &mut DrawInterpretor) {
        // In production, this would register commands like:
        // - d3d_test_frame: test framebuffer operations
        // - d3d_test_render: test rendering pipeline
        // - d3d_test_view: test view manipulation
        // etc.
        let _ = interpreter;
    }

    /// Plugin entry point function.
    /// This is called when the D3DHostTest plugin is loaded.
    /// It initializes the module and registers all available commands.
    pub fn factory(interpreter: &mut DrawInterpretor) {
        Self::commands(interpreter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_interpretor_creation() {
        let interp = DrawInterpretor::new("test_interpreter");
        assert_eq!(interp.name(), "test_interpreter");
    }

    #[test]
    fn test_d3d_host_test_commands() {
        let mut interp = DrawInterpretor::new("d3d_test");
        D3DHostTest::commands(&mut interp);
        assert_eq!(interp.name(), "d3d_test");
    }

    #[test]
    fn test_d3d_host_test_factory() {
        let mut interp = DrawInterpretor::new("factory_test");
        D3DHostTest::factory(&mut interp);
        assert_eq!(interp.name(), "factory_test");
    }

    #[test]
    fn test_multiple_commands_registration() {
        let mut interp1 = DrawInterpretor::new("interp1");
        let mut interp2 = DrawInterpretor::new("interp2");

        D3DHostTest::commands(&mut interp1);
        D3DHostTest::commands(&mut interp2);

        assert_eq!(interp1.name(), "interp1");
        assert_eq!(interp2.name(), "interp2");
    }

    #[test]
    fn test_factory_creates_valid_state() {
        let mut interp = DrawInterpretor::new("valid_state");
        D3DHostTest::factory(&mut interp);

        // After factory call, the interpreter should be in a valid state
        assert!(!interp.name().is_empty());
    }
}
