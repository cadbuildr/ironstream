// FILE: step_dim_tol_tolerance_zone.rs
// occt: StepDimTol_ToleranceZone

pub struct ToleranceZone {
    pub name: Option<String>,
    pub description: Option<String>,
    pub form: Option<ToleranceZoneForm>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToleranceZoneForm {
    Cylindrical,
    Spherical,
    Planar,
    Linear,
}

impl ToleranceZone {
    pub fn new() -> Self {
        ToleranceZone {
            name: None,
            description: None,
            form: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_form(&mut self, form: ToleranceZoneForm) {
        self.form = Some(form);
    }

    pub fn get_form(&self) -> Option<ToleranceZoneForm> {
        self.form
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let zone = ToleranceZone::new();
        assert!(zone.name.is_none());
        assert!(zone.form.is_none());
    }

    #[test]
    fn test_set_form() {
        let mut zone = ToleranceZone::new();
        zone.set_form(ToleranceZoneForm::Cylindrical);
        assert_eq!(zone.get_form(), Some(ToleranceZoneForm::Cylindrical));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut zone = ToleranceZone::new();
        zone.set_name("zone1".to_string());
        assert_eq!(zone.get_name(), Some("zone1"));
    }
}
