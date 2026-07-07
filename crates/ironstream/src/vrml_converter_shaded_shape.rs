// FILE: vrml_converter_shaded_shape.rs
// occt: VrmlConverter_ShadedShape

#[derive(Clone, Debug)]
pub struct VrmlConverterShadedShape {
    is_shaded: bool,
}

impl VrmlConverterShadedShape {
    pub fn new(is_shaded: bool) -> Self {
        VrmlConverterShadedShape { is_shaded }
    }

    pub fn is_shaded(&self) -> bool {
        self.is_shaded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shaded() {
        let shape = VrmlConverterShadedShape::new(true);
        assert!(shape.is_shaded());
    }

    #[test]
    fn test_create_unshaded() {
        let shape = VrmlConverterShadedShape::new(false);
        assert!(!shape.is_shaded());
    }
}
