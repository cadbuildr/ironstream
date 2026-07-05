// FILE: vrml_converter_drawer.rs
// occt: VrmlConverter_Drawer

#[derive(Clone, Debug)]
pub struct VrmlConverterDrawer {
    is_enabled: bool,
}

impl VrmlConverterDrawer {
    pub fn new() -> Self {
        VrmlConverterDrawer { is_enabled: true }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

impl Default for VrmlConverterDrawer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let drawer = VrmlConverterDrawer::new();
        assert!(drawer.is_enabled());
    }

    #[test]
    fn test_enable() {
        let mut drawer = VrmlConverterDrawer::new();
        drawer.set_enabled(false);
        assert!(!drawer.is_enabled());
    }
}
