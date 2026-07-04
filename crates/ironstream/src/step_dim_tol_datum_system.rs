// FILE: step_dim_tol_datum_system.rs
// occt: StepDimTol_DatumSystem

use std::collections::HashMap;

pub struct StepDimTolDatumSystem {
    datums: Vec<String>,
}

impl StepDimTolDatumSystem {
    pub fn new() -> Self {
        StepDimTolDatumSystem {
            datums: Vec::new(),
        }
    }

    pub fn add_datum(&mut self, datum: &str) {
        self.datums.push(datum.to_string());
    }

    pub fn nb_datums(&self) -> usize {
        self.datums.len()
    }

    pub fn datum(&self, idx: usize) -> Option<&str> {
        if idx < self.datums.len() {
            Some(&self.datums[idx])
        } else {
            None
        }
    }
}

impl Default for StepDimTolDatumSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_system_new() {
        let system = StepDimTolDatumSystem::new();
        assert_eq!(system.nb_datums(), 0);
    }

    #[test]
    fn test_add_datum() {
        let mut system = StepDimTolDatumSystem::new();
        system.add_datum("A");
        system.add_datum("B");
        assert_eq!(system.nb_datums(), 2);
        assert_eq!(system.datum(0), Some("A"));
    }
}
