// FILE: iges_appli_pwb_artwork_stackup.rs
// occt: IGESAppli_PWBArtworkStackup

/// Defines PWB artwork stackup information.
#[derive(Clone, Debug)]
pub struct IgesAppliPwbArtworkStackup {
    stackup_id: i32,
    layer_count: i32,
}

impl IgesAppliPwbArtworkStackup {
    pub fn new() -> Self {
        Self {
            stackup_id: 0,
            layer_count: 0,
        }
    }

    pub fn init(&mut self, id: i32, layers: i32) {
        self.stackup_id = id;
        self.layer_count = layers;
    }

    pub fn stackup_id(&self) -> i32 {
        self.stackup_id
    }

    pub fn layer_count(&self) -> i32 {
        self.layer_count
    }
}

impl Default for IgesAppliPwbArtworkStackup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut stackup = IgesAppliPwbArtworkStackup::new();
        stackup.init(1, 8);

        assert_eq!(stackup.stackup_id(), 1);
        assert_eq!(stackup.layer_count(), 8);
    }
}
