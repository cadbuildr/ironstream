// FILE: std_prs_pole_curve.rs
// occt: StdPrs_PoleCurve

/// Pole curve presentation (control polygon).
#[derive(Clone, Debug)]
pub struct PoleCurve {
    pub curve_id: u32,
    pub pole_count: usize,
}

impl PoleCurve {
    pub fn new(curve_id: u32) -> Self {
        Self {
            curve_id,
            pole_count: 0,
        }
    }

    pub fn add_pole(&mut self) {
        self.pole_count += 1;
    }

    pub fn nb_poles(&self) -> usize {
        self.pole_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_curve() {
        let curve = PoleCurve::new(1);
        assert_eq!(curve.curve_id, 1);
        assert_eq!(curve.nb_poles(), 0);
    }

    #[test]
    fn add_poles() {
        let mut curve = PoleCurve::new(1);
        for _ in 0..5 {
            curve.add_pole();
        }
        assert_eq!(curve.nb_poles(), 5);
    }
}
