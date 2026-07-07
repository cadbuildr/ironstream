// FILE: iges_basic_hierarchy.rs
// occt: IGESBasic_Hierarchy

/// Hierarchy, Type <406> Form <10>
/// Provides ability to control the hierarchy of each directory entry attribute.
pub struct IgesBasicHierarchy {
    nb_property_values: i32,
    line_font: i32,
    view: i32,
    entity_level: i32,
    blank_status: i32,
    line_weight: i32,
    color_num: i32,
}

impl IgesBasicHierarchy {
    /// Create a new Hierarchy with default values.
    pub fn new() -> Self {
        Self {
            nb_property_values: 6,
            line_font: 0,
            view: 0,
            entity_level: 0,
            blank_status: 0,
            line_weight: 0,
            color_num: 0,
        }
    }

    /// Set the fields of the class Hierarchy.
    /// - nb_prop_val: Number of Property values = 6
    /// - line_font: indicates the line font
    /// - view: indicates the view
    /// - entity_level: indicates the entity level
    /// - blank_status: indicates the blank status
    /// - line_wt: indicates the line weight
    /// - color_num: indicates the color num
    ///
    /// These can take 0 or 1:
    /// - 0: The directory entry attribute will apply to entities physically subordinate
    /// - 1: The directory entry attribute will not apply to physically subordinate entities
    pub fn init(
        &mut self,
        nb_prop_val: i32,
        line_font: i32,
        view: i32,
        entity_level: i32,
        blank_status: i32,
        line_wt: i32,
        color_num: i32,
    ) {
        self.nb_property_values = nb_prop_val;
        self.line_font = line_font;
        self.view = view;
        self.entity_level = entity_level;
        self.blank_status = blank_status;
        self.line_weight = line_wt;
        self.color_num = color_num;
    }

    /// Returns the number of property values, which should be 6.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the line font.
    pub fn new_line_font(&self) -> i32 {
        self.line_font
    }

    /// Returns the view.
    pub fn new_view(&self) -> i32 {
        self.view
    }

    /// Returns the entity level.
    pub fn new_entity_level(&self) -> i32 {
        self.entity_level
    }

    /// Returns the blank status.
    pub fn new_blank_status(&self) -> i32 {
        self.blank_status
    }

    /// Returns the line weight.
    pub fn new_line_weight(&self) -> i32 {
        self.line_weight
    }

    /// Returns the color number.
    pub fn new_color_num(&self) -> i32 {
        self.color_num
    }
}

impl Default for IgesBasicHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hier = IgesBasicHierarchy::new();
        assert_eq!(hier.nb_property_values(), 6);
        assert_eq!(hier.new_line_font(), 0);
        assert_eq!(hier.new_view(), 0);
        assert_eq!(hier.new_entity_level(), 0);
        assert_eq!(hier.new_blank_status(), 0);
        assert_eq!(hier.new_line_weight(), 0);
        assert_eq!(hier.new_color_num(), 0);
    }

    #[test]
    fn test_init() {
        let mut hier = IgesBasicHierarchy::new();
        hier.init(6, 1, 0, 1, 0, 1, 0);
        assert_eq!(hier.nb_property_values(), 6);
        assert_eq!(hier.new_line_font(), 1);
        assert_eq!(hier.new_view(), 0);
        assert_eq!(hier.new_entity_level(), 1);
        assert_eq!(hier.new_blank_status(), 0);
        assert_eq!(hier.new_line_weight(), 1);
        assert_eq!(hier.new_color_num(), 0);
    }

    #[test]
    fn test_binary_values() {
        let mut hier = IgesBasicHierarchy::new();
        hier.init(6, 0, 0, 0, 0, 0, 0);
        assert_eq!(hier.new_line_font(), 0);
        hier.init(6, 1, 1, 1, 1, 1, 1);
        assert_eq!(hier.new_line_font(), 1);
    }

    #[test]
    fn test_default() {
        let hier = IgesBasicHierarchy::default();
        assert_eq!(hier.nb_property_values(), 6);
        assert_eq!(hier.new_line_font(), 0);
    }
}
