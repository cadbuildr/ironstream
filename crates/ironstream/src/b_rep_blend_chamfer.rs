// FILE: b_rep_blend_chamfer.rs
// occt: BRepBlend_Chamfer

#[derive(Clone, Debug)]
pub struct BRepBlendChamfer {
    distance1: f64,
    distance2: f64,
}

impl BRepBlendChamfer {
    pub fn new() -> Self {
        BRepBlendChamfer {
            distance1: 0.0,
            distance2: 0.0,
        }
    }

    pub fn distance1(&self) -> f64 {
        self.distance1
    }

    pub fn distance2(&self) -> f64 {
        self.distance2
    }

    pub fn set_distances(&mut self, d1: f64, d2: f64) {
        self.distance1 = d1;
        self.distance2 = d2;
    }
}

impl Default for BRepBlendChamfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ch = BRepBlendChamfer::new();
        assert_eq!(ch.distance1(), 0.0);
        assert_eq!(ch.distance2(), 0.0);
    }

    #[test]
    fn test_set_distances() {
        let mut ch = BRepBlendChamfer::new();
        ch.set_distances(1.0, 2.0);
        assert_eq!(ch.distance1(), 1.0);
        assert_eq!(ch.distance2(), 2.0);
    }
}
