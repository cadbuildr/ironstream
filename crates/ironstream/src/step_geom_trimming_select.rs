// FILE: step_geom_trimming_select.rs
// occt: StepGeom_TrimmingSelect

#[derive(Clone, Debug)]
pub enum StepGeomTrimmingSelect {
    CartesianPoint,
    ParameterValue(f64),
}

pub struct TrimmingSelectMember {
    param_value: Option<f64>,
}

impl TrimmingSelectMember {
    pub fn new() -> Self {
        TrimmingSelectMember {
            param_value: None,
        }
    }

    pub fn set_parameter_value(&mut self, value: f64) {
        self.param_value = Some(value);
    }

    pub fn parameter_value(&self) -> f64 {
        self.param_value.unwrap_or(0.0)
    }
}

pub struct TrimmingSelect {
    value: Option<StepGeomTrimmingSelect>,
    member: Option<TrimmingSelectMember>,
}

impl TrimmingSelect {
    pub fn new() -> Self {
        TrimmingSelect {
            value: None,
            member: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match &self.value {
            Some(StepGeomTrimmingSelect::CartesianPoint) => 1,
            Some(StepGeomTrimmingSelect::ParameterValue(_)) => 0,
            None => 0,
        }
    }

    pub fn new_member() -> TrimmingSelectMember {
        TrimmingSelectMember::new()
    }

    pub fn case_mem(&self, member: &TrimmingSelectMember) -> i32 {
        if member.param_value.is_some() {
            1
        } else {
            0
        }
    }

    pub fn cartesian_point(&self) -> Option<&StepGeomTrimmingSelect> {
        match &self.value {
            Some(StepGeomTrimmingSelect::CartesianPoint) => self.value.as_ref(),
            _ => None,
        }
    }

    pub fn set_parameter_value(&mut self, value: f64) {
        self.value = Some(StepGeomTrimmingSelect::ParameterValue(value));
    }

    pub fn parameter_value(&self) -> f64 {
        match &self.value {
            Some(StepGeomTrimmingSelect::ParameterValue(v)) => *v,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimming_select_creation() {
        let select = TrimmingSelect::new();
        assert_eq!(select.case_num(), 0);
        assert_eq!(select.parameter_value(), 0.0);
    }

    #[test]
    fn test_trimming_select_set_parameter_value() {
        let mut select = TrimmingSelect::new();
        select.set_parameter_value(2.5);
        assert_eq!(select.parameter_value(), 2.5);
        assert_eq!(select.case_num(), 0);
    }

    #[test]
    fn test_trimming_select_member() {
        let mut member = TrimmingSelectMember::new();
        member.set_parameter_value(1.5);
        assert_eq!(member.parameter_value(), 1.5);
    }

    #[test]
    fn test_trimming_select_case_mem() {
        let select = TrimmingSelect::new();
        let mut member = TrimmingSelectMember::new();
        assert_eq!(select.case_mem(&member), 0);

        member.set_parameter_value(5.0);
        assert_eq!(select.case_mem(&member), 1);
    }
}
