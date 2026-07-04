// FILE: step_data_p_descr.rs
// occt: StepData_PDescr

//! Describes a parameter in STEP format
pub struct StepDataPDescr {
    name: String,
    kind: i32,
}

impl StepDataPDescr {
    //! Creates a PDescr
    pub fn new() -> Self {
        StepDataPDescr {
            name: String::new(),
            kind: 0,
        }
    }

    //! Sets the name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    //! Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }

    //! Sets the kind
    pub fn set_kind(&mut self, kind: i32) {
        self.kind = kind;
    }

    //! Returns the kind
    pub fn kind(&self) -> i32 {
        self.kind
    }
}

impl Default for StepDataPDescr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_descr_new() {
        let descr = StepDataPDescr::new();
        assert_eq!(descr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut descr = StepDataPDescr::new();
        descr.set_name("param");
        assert_eq!(descr.name(), "param");
    }

    #[test]
    fn test_set_kind() {
        let mut descr = StepDataPDescr::new();
        descr.set_kind(5);
        assert_eq!(descr.kind(), 5);
    }
}
