// FILE: mat_zone.rs
// occt: MAT_Zone

/// Definition of zone of proximity of a BasicElt.
#[derive(Debug, Clone)]
pub struct MatZone {
    limited: bool,
    nb_arcs: i32,
}

impl MatZone {
    pub fn new() -> Self {
        MatZone {
            limited: false,
            nb_arcs: 0,
        }
    }

    pub fn number_of_arcs(&self) -> i32 {
        self.nb_arcs
    }

    pub fn limited(&self) -> bool {
        self.limited
    }

    pub fn no_empty_zone(&self) -> bool {
        self.nb_arcs > 0
    }
}

impl Default for MatZone {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone() {
        let z = MatZone::new();
        assert!(!z.limited());
        assert!(!z.no_empty_zone());
    }
}
