// FILE: iges_draw_network_subfigure.rs
// occt: IGESDraw_NetworkSubfigure

/// Network subfigure entity
pub struct IgesDrawNetworkSubfigure {
    definition: Option<Box<dyn std::any::Any>>,
    location: (f64, f64, f64),
}

impl IgesDrawNetworkSubfigure {
    pub fn new() -> Self {
        IgesDrawNetworkSubfigure {
            definition: None,
            location: (0.0, 0.0, 0.0),
        }
    }

    pub fn init(&mut self, location: (f64, f64, f64)) {
        self.location = location;
    }

    pub fn location(&self) -> (f64, f64, f64) {
        self.location
    }
}

impl Default for IgesDrawNetworkSubfigure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ns = IgesDrawNetworkSubfigure::new();
        assert_eq!(ns.location(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_init() {
        let mut ns = IgesDrawNetworkSubfigure::new();
        ns.init((1.0, 2.0, 3.0));
        assert_eq!(ns.location(), (1.0, 2.0, 3.0));
    }
}
