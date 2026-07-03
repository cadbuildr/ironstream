// FILE: geom_int_the_multi_line_of_wl_approx.rs
// occt: GeomInt_TheMultiLineOfWLApprox

pub struct GeomIntTheMultiLine {
    nb_points: usize,
}

impl GeomIntTheMultiLine {
    pub fn new() -> Self {
        GeomIntTheMultiLine { nb_points: 0 }
    }

    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    pub fn set_nb_points(&mut self, n: usize) {
        self.nb_points = n;
    }
}

impl Default for GeomIntTheMultiLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let line = GeomIntTheMultiLine::new();
        assert_eq!(line.nb_points(), 0);
    }

    #[test]
    fn test_set_nb_points() {
        let mut line = GeomIntTheMultiLine::new();
        line.set_nb_points(10);
        assert_eq!(line.nb_points(), 10);
    }
}
