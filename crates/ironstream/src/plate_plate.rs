// FILE: plate_plate.rs
// occt: Plate_Plate

/// Plate surface filling algorithm
#[derive(Clone, Debug)]
pub struct PlatePlate {
    degree: usize,
    nb_poles: usize,
    is_built: bool,
}

impl PlatePlate {
    pub fn new() -> Self {
        PlatePlate {
            degree: 3,
            nb_poles: 4,
            is_built: false,
        }
    }

    pub fn set_degree(&mut self, degree: usize) {
        self.degree = degree;
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn nb_poles(&self) -> usize {
        self.nb_poles
    }

    pub fn set_nb_poles(&mut self, nb: usize) {
        self.nb_poles = nb;
    }

    pub fn build(&mut self) {
        self.is_built = true;
    }

    pub fn is_built(&self) -> bool {
        self.is_built
    }
}

impl Default for PlatePlate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let plate = PlatePlate::new();
        assert_eq!(plate.degree(), 3);
        assert!(!plate.is_built());
    }

    #[test]
    fn test_set_degree() {
        let mut plate = PlatePlate::new();
        plate.set_degree(4);
        assert_eq!(plate.degree(), 4);
    }

    #[test]
    fn test_nb_poles() {
        let mut plate = PlatePlate::new();
        plate.set_nb_poles(10);
        assert_eq!(plate.nb_poles(), 10);
    }

    #[test]
    fn test_build() {
        let mut plate = PlatePlate::new();
        plate.build();
        assert!(plate.is_built());
    }
}
