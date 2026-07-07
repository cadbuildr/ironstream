// FILE: iges_appli_region_restriction.rs
// occt: IGESAppli_RegionRestriction

/// Defines region restrictions for PCB design.
#[derive(Clone, Debug)]
pub struct IgesAppliRegionRestriction {
    region_id: i32,
    restricted: bool,
}

impl IgesAppliRegionRestriction {
    pub fn new() -> Self {
        Self {
            region_id: 0,
            restricted: false,
        }
    }

    pub fn init(&mut self, rid: i32, restr: bool) {
        self.region_id = rid;
        self.restricted = restr;
    }

    pub fn region_id(&self) -> i32 {
        self.region_id
    }

    pub fn restricted(&self) -> bool {
        self.restricted
    }
}

impl Default for IgesAppliRegionRestriction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut region = IgesAppliRegionRestriction::new();
        region.init(5, true);

        assert_eq!(region.region_id(), 5);
        assert!(region.restricted());
    }
}
