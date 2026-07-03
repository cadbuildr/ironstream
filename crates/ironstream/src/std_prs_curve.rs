// FILE: std_prs_curve.rs
// occt: StdPrs_Curve

/// Presentation of a curve.
#[derive(Clone, Debug)]
pub struct Curve {
    pub curve_id: u32,
    pub point_count: usize,
    pub is_computed: bool,
}

impl Curve {
    pub fn new(curve_id: u32) -> Self {
        Self {
            curve_id,
            point_count: 0,
            is_computed: false,
        }
    }

    pub fn add_point(&mut self) {
        self.point_count += 1;
    }

    pub fn nb_points(&self) -> usize {
        self.point_count
    }

    pub fn compute(&mut self) {
        self.is_computed = true;
    }

    pub fn is_computed(&self) -> bool {
        self.is_computed
    }

    pub fn clear(&mut self) {
        self.point_count = 0;
        self.is_computed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_curve() {
        let curve = Curve::new(1);
        assert_eq!(curve.curve_id, 1);
        assert!(!curve.is_computed());
    }

    #[test]
    fn add_points() {
        let mut curve = Curve::new(1);
        for _ in 0..10 {
            curve.add_point();
        }
        assert_eq!(curve.nb_points(), 10);
    }

    #[test]
    fn compute() {
        let mut curve = Curve::new(1);
        curve.add_point();
        curve.compute();
        assert!(curve.is_computed());
    }

    #[test]
    fn clear() {
        let mut curve = Curve::new(1);
        curve.add_point();
        curve.compute();
        curve.clear();
        assert_eq!(curve.nb_points(), 0);
        assert!(!curve.is_computed());
    }
}
