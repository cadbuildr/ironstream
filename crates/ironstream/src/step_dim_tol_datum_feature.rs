// FILE: step_dim_tol_datum_feature.rs
// occt: StepDimTol_DatumFeature

pub struct StepDimTolDatumFeature {
    name: String,
}

impl StepDimTolDatumFeature {
    pub fn new(name: &str) -> Self {
        StepDimTolDatumFeature {
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
    fn test_datum_feature_new() {
        let feature = StepDimTolDatumFeature::new("Feature1");
        assert_eq!(feature.name(), "Feature1");
    }
}
