// FILE: blend_func_ch_asym.rs
// occt: BlendFunc_ChAsym

#[derive(Clone)]
pub struct BlendFuncChAsym {
    dist1: f64,
    dist2: f64,
    choix: i32,
}

impl BlendFuncChAsym {
    pub fn new() -> Self {
        Self {
            dist1: 0.0,
            dist2: 0.0,
            choix: 0,
        }
    }

    pub fn set_distances(&mut self, d1: f64, d2: f64, choice: i32) {
        self.dist1 = d1;
        self.dist2 = d2;
        self.choix = choice;
    }

    pub fn dist1(&self) -> f64 {
        self.dist1
    }

    pub fn dist2(&self) -> f64 {
        self.dist2
    }

    pub fn choix(&self) -> i32 {
        self.choix
    }
}

impl Default for BlendFuncChAsym {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let asym = BlendFuncChAsym::new();
        assert_eq!(asym.dist1(), 0.0);
        assert_eq!(asym.dist2(), 0.0);
    }

    #[test]
    fn test_set_distances() {
        let mut asym = BlendFuncChAsym::new();
        asym.set_distances(1.5, 2.5, 1);
        assert_eq!(asym.dist1(), 1.5);
        assert_eq!(asym.dist2(), 2.5);
        assert_eq!(asym.choix(), 1);
    }
}
