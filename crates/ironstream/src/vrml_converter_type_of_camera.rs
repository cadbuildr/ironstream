// FILE: vrml_converter_type_of_camera.rs
// occt: VrmlConverter_TypeOfCamera

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrmlConverterTypeOfCamera {
    Orthographic = 0,
    Perspective = 1,
}

impl VrmlConverterTypeOfCamera {
    pub fn is_orthographic(self) -> bool {
        self == VrmlConverterTypeOfCamera::Orthographic
    }

    pub fn is_perspective(self) -> bool {
        self == VrmlConverterTypeOfCamera::Perspective
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orthographic() {
        let cam = VrmlConverterTypeOfCamera::Orthographic;
        assert!(cam.is_orthographic());
        assert!(!cam.is_perspective());
    }

    #[test]
    fn test_perspective() {
        let cam = VrmlConverterTypeOfCamera::Perspective;
        assert!(!cam.is_orthographic());
        assert!(cam.is_perspective());
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            VrmlConverterTypeOfCamera::Orthographic,
            VrmlConverterTypeOfCamera::Orthographic
        );
        assert_ne!(
            VrmlConverterTypeOfCamera::Orthographic,
            VrmlConverterTypeOfCamera::Perspective
        );
    }
}
