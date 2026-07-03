// FILE: std_select_type_of_face.rs
// occt: StdSelect_TypeOfFace

//! Provides values for different types of faces.
//! These values are used to filter faces in frameworks inheriting StdSelect_FaceFilter.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeOfFace {
    #[default]
    AnyFace,
    Plane,
    Cylinder,
    Sphere,
    Torus,
    Revol,
    Cone,
}

impl TypeOfFace {
    pub fn as_u32(&self) -> u32 {
        match self {
            TypeOfFace::AnyFace => 0,
            TypeOfFace::Plane => 1,
            TypeOfFace::Cylinder => 2,
            TypeOfFace::Sphere => 3,
            TypeOfFace::Torus => 4,
            TypeOfFace::Revol => 5,
            TypeOfFace::Cone => 6,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(TypeOfFace::AnyFace),
            1 => Some(TypeOfFace::Plane),
            2 => Some(TypeOfFace::Cylinder),
            3 => Some(TypeOfFace::Sphere),
            4 => Some(TypeOfFace::Torus),
            5 => Some(TypeOfFace::Revol),
            6 => Some(TypeOfFace::Cone),
            _ => None,
        }
    }

    pub fn is_any(&self) -> bool {
        *self == TypeOfFace::AnyFace
    }

    pub fn is_quadric(&self) -> bool {
        matches!(self, TypeOfFace::Plane | TypeOfFace::Cylinder | TypeOfFace::Sphere | TypeOfFace::Cone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_face_conversions() {
        assert_eq!(TypeOfFace::AnyFace.as_u32(), 0);
        assert_eq!(TypeOfFace::Plane.as_u32(), 1);
        assert_eq!(TypeOfFace::Cylinder.as_u32(), 2);
        assert_eq!(TypeOfFace::Sphere.as_u32(), 3);
        assert_eq!(TypeOfFace::Torus.as_u32(), 4);
        assert_eq!(TypeOfFace::Revol.as_u32(), 5);
        assert_eq!(TypeOfFace::Cone.as_u32(), 6);
    }

    #[test]
    fn type_of_face_from_u32() {
        assert_eq!(TypeOfFace::from_u32(0), Some(TypeOfFace::AnyFace));
        assert_eq!(TypeOfFace::from_u32(3), Some(TypeOfFace::Sphere));
        assert_eq!(TypeOfFace::from_u32(6), Some(TypeOfFace::Cone));
        assert_eq!(TypeOfFace::from_u32(99), None);
    }

    #[test]
    fn type_of_face_predicates() {
        assert!(TypeOfFace::AnyFace.is_any());
        assert!(TypeOfFace::Plane.is_quadric());
        assert!(TypeOfFace::Sphere.is_quadric());
        assert!(!TypeOfFace::Torus.is_quadric());
    }
}
