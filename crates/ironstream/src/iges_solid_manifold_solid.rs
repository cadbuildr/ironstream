// FILE: iges_solid_manifold_solid.rs
// occt: IGESSolid_ManifoldSolid

//! Manifold Solid entity (IGES Type 514, Form 1).
//!
//! Represents a closed solid bounded by faces (shell).

#[derive(Clone)]
pub struct Shell {
    id: usize,
}

impl Shell {
    pub fn new(id: usize) -> Self {
        Shell { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Clone)]
pub struct Face {
    id: usize,
}

impl Face {
    pub fn new(id: usize) -> Self {
        Face { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Manifold solid entity
pub struct IGESSolidManifoldSolid {
    shell: Option<Shell>,
    faces: Vec<Face>,
}

impl IGESSolidManifoldSolid {
    /// Creates a new manifold solid
    pub fn new() -> Self {
        IGESSolidManifoldSolid {
            shell: None,
            faces: Vec::new(),
        }
    }

    /// Initializes the manifold solid
    pub fn init(&mut self, shell: Shell, faces: Vec<Face>) {
        self.shell = Some(shell);
        self.faces = faces;
    }

    /// Returns the shell
    pub fn shell(&self) -> Option<&Shell> {
        self.shell.as_ref()
    }

    /// Returns the number of faces
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns the index-th face
    pub fn face(&self, index: usize) -> Option<&Face> {
        self.faces.get(index)
    }

    /// Returns all faces
    pub fn faces(&self) -> &[Face] {
        &self.faces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_creation() {
        let s = Shell::new(1);
        assert_eq!(s.id(), 1);
        assert!(!s.is_null());
    }

    #[test]
    fn test_face_creation() {
        let f = Face::new(2);
        assert_eq!(f.id(), 2);
        assert!(!f.is_null());
    }

    #[test]
    fn test_manifold_solid_creation() {
        let ms = IGESSolidManifoldSolid::new();
        assert!(ms.shell().is_none());
        assert_eq!(ms.nb_faces(), 0);
    }

    #[test]
    fn test_manifold_solid_init() {
        let mut ms = IGESSolidManifoldSolid::new();
        let shell = Shell::new(1);
        let faces = vec![Face::new(2), Face::new(3), Face::new(4)];

        ms.init(shell, faces);

        assert!(ms.shell().is_some());
        assert_eq!(ms.nb_faces(), 3);
    }

    #[test]
    fn test_manifold_solid_face() {
        let mut ms = IGESSolidManifoldSolid::new();
        let faces = vec![Face::new(10), Face::new(20)];

        ms.init(Shell::new(1), faces);

        assert_eq!(ms.face(0).unwrap().id(), 10);
        assert_eq!(ms.face(1).unwrap().id(), 20);
        assert!(ms.face(2).is_none());
    }
}
