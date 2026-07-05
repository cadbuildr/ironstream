// FILE: vrml_converter_point_aspect.rs
// occt: VrmlConverter_PointAspect

#[derive(Clone, Debug)]
pub struct VrmlConverterPointAspect {
    point_size: f32,
}

impl VrmlConverterPointAspect {
    pub fn new(point_size: f32) -> Self {
        VrmlConverterPointAspect { point_size }
    }

    pub fn point_size(&self) -> f32 {
        self.point_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let aspect = VrmlConverterPointAspect::new(2.0);
        assert_eq!(aspect.point_size(), 2.0);
    }
}
