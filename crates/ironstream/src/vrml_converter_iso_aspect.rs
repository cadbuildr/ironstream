// FILE: vrml_converter_iso_aspect.rs
// occt: VrmlConverter_IsoAspect

#[derive(Clone, Debug)]
pub struct VrmlConverterIsoAspect {
    iso_count: u32,
}

impl VrmlConverterIsoAspect {
    pub fn new(iso_count: u32) -> Self {
        VrmlConverterIsoAspect { iso_count }
    }

    pub fn iso_count(&self) -> u32 {
        self.iso_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let aspect = VrmlConverterIsoAspect::new(10);
        assert_eq!(aspect.iso_count(), 10);
    }
}
