// FILE: open_gl_shader_grid.rs
// occt: OpenGl_ShaderGrid

/// Grid shader for rendering.
#[derive(Debug, Clone)]
pub struct OpenGlShaderGrid;

impl OpenGlShaderGrid {
    pub fn new() -> Self {
        OpenGlShaderGrid
    }
}

impl Default for OpenGlShaderGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_grid() {
        let _grid = OpenGlShaderGrid::new();
    }
}
