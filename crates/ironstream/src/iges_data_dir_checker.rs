// FILE: iges_data_dir_checker.rs
// occt: IGESData_DirChecker

//! Centralizes general checks upon an IGES Entity's Directory Part.
//! Checks criteria for fields: Ignored, Required, or Required with given Value.

#[derive(Clone, Debug)]
pub enum DefType {
    Void,
    Value,
    Reference,
    Any,
    ErrorVal,
    ErrorRef,
}

/// DirChecker performs checks on IGES Entity directory parts.
/// Handles criteria for Type/Form numbers, Structure, LineFont, LineWeight, Color,
/// Blank Status, Subordinate Status, UseFlag, and Hierarchy Status.
#[derive(Clone, Debug)]
pub struct DirChecker {
    is_set: bool,
    type_num: Option<i32>,
    form1: Option<i32>,
    form2: Option<i32>,
    structure: Option<DefType>,
    line_font: Option<DefType>,
    line_weight: Option<DefType>,
    color: Option<DefType>,
    graphics_hierarchy: Option<i32>,
    blank_status: Option<i32>,
    subordinate_status: Option<i32>,
    use_flag: Option<i32>,
    hierarchy_status: Option<i32>,
}

impl DirChecker {
    /// Returns a DirChecker with no criterium at all to be checked
    pub fn new() -> Self {
        DirChecker {
            is_set: false,
            type_num: None,
            form1: None,
            form2: None,
            structure: None,
            line_font: None,
            line_weight: None,
            color: None,
            graphics_hierarchy: None,
            blank_status: None,
            subordinate_status: None,
            use_flag: None,
            hierarchy_status: None,
        }
    }

    /// Returns a DirChecker with Required Type
    pub fn with_type(atype: i32) -> Self {
        let mut dc = Self::new();
        dc.type_num = Some(atype);
        dc.is_set = true;
        dc
    }

    /// Returns a DirChecker with Required Type and Form numbers
    pub fn with_type_form(atype: i32, aform: i32) -> Self {
        let mut dc = Self::new();
        dc.type_num = Some(atype);
        dc.form1 = Some(aform);
        dc.form2 = Some(aform);
        dc.is_set = true;
        dc
    }

    /// Returns a DirChecker with Required Type and Range for Form number
    pub fn with_type_form_range(atype: i32, aform1: i32, aform2: i32) -> Self {
        let mut dc = Self::new();
        dc.type_num = Some(atype);
        dc.form1 = Some(aform1);
        dc.form2 = Some(aform2);
        dc.is_set = true;
        dc
    }

    /// Returns True if at least one criterium has already been set
    pub fn is_set(&self) -> bool {
        self.is_set
    }

    /// Sets a DirChecker with most current criteria (Structure Ignored)
    pub fn set_default(&mut self) {
        self.structure = Some(DefType::Void);
        self.is_set = true;
    }

    /// Sets Structure criterium
    pub fn structure(&mut self, crit: DefType) {
        self.structure = Some(crit);
        self.is_set = true;
    }

    /// Sets LineFont criterium
    pub fn line_font(&mut self, crit: DefType) {
        self.line_font = Some(crit);
        self.is_set = true;
    }

    /// Sets LineWeight criterium
    pub fn line_weight(&mut self, crit: DefType) {
        self.line_weight = Some(crit);
        self.is_set = true;
    }

    /// Sets Color criterium
    pub fn color(&mut self, crit: DefType) {
        self.color = Some(crit);
        self.is_set = true;
    }

    /// Sets Graphics data to be ignored according hierarchy status
    pub fn graphics_ignored(&mut self, hierarchy: Option<i32>) {
        self.graphics_hierarchy = hierarchy;
        self.is_set = true;
    }

    /// Sets Blank Status to be ignored
    pub fn blank_status_ignored(&mut self) {
        self.blank_status = Some(0);
        self.is_set = true;
    }

