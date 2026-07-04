// FILE: step_visual_face_or_surface.rs
// occt: StepVisual_FaceOrSurface

/// A union type selecting either a face or a surface in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum FaceOrSurface {
    Face(i32),
    Surface(i32),
}

impl FaceOrSurface {
    /// Creates a FaceOrSurface from a face.
    pub fn face(id: i32) -> Self {
        FaceOrSurface::Face(id)
    }

    /// Creates a FaceOrSurface from a surface.
    pub fn surface(id: i32) -> Self {
        FaceOrSurface::Surface(id)
    }

    /// Returns the case number (1 = Face, 2 = Surface).
    pub fn case_num(&self) -> i32 {
        match self {
            FaceOrSurface::Face(_) => 1,
            FaceOrSurface::Surface(_) => 2,
        }
    }

    /// Returns the ID if this is a face.
    pub fn as_face(&self) -> Option<i32> {
        match self {
            FaceOrSurface::Face(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the ID if this is a surface.
    pub fn as_surface(&self) -> Option<i32> {
        match self {
            FaceOrSurface::Surface(id) => Some(*id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_or_surface_face() {
        let fos = FaceOrSurface::face(3);
        assert_eq!(fos.case_num(), 1);
        assert_eq!(fos.as_face(), Some(3));
        assert_eq!(fos.as_surface(), None);
    }

    #[test]
    fn test_face_or_surface_surface() {
        let fos = FaceOrSurface::surface(7);
        assert_eq!(fos.case_num(), 2);
        assert_eq!(fos.as_surface(), Some(7));
        assert_eq!(fos.as_face(), None);
    }
}
