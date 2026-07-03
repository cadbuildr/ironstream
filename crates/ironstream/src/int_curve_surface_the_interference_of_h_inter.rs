// FILE: int_curve_surface_the_interference_of_h_inter.rs
// occt: IntCurveSurface_TheInterferenceOfHInter

pub struct IntCurveSurfaceTheInterferenceOfHInter {
    nb_interferences: usize,
}

impl IntCurveSurfaceTheInterferenceOfHInter {
    pub fn new() -> Self {
        IntCurveSurfaceTheInterferenceOfHInter {
            nb_interferences: 0,
        }
    }

    pub fn nb_interferences(&self) -> usize {
        self.nb_interferences
    }

    pub fn set_nb_interferences(&mut self, n: usize) {
        self.nb_interferences = n;
    }
}

impl Default for IntCurveSurfaceTheInterferenceOfHInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let inter = IntCurveSurfaceTheInterferenceOfHInter::new();
        assert_eq!(inter.nb_interferences(), 0);
    }

    #[test]
    fn test_set_nb_interferences() {
        let mut inter = IntCurveSurfaceTheInterferenceOfHInter::new();
        inter.set_nb_interferences(5);
        assert_eq!(inter.nb_interferences(), 5);
    }
}
