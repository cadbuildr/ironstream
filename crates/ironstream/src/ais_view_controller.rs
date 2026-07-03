// FILE: ais_view_controller.rs
// occt: AIS_ViewController

#[derive(Clone, Debug)]
pub struct AisViewController {
    pub controller_id: u32,
    pub is_active: bool,
}

impl AisViewController {
    pub fn new(controller_id: u32) -> Self {
        Self { controller_id, is_active: true }
    }
    pub fn set_active(&mut self, v: bool) { self.is_active = v; }
}

impl Default for AisViewController {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_active() {
        let vc = AisViewController::new(1);
        assert!(vc.is_active);
    }
}
