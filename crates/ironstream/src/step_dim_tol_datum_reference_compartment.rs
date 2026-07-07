// FILE: step_dim_tol_datum_reference_compartment.rs
// occt: StepDimTol_DatumReferenceCompartment

pub struct StepDimTolDatumReferenceCompartment {
    name: String,
}

impl StepDimTolDatumReferenceCompartment {
    pub fn new(name: &str) -> Self {
        StepDimTolDatumReferenceCompartment {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_reference_compartment_new() {
        let comp = StepDimTolDatumReferenceCompartment::new("Compartment1");
        assert_eq!(comp.name(), "Compartment1");
    }
}
