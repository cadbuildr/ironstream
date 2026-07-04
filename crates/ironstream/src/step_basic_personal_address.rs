// FILE: step_basic_personal_address.rs
// occt: StepBasic_PersonalAddress

#[derive(Clone, Debug)]
pub struct StepBasicPersonalAddress {
    internal_location: Option<String>,
    street_number: Option<String>,
    street: Option<String>,
    postal_box: Option<String>,
    town: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    facsimile_number: Option<String>,
    telephone_number: Option<String>,
    electronic_mail_address: Option<String>,
    telex_number: Option<String>,
    person_id: String,
}

impl StepBasicPersonalAddress {
    pub fn new() -> Self {
        Self {
            internal_location: None,
            street_number: None,
            street: None,
            postal_box: None,
            town: None,
            region: None,
            postal_code: None,
            country: None,
            facsimile_number: None,
            telephone_number: None,
            electronic_mail_address: None,
            telex_number: None,
            person_id: String::new(),
        }
    }

    pub fn init(&mut self, person_id: String) {
        self.person_id = person_id;
    }

    pub fn person(&self) -> &str { &self.person_id }
    pub fn set_person(&mut self, id: String) { self.person_id = id; }
}

impl Default for StepBasicPersonalAddress {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut a = StepBasicPersonalAddress::new();
        a.init("P-1".into());
        assert_eq!(a.person(), "P-1");
    }
}
