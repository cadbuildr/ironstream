// FILE: step_geom_trimmed_curve.rs
// occt: StepGeom_TrimmedCurve

pub struct TrimmedCurve {
    basis_curve: Option<Box<dyn std::any::Any>>,
    trim1: Vec<TrimmingSelect>,
    trim2: Vec<TrimmingSelect>,
    sense_agreement: bool,
    master_representation: TrimmingPreference,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrimmingPreference {
    Cartesian,
    Parameter,
    Unspecified,
}

#[derive(Clone, Debug)]
pub enum TrimmingSelect {
    ParameterValue(f64),
    CartesianPoint,
}

impl TrimmedCurve {
    pub fn new() -> Self {
        TrimmedCurve {
            basis_curve: None,
            trim1: vec![],
            trim2: vec![],
            sense_agreement: false,
            master_representation: TrimmingPreference::Unspecified,
        }
    }

    pub fn init(
        &mut self,
        basis_curve: Option<Box<dyn std::any::Any>>,
        trim1: Vec<TrimmingSelect>,
        trim2: Vec<TrimmingSelect>,
        sense_agreement: bool,
        master_representation: TrimmingPreference,
    ) {
        self.basis_curve = basis_curve;
        self.trim1 = trim1;
        self.trim2 = trim2;
        self.sense_agreement = sense_agreement;
        self.master_representation = master_representation;
    }

    pub fn set_basis_curve(&mut self, basis_curve: Option<Box<dyn std::any::Any>>) {
        self.basis_curve = basis_curve;
    }

    pub fn basis_curve(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.basis_curve
    }

    pub fn set_trim1(&mut self, trim1: Vec<TrimmingSelect>) {
        self.trim1 = trim1;
    }

    pub fn trim1(&self) -> &[TrimmingSelect] {
        &self.trim1
    }

    pub fn trim1_value(&self, index: usize) -> Option<&TrimmingSelect> {
        self.trim1.get(index)
    }

    pub fn nb_trim1(&self) -> usize {
        self.trim1.len()
    }

    pub fn set_trim2(&mut self, trim2: Vec<TrimmingSelect>) {
        self.trim2 = trim2;
    }

    pub fn trim2(&self) -> &[TrimmingSelect] {
        &self.trim2
    }

    pub fn trim2_value(&self, index: usize) -> Option<&TrimmingSelect> {
        self.trim2.get(index)
    }

    pub fn nb_trim2(&self) -> usize {
        self.trim2.len()
    }

    pub fn set_sense_agreement(&mut self, sense_agreement: bool) {
        self.sense_agreement = sense_agreement;
    }

    pub fn sense_agreement(&self) -> bool {
        self.sense_agreement
    }

    pub fn set_master_representation(&mut self, master_representation: TrimmingPreference) {
        self.master_representation = master_representation;
    }

    pub fn master_representation(&self) -> TrimmingPreference {
        self.master_representation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimmed_curve_creation() {
        let curve = TrimmedCurve::new();
        assert_eq!(curve.nb_trim1(), 0);
        assert_eq!(curve.nb_trim2(), 0);
        assert_eq!(curve.sense_agreement(), false);
        assert_eq!(curve.master_representation(), TrimmingPreference::Unspecified);
    }

    #[test]
    fn test_trimmed_curve_init() {
        let mut curve = TrimmedCurve::new();
        let trim1 = vec![TrimmingSelect::ParameterValue(1.0)];
        let trim2 = vec![TrimmingSelect::ParameterValue(2.0)];
        curve.init(None, trim1, trim2, true, TrimmingPreference::Parameter);

        assert_eq!(curve.nb_trim1(), 1);
        assert_eq!(curve.nb_trim2(), 1);
        assert_eq!(curve.sense_agreement(), true);
        assert_eq!(curve.master_representation(), TrimmingPreference::Parameter);
    }

    #[test]
    fn test_trimmed_curve_setters() {
        let mut curve = TrimmedCurve::new();
        curve.set_sense_agreement(true);
        curve.set_master_representation(TrimmingPreference::Cartesian);

        assert_eq!(curve.sense_agreement(), true);
        assert_eq!(curve.master_representation(), TrimmingPreference::Cartesian);
    }
}
