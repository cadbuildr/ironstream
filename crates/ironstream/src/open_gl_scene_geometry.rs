// FILE: open_gl_scene_geometry.rs
// occt: OpenGl_SceneGeometry

/// Scene geometry container.
#[derive(Debug, Clone)]
pub struct OpenGlSceneGeometry;

impl OpenGlSceneGeometry {
    pub fn new() -> Self {
        OpenGlSceneGeometry
    }
}

impl Default for OpenGlSceneGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_geometry() {
        let _geom = OpenGlSceneGeometry::new();
    }
}
