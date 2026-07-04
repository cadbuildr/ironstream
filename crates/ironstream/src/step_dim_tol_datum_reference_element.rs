// FILE: step_dim_tol_datum_reference_element.rs
// occt: StepDimTol_DatumReferenceElement

pub struct StepDimTolDatumReferenceElement {
    name: String,
}

impl StepDimTolDatumReferenceElement {
    pub fn new(name: &str) -> Self {
        StepDimTolDatumReferenceElement {
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
    fn test_datum_reference_element_new() {
        let elem = StepDimTolDatumReferenceElement::new("Element1");
        assert_eq!(elem.name(), "Element1");
    }
}