    /// Sets Blank Status to be required at a given value
    pub fn blank_status_required(&mut self, val: i32) {
        self.blank_status = Some(val);
        self.is_set = true;
    }

    /// Sets Subordinate Status to be ignored
    pub fn subordinate_status_ignored(&mut self) {
        self.subordinate_status = Some(0);
        self.is_set = true;
    }

    /// Sets Subordinate Status to be required at a given value
    pub fn subordinate_status_required(&mut self, val: i32) {
        self.subordinate_status = Some(val);
        self.is_set = true;
    }

    /// Sets UseFlag to be ignored
    pub fn use_flag_ignored(&mut self) {
        self.use_flag = Some(0);
        self.is_set = true;
    }

    /// Sets UseFlag to be required at a given value
    pub fn use_flag_required(&mut self, val: i32) {
        self.use_flag = Some(val);
        self.is_set = true;
    }

    /// Sets Hierarchy Status to be ignored
    pub fn hierarchy_status_ignored(&mut self) {
        self.hierarchy_status = Some(0);
        self.is_set = true;
    }

    /// Sets Hierarchy Status to be required at a given value
    pub fn hierarchy_status_required(&mut self, val: i32) {
        self.hierarchy_status = Some(val);
        self.is_set = true;
    }

    /// Returns the required type number if set
    pub fn type_num(&self) -> Option<i32> {
        self.type_num
    }

    /// Returns the required form range if set
    pub fn form_range(&self) -> Option<(i32, i32)> {
        match (self.form1, self.form2) {
            (Some(f1), Some(f2)) => Some((f1, f2)),
            _ => None,
        }
    }
}

impl Default for DirChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dc = DirChecker::new();
        assert!(!dc.is_set());
        assert_eq!(dc.type_num(), None);
    }

    #[test]
    fn test_with_type() {
        let dc = DirChecker::with_type(42);
        assert!(dc.is_set());
        assert_eq!(dc.type_num(), Some(42));
    }

    #[test]
    fn test_with_type_form() {
        let dc = DirChecker::with_type_form(10, 5);
        assert!(dc.is_set());
        assert_eq!(dc.type_num(), Some(10));
        assert_eq!(dc.form_range(), Some((5, 5)));
    }

    #[test]
    fn test_with_type_form_range() {
        let dc = DirChecker::with_type_form_range(20, 0, 3);
        assert!(dc.is_set());
        assert_eq!(dc.type_num(), Some(20));
        assert_eq!(dc.form_range(), Some((0, 3)));
    }

    #[test]
    fn test_set_default() {
        let mut dc = DirChecker::new();
        assert!(!dc.is_set());
        dc.set_default();
        assert!(dc.is_set());
    }

    #[test]
    fn test_structure() {
        let mut dc = DirChecker::new();
        dc.structure(DefType::Void);
        assert!(dc.is_set());
    }

    #[test]
    fn test_line_font() {
        let mut dc = DirChecker::new();
        dc.line_font(DefType::Value);
        assert!(dc.is_set());
    }

    #[test]
    fn test_blank_status_ignored() {
        let mut dc = DirChecker::new();
        dc.blank_status_ignored();
        assert!(dc.is_set());
    }

    #[test]
    fn test_blank_status_required() {
        let mut dc = DirChecker::new();
        dc.blank_status_required(1);
        assert!(dc.is_set());
    }

    #[test]
    fn test_hierarchy_status() {
        let mut dc = DirChecker::new();
        dc.hierarchy_status_ignored();
        assert!(dc.is_set());

        let mut dc2 = DirChecker::new();
        dc2.hierarchy_status_required(5);
        assert!(dc2.is_set());
    }

    #[test]
    fn test_graphics_ignored() {
        let mut dc = DirChecker::new();
        dc.graphics_ignored(None);
        assert!(dc.is_set());

        let mut dc2 = DirChecker::new();
        dc2.graphics_ignored(Some(2));
        assert!(dc2.is_set());
    }
}
