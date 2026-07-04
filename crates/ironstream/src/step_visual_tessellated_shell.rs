// FILE: step_visual_tessellated_shell.rs
// occt: StepVisual_TessellatedShell

use std::sync::Arc;

pub struct HasciiString;
pub struct TessellatedFace;

pub struct TessellatedShell {
    name: Option<Arc<HasciiString>>,
    faces: Option<Arc<Vec<Arc<TessellatedFace>>>>,
}

impl TessellatedShell {
    pub fn new() -> Self {
        TessellatedShell {
            name: None,
            faces: None,
        }
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }

    pub fn faces(&self) -> Option<&Arc<Vec<Arc<TessellatedFace>>>> {
        self.faces.as_ref()
    }

    pub fn set_faces(&mut self, faces: Option<Arc<Vec<Arc<TessellatedFace>>>>) {
        self.faces = faces;
    }

    pub fn nb_faces(&self) -> usize {
        self.faces.as_ref().map(|f| f.len()).unwrap_or(0)
    }
}

impl Default for TessellatedShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = TessellatedShell::new();
        assert!(ts.name().is_none());
        assert_eq!(ts.nb_faces(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut ts = TessellatedShell::new();
        let name = Arc::new(HasciiString);
        ts.set_name(Some(name));
        assert!(ts.name().is_some());
    }

    #[test]
    fn test_set_and_get_faces() {
        let mut ts = TessellatedShell::new();
        let faces = vec![Arc::new(TessellatedFace)];
        ts.set_faces(Some(Arc::new(faces)));
        assert_eq!(ts.nb_faces(), 1);
    }
}
