// FILE: iges_appli_nodal_results.rs
// occt: IGESAppli_NodalResults

/// Stores FEA nodal analysis results.
#[derive(Clone, Debug)]
pub struct IgesAppliNodalResults {
    node_id: i32,
    result_type: i32,
    values: Vec<f64>,
}

impl IgesAppliNodalResults {
    pub fn new() -> Self {
        Self {
            node_id: 0,
            result_type: 0,
            values: Vec::new(),
        }
    }

    pub fn init(&mut self, nid: i32, rtype: i32, vals: Vec<f64>) {
        self.node_id = nid;
        self.result_type = rtype;
        self.values = vals;
    }

    pub fn node_id(&self) -> i32 {
        self.node_id
    }

    pub fn result_type(&self) -> i32 {
        self.result_type
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

impl Default for IgesAppliNodalResults {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut results = IgesAppliNodalResults::new();
        results.init(42, 1, vec![10.5, 20.3, 15.8]);

        assert_eq!(results.node_id(), 42);
        assert_eq!(results.result_type(), 1);
        assert_eq!(results.values(), &[10.5, 20.3, 15.8]);
    }
}
