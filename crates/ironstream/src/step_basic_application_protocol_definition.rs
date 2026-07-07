// FILE: step_basic_application_protocol_definition.rs
// occt: StepBasic_ApplicationProtocolDefinition

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }
}

pub struct StepBasic_ApplicationProtocolDefinition {
    status: Option<Rc<RefCell<HString>>>,
    application_interpreted_model_schema_name: Option<Rc<RefCell<HString>>>,
    application_protocol_year: i32,
    application: Option<Rc<RefCell<dyn std::any::Any>>>,
}

impl StepBasic_ApplicationProtocolDefinition {
    pub fn new() -> Self {
        StepBasic_ApplicationProtocolDefinition {
            status: None,
            application_interpreted_model_schema_name: None,
            application_protocol_year: 0,
            application: None,
        }
    }

    pub fn init(
        &mut self,
        status: Option<Rc<RefCell<HString>>>,
        application_interpreted_model_schema_name: Option<Rc<RefCell<HString>>>,
        application_protocol_year: i32,
        application: Option<Rc<RefCell<dyn std::any::Any>>>,
    ) {
        self.status = status;
        self.application_interpreted_model_schema_name = application_interpreted_model_schema_name;
        self.application_protocol_year = application_protocol_year;
        self.application = application;
    }

    pub fn set_status(&mut self, status: Option<Rc<RefCell<HString>>>) {
        self.status = status;
    }

    pub fn status(&self) -> Option<Rc<RefCell<HString>>> {
        self.status.clone()
    }

    pub fn set_application_interpreted_model_schema_name(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
    ) {
        self.application_interpreted_model_schema_name = name;
    }

    pub fn application_interpreted_model_schema_name(&self) -> Option<Rc<RefCell<HString>>> {
        self.application_interpreted_model_schema_name.clone()
    }

    pub fn set_application_protocol_year(&mut self, year: i32) {
        self.application_protocol_year = year;
    }

    pub fn application_protocol_year(&self) -> i32 {
        self.application_protocol_year
    }

    pub fn set_application(&mut self, application: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.application = application;
    }

    pub fn application(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.application.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let apd = StepBasic_ApplicationProtocolDefinition::new();
        assert!(apd.status().is_none());
        assert_eq!(apd.application_protocol_year(), 0);
    }

    #[test]
    fn test_init() {
        let mut apd = StepBasic_ApplicationProtocolDefinition::new();
        let status = HString::new("APPROVED".to_string());
        let schema_name = HString::new("AP203".to_string());

        apd.init(Some(status), Some(schema_name), 2014, None);

        assert!(apd.status().is_some());
        assert!(apd.application_interpreted_model_schema_name().is_some());
        assert_eq!(apd.application_protocol_year(), 2014);
    }

    #[test]
    fn test_set_year() {
        let mut apd = StepBasic_ApplicationProtocolDefinition::new();
        apd.set_application_protocol_year(2020);
        assert_eq!(apd.application_protocol_year(), 2020);
    }
}
