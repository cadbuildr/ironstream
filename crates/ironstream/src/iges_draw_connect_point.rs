// FILE: iges_draw_connect_point.rs
// occt: IGESDraw_ConnectPoint

/// defines IGESConnectPoint, Type <132> Form Number <0>
/// in package IGESDraw
///
/// Connect Point Entity describes a point of connection for
/// zero, one or more entities. Its referenced from Composite
/// curve, or Network Subfigure Definition/Instance, or Flow
/// Associative Instance, or it may stand alone.
pub struct IgesDrawConnectPoint {
    point: (f64, f64, f64),
    display_symbol: Option<Box<dyn std::any::Any>>,
    type_flag: i32,
    function_flag: i32,
    function_identifier: String,
    identifier_template: Option<Box<dyn std::any::Any>>,
    function_name: String,
    function_template: Option<Box<dyn std::any::Any>>,
    point_identifier: i32,
    function_code: i32,
    swap_flag: bool,
    owner_subfigure: Option<Box<dyn std::any::Any>>,
}

impl IgesDrawConnectPoint {
    /// Create a new ConnectPoint
    pub fn new() -> Self {
        IgesDrawConnectPoint {
            point: (0.0, 0.0, 0.0),
            display_symbol: None,
            type_flag: 0,
            function_flag: 0,
            function_identifier: String::new(),
            identifier_template: None,
            function_name: String::new(),
            function_template: None,
            point_identifier: 0,
            function_code: 0,
            swap_flag: false,
            owner_subfigure: None,
        }
    }

    /// This method is used to set the fields of the class ConnectPoint
    pub fn init(
        &mut self,
        point: (f64, f64, f64),
        type_flag: i32,
        function_flag: i32,
        function_identifier: String,
        function_name: String,
        point_identifier: i32,
        function_code: i32,
        swap_flag: bool,
    ) {
        self.point = point;
        self.type_flag = type_flag;
        self.function_flag = function_flag;
        self.function_identifier = function_identifier;
        self.function_name = function_name;
        self.point_identifier = point_identifier;
        self.function_code = function_code;
        self.swap_flag = swap_flag;
    }

    /// returns the coordinate of the connection point
    pub fn point(&self) -> (f64, f64, f64) {
        self.point
    }

    /// returns the Transformed coordinate of the connection point
    pub fn transformed_point(&self) -> (f64, f64, f64) {
        self.point
    }

    /// returns True if Display symbol is specified
    pub fn has_display_symbol(&self) -> bool {
        self.display_symbol.is_some()
    }

    /// if display symbol specified returns display symbol geometric entity
    pub fn display_symbol(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.display_symbol
    }

    /// return value specifies a particular type of connection
    pub fn type_flag(&self) -> i32 {
        self.type_flag
    }

    /// returns Function Code that specifies a particular function for the connection
    pub fn function_flag(&self) -> i32 {
        self.function_flag
    }

    /// return HAsciiString identifying Pin Number or Nozzle Label etc.
    pub fn function_identifier(&self) -> &str {
        &self.function_identifier
    }

    /// returns True if Text Display Template is specified for Identifier
    pub fn has_identifier_template(&self) -> bool {
        self.identifier_template.is_some()
    }

    /// if Text Display Template for the Function Identifier is defined
    pub fn identifier_template(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.identifier_template
    }

    /// returns Connection Point Function Name
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// returns True if Text Display Template is specified for Function Name
    pub fn has_function_template(&self) -> bool {
        self.function_template.is_some()
    }

    /// if Text Display Template for the Function Name is defined
    pub fn function_template(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.function_template
    }

    /// returns the Unique Connect Point Identifier
    pub fn point_identifier(&self) -> i32 {
        self.point_identifier
    }

    /// returns the Connect Point Function Code
    pub fn function_code(&self) -> i32 {
        self.function_code
    }

    /// return value = 0 : Connect point may be swapped
    /// = 1 : Connect point may not be swapped
    pub fn swap_flag(&self) -> bool {
        self.swap_flag
    }

    /// returns True if Network Subfigure Instance/Definition Entity is specified
    pub fn has_owner_subfigure(&self) -> bool {
        self.owner_subfigure.is_some()
    }

    /// returns "owner" Network Subfigure Instance Entity
    pub fn owner_subfigure(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.owner_subfigure
    }
}

impl Default for IgesDrawConnectPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cp = IgesDrawConnectPoint::new();
        assert_eq!(cp.point(), (0.0, 0.0, 0.0));
        assert_eq!(cp.type_flag(), 0);
        assert!(!cp.has_display_symbol());
    }

    #[test]
    fn test_init() {
        let mut cp = IgesDrawConnectPoint::new();
        cp.init(
            (1.0, 2.0, 3.0),
            101,
            1,
            "PIN1".to_string(),
            "Power".to_string(),
            1,
            0,
            false,
        );

        assert_eq!(cp.point(), (1.0, 2.0, 3.0));
        assert_eq!(cp.type_flag(), 101);
        assert_eq!(cp.function_flag(), 1);
        assert_eq!(cp.function_identifier(), "PIN1");
        assert_eq!(cp.function_name(), "Power");
        assert_eq!(cp.point_identifier(), 1);
        assert!(!cp.swap_flag());
    }

    #[test]
    fn test_swap_flag() {
        let mut cp = IgesDrawConnectPoint::new();
        cp.init((0.0, 0.0, 0.0), 0, 0, "".to_string(), "".to_string(), 0, 0, true);
        assert!(cp.swap_flag());
    }
}
