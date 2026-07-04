// FILE: step_basic_person_and_organization_role.rs
// occt: StepBasic_PersonAndOrganizationRole

#[derive(Clone, Debug)]
pub struct StepBasicPersonAndOrganizationRole {
    name: String,
}

impl StepBasicPersonAndOrganizationRole {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicPersonAndOrganizationRole {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut r = StepBasicPersonAndOrganizationRole::new();
        r.init("Manager".into());
        assert_eq!(r.name(), "Manager");
    }
}
