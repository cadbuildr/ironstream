// FILE: iges_appli_flow.rs
// occt: IGESAppli_Flow

/// Represents flow information in IGES applications.
///
/// IGES Type 402 Form 23
/// Stores piping or flow network data.
#[derive(Clone, Debug)]
pub struct IgesAppliFlow {
    name: String,
    flow_type: i32,
}

impl IgesAppliFlow {
    /// Creates a new Flow entity.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            flow_type: 0,
        }
    }

    /// Initializes with flow name and type.
    pub fn init(&mut self, n: String, ft: i32) {
        self.name = n;
        self.flow_type = ft;
    }

    /// Returns the flow name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the flow type.
    pub fn flow_type(&self) -> i32 {
        self.flow_type
    }
}

impl Default for IgesAppliFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let flow = IgesAppliFlow::new();
        assert_eq!(flow.name(), "");
        assert_eq!(flow.flow_type(), 0);
    }

    #[test]
    fn test_init() {
        let mut flow = IgesAppliFlow::new();
        flow.init("coolant".to_string(), 1);

        assert_eq!(flow.name(), "coolant");
        assert_eq!(flow.flow_type(), 1);
    }
}
