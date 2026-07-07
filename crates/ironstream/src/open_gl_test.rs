// FILE: open_gl_test.rs
// occt: OpenGlTest

/// OpenGlTest package defines a set of Draw commands for testing of TKOpenGl library.
pub struct OpenGlTest;

impl OpenGlTest {
    /// Adds Draw commands to the draw interpreter.
    pub fn commands() {
        // This is a placeholder for the actual Draw interpreter integration
        // In a real CAD kernel, this would register OpenGL test commands
    }

    /// Plugin entry point function.
    pub fn factory() {
        // This is the factory function that initializes the OpenGL test module
        Self::commands();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opengl_test_factory() {
        OpenGlTest::factory();
    }

    #[test]
    fn test_opengl_test_commands() {
        OpenGlTest::commands();
    }
}
