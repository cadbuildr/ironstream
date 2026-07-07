// FILE: vrml_converter_line_aspect.rs
// occt: VrmlConverter_LineAspect

#[derive(Clone, Debug)]
pub struct VrmlConverterLineAspect {
    width: f32,
}

impl VrmlConverterLineAspect {
    pub fn new(width: f32) -> Self {
        VrmlConverterLineAspect { width }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let aspect = VrmlConverterLineAspect::new(1.5);
        assert_eq!(aspect.width(), 1.5);
    }
}
