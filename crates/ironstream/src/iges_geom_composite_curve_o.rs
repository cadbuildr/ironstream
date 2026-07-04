// FILE: iges_geom_composite_curve_o.rs
// occt: IGESGeom_CompositeCurve

pub struct IgesGeomCompositeCurve {
    entities: Vec<Option<Box<dyn std::any::Any>>>,
}

impl IgesGeomCompositeCurve {
    pub fn new() -> Self {
        IgesGeomCompositeCurve {
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self, entities: Vec<Option<Box<dyn std::any::Any>>>) {
        self.entities = entities;
    }

    pub fn nb_curves(&self) -> usize {
        self.entities.len()
    }

    pub fn curve(&self, index: usize) -> Option<&Box<dyn std::any::Any>> {
        if index > 0 && index <= self.entities.len() {
            self.entities[index - 1].as_ref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_curve_creation() {
        let cc = IgesGeomCompositeCurve::new();
        assert_eq!(cc.nb_curves(), 0);
    }

    #[test]
    fn test_composite_curve_init() {
        let mut cc = IgesGeomCompositeCurve::new();
        let entities: Vec<Option<Box<dyn std::any::Any>>> = vec![
            Some(Box::new(1)),
            Some(Box::new(2)),
            Some(Box::new(3)),
        ];
        cc.init(entities);
        assert_eq!(cc.nb_curves(), 3);
    }

    #[test]
    fn test_curve_access() {
        let mut cc = IgesGeomCompositeCurve::new();
        let entities: Vec<Option<Box<dyn std::any::Any>>> = vec![
            Some(Box::new(1)),
            Some(Box::new(2)),
        ];
        cc.init(entities);
        assert!(cc.curve(1).is_some());
        assert!(cc.curve(2).is_some());
        assert!(cc.curve(3).is_none());
        assert!(cc.curve(0).is_none());
    }
}
