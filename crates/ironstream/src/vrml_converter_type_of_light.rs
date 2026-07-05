// FILE: vrml_converter_type_of_light.rs
// occt: VrmlConverter_TypeOfLight

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrmlConverterTypeOfLight {
    Ambient = 0,
    Directional = 1,
    Point = 2,
    Spot = 3,
}

impl VrmlConverterTypeOfLight {
    pub fn is_ambient(self) -> bool {
        self == VrmlConverterTypeOfLight::Ambient
    }

    pub fn is_directional(self) -> bool {
        self == VrmlConverterTypeOfLight::Directional
    }

    pub fn is_point(self) -> bool {
        self == VrmlConverterTypeOfLight::Point
    }

    pub fn is_spot(self) -> bool {
        self == VrmlConverterTypeOfLight::Spot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_types() {
        let ambient = VrmlConverterTypeOfLight::Ambient;
        let directional = VrmlConverterTypeOfLight::Directional;
        let point = VrmlConverterTypeOfLight::Point;
        let spot = VrmlConverterTypeOfLight::Spot;

        assert!(ambient.is_ambient());
        assert!(directional.is_directional());
        assert!(point.is_point());
        assert!(spot.is_spot());
    }

    #[test]
    fn test_exclusive() {
        let ambient = VrmlConverterTypeOfLight::Ambient;
        assert!(!ambient.is_directional());
        assert!(!ambient.is_point());
        assert!(!ambient.is_spot());
    }
}
