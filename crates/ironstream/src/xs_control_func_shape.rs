// FILE: xs_control_func_shape.rs
// occt: XSControl_FuncShape

/// Shape function handler for the control framework.
/// Provides shape-specific function handling.
#[derive(Clone, Debug)]
pub struct XSControlFuncShape {
    /// Function identifier
    func_id: u32,
    /// Function name
    name: String,
}

impl XSControlFuncShape {
    /// Creates a new shape function handler.
    pub fn new(name: &str) -> Self {
        Self {
            func_id: 0,
            name: String::from(name),
        }
    }

    /// Returns the function ID.
    pub fn id(&self) -> u32 {
        self.func_id
    }

    /// Returns the function name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the function name.
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

impl Default for XSControlFuncShape {
    fn default() -> Self {
        Self::new("ShapeFunc")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = XSControlFuncShape::new("ProcessShape");
        assert_eq!(func.name(), "ProcessShape");
    }

    #[test]
    fn test_set_name() {
        let mut func = XSControlFuncShape::new("Old");
        func.set_name("New");
        assert_eq!(func.name(), "New");
    }

    #[test]
    fn test_default() {
        let func = XSControlFuncShape::default();
        assert_eq!(func.name(), "ShapeFunc");
    }
}
