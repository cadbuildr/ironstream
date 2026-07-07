// FILE: iges_draw_network_subfigure_def.rs
// occt: IGESDraw_NetworkSubfigureDef

/// Network subfigure definition entity
pub struct IgesDrawNetworkSubfigureDef {
    entities: Vec<Box<dyn std::any::Any>>,
}

impl IgesDrawNetworkSubfigureDef {
    pub fn new() -> Self {
        IgesDrawNetworkSubfigureDef {
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self, entities: Vec<Box<dyn std::any::Any>>) {
        self.entities = entities;
    }

    pub fn nb_entities(&self) -> i32 {
        self.entities.len() as i32
    }
}

impl Default for IgesDrawNetworkSubfigureDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let nsd = IgesDrawNetworkSubfigureDef::new();
        assert_eq!(nsd.nb_entities(), 0);
    }

    #[test]
    fn test_init() {
        let mut nsd = IgesDrawNetworkSubfigureDef::new();
        nsd.init(vec![]);
        assert_eq!(nsd.nb_entities(), 0);
    }
}
