// FILE: std_prs_hlr_shape_i.rs
// occt: StdPrs_HLRShapeI

/// Internal HLR shape interface.
#[derive(Clone, Debug)]
pub struct HlrShapeI {
    pub shape_id: u32,
    pub is_prepared: bool,
}

impl HlrShapeI {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            is_prepared: false,
        }
    }

    pub fn prepare(&mut self) {
        self.is_prepared = true;
    }

    pub fn is_prepared(&self) -> bool {
        self.is_prepared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_interface() {
        let intf = HlrShapeI::new(1);
        assert_eq!(intf.shape_id, 1);
        assert!(!intf.is_prepared());
    }

    #[test]
    fn prepare() {
        let mut intf = HlrShapeI::new(1);
        intf.prepare();
        assert!(intf.is_prepared());
    }
}
