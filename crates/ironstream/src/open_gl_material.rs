// FILE: open_gl_material.rs
// occt: OpenGl_Material

/// OpenGL material properties (ambient, diffuse, specular, etc).
#[derive(Debug, Clone)]
pub struct OpenGlMaterial {
    ambient: [f32; 4],
    diffuse: [f32; 4],
    specular: [f32; 4],
    shininess: f32,
}

impl OpenGlMaterial {
    /// Creates default material.
    pub fn new() -> Self {
        OpenGlMaterial {
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [0.8, 0.8, 0.8, 1.0],
            specular: [1.0, 1.0, 1.0, 1.0],
            shininess: 128.0,
        }
    }

    /// Sets ambient color.
    pub fn set_ambient(&mut self, color: [f32; 4]) {
        self.ambient = color;
    }

    /// Sets diffuse color.
    pub fn set_diffuse(&mut self, color: [f32; 4]) {
        self.diffuse = color;
    }

    /// Sets specular color.
    pub fn set_specular(&mut self, color: [f32; 4]) {
        self.specular = color;
    }

    /// Sets shininess.
    pub fn set_shininess(&mut self, s: f32) {
        self.shininess = s;
    }
}

impl Default for OpenGlMaterial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_defaults() {
        let mat = OpenGlMaterial::new();
        assert_eq!(mat.ambient[0], 0.2);
        assert_eq!(mat.diffuse[0], 0.8);
    }
}
