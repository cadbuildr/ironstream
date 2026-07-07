// FILE: open_gl_background_array.rs
// occt: OpenGl_BackgroundArray

pub struct OpenGlBackgroundArray {
    vertices: Vec<[f32; 3]>,
}

impl OpenGlBackgroundArray {
    pub fn new() -> Self {
        OpenGlBackgroundArray {
            vertices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push([x, y, z]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let arr = OpenGlBackgroundArray::new();
        assert_eq!(arr.vertices.len(), 0);
    }
}
