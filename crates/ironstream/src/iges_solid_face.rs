// FILE: iges_solid_face.rs
// occt: IGESSolid_Face

//! Face entity for boundary representation (IGES Type 510, Form 1).
//!
//! Defines a face with surface reference and loop list.

#[derive(Clone)]
pub struct Surface {
    id: usize,
}

impl Surface {
    pub fn new(id: usize) -> Self {
        Surface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Clone)]
pub struct Loop {
    id: usize,
}

impl Loop {
    pub fn new(id: usize) -> Self {
        Loop { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Face entity
pub struct IGESSolidFace {
    surface: Option<Surface>,
    outer_loop: Option<Loop>,
    inner_loops: Vec<Loop>,
}

impl IGESSolidFace {
    pub fn new() -> Self {
        IGESSolidFace {
            surface: None,
            outer_loop: None,
            inner_loops: Vec::new(),
        }
    }

    pub fn init(&mut self, surface: Surface, outer_loop: Loop, inner_loops: Vec<Loop>) {
        self.surface = Some(surface);
        self.outer_loop = Some(outer_loop);
        self.inner_loops = inner_loops;
    }

    pub fn surface(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    pub fn outer_loop(&self) -> Option<&Loop> {
        self.outer_loop.as_ref()
    }

    pub fn nb_inner_loops(&self) -> usize {
        self.inner_loops.len()
    }

    pub fn inner_loop(&self, index: usize) -> Option<&Loop> {
        self.inner_loops.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_creation() {
        let s = Surface::new(1);
        assert_eq!(s.id(), 1);
        assert!(!s.is_null());
    }

    #[test]
    fn test_loop_creation() {
        let l = Loop::new(2);
        assert_eq!(l.id(), 2);
        assert!(!l.is_null());
    }

    #[test]
    fn test_face_creation() {
        let f = IGESSolidFace::new();
        assert!(f.surface().is_none());
        assert!(f.outer_loop().is_none());
    }

    #[test]
    fn test_face_init() {
        let mut f = IGESSolidFace::new();
        let surf = Surface::new(1);
        let outer = Loop::new(2);
        let inner = vec![Loop::new(3)];

        f.init(surf, outer, inner);

        assert!(f.surface().is_some());
        assert!(f.outer_loop().is_some());
        assert_eq!(f.nb_inner_loops(), 1);
    }
}
