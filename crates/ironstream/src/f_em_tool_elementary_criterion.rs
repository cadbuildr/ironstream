// FILE: f_em_tool_elementary_criterion.rs
// occt: FEmTool_ElementaryCriterion

/// Elementary criterion for finite element method.
pub struct FemToolElementaryCriterion {
    value: f64,
}

impl FemToolElementaryCriterion {
    pub fn new(value: f64) -> Self {
        FemToolElementaryCriterion { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let crit = FemToolElementaryCriterion::new(1.5);
        assert_eq!(crit.value(), 1.5);
    }

    #[test]
    fn test_set_value() {
        let mut crit = FemToolElementaryCriterion::new(1.5);
        crit.set_value(2.5);
        assert_eq!(crit.value(), 2.5);
    }
}
