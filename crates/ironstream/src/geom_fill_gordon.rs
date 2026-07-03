// FILE: geom_fill_gordon.rs
// occt: GeomFill_Gordon

/// Gordon surface filling algorithm
#[derive(Clone, Debug)]
pub struct GeomFillGordon {
    done: bool,
    num_curves: usize,
}

impl GeomFillGordon {
    pub fn new() -> Self {
        GeomFillGordon { done: false, num_curves: 0 }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn num_curves(&self) -> usize {
        self.num_curves
    }

    pub fn set_num_curves(&mut self, n: usize) {
        self.num_curves = n;
    }
}

impl Default for GeomFillGordon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let gordon = GeomFillGordon::new();
        assert!(!gordon.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut gordon = GeomFillGordon::new();
        gordon.set_done();
        assert!(gordon.is_done());
    }

    #[test]
    fn test_curves() {
        let mut gordon = GeomFillGordon::new();
        gordon.set_num_curves(5);
        assert_eq!(gordon.num_curves(), 5);
    }
}
