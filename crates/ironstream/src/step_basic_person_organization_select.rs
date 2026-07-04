// FILE: step_basic_person_organization_select.rs
// occt: StepBasic_PersonOrganizationSelect

#[derive(Clone, Debug)]
pub enum StepBasicPersonOrganizationSelect {
    Person(String),
    Organization(String),
}

impl StepBasicPersonOrganizationSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            Self::Person(_) => 1,
            Self::Organization(_) => 2,
        }
    }

    pub fn person(&self) -> Option<&str> {
        match self { Self::Person(v) => Some(v), _ => None }
    }

    pub fn organization(&self) -> Option<&str> {
        match self { Self::Organization(v) => Some(v), _ => None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        let p = StepBasicPersonOrganizationSelect::Person("P-1".into());
        assert_eq!(p.case_num(), 1);
        let o = StepBasicPersonOrganizationSelect::Organization("O-1".into());
        assert_eq!(o.case_num(), 2);
    }
}
