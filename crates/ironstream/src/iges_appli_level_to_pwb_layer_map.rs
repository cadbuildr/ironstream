// FILE: iges_appli_level_to_pwb_layer_map.rs
// occt: IGESAppli_LevelToPWBLayerMap

/// Maps IGES levels to PWB layers.
#[derive(Clone, Debug)]
pub struct IgesAppliLevelToPwbLayerMap {
    level_id: i32,
    layer_id: i32,
}

impl IgesAppliLevelToPwbLayerMap {
    pub fn new() -> Self {
        Self {
            level_id: 0,
            layer_id: 0,
        }
    }

    pub fn init(&mut self, level: i32, layer: i32) {
        self.level_id = level;
        self.layer_id = layer;
    }

    pub fn level_id(&self) -> i32 {
        self.level_id
    }

    pub fn layer_id(&self) -> i32 {
        self.layer_id
    }
}

impl Default for IgesAppliLevelToPwbLayerMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut map = IgesAppliLevelToPwbLayerMap::new();
        map.init(10, 2);
        assert_eq!(map.level_id(), 10);
        assert_eq!(map.layer_id(), 2);
    }
}
