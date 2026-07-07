// FILE: step_shape_boolean_operand.rs
// occt: StepShape_BooleanOperand

/// Placeholder for SolidModel
#[derive(Clone, Debug, PartialEq)]
pub struct SolidModel {
    id: String,
}

/// Placeholder for HalfSpaceSolid
#[derive(Clone, Debug, PartialEq)]
pub struct HalfSpaceSolid {
    id: String,
}

/// Placeholder for BooleanResult
#[derive(Clone, Debug, PartialEq)]
pub struct BooleanResult {
    id: String,
}

/// Placeholder for CsgPrimitive
#[derive(Clone, Debug, PartialEq)]
pub struct CsgPrimitive {
    id: String,
}

/// Represents a boolean operand in CSG operations
pub struct BooleanOperand {
    solid_model: Option<SolidModel>,
    half_space_solid: Option<HalfSpaceSolid>,
    csg_primitive: Option<CsgPrimitive>,
    boolean_result: Option<BooleanResult>,
    type_of_content: i32,
}

impl BooleanOperand {
    /// Create a new BooleanOperand
    pub fn new() -> Self {
        BooleanOperand {
            solid_model: None,
            half_space_solid: None,
            csg_primitive: None,
            boolean_result: None,
            type_of_content: 0,
        }
    }

    /// Set the type of content
    pub fn set_type_of_content(&mut self, type_of_content: i32) {
        self.type_of_content = type_of_content;
    }

    /// Get the type of content
    pub fn type_of_content(&self) -> i32 {
        self.type_of_content
    }

    /// Get the solid model
    pub fn solid_model(&self) -> Option<&SolidModel> {
        self.solid_model.as_ref()
    }

    /// Set the solid model
    pub fn set_solid_model(&mut self, solid_model: SolidModel) {
        self.solid_model = Some(solid_model);
    }

    /// Get the half space solid
    pub fn half_space_solid(&self) -> Option<&HalfSpaceSolid> {
        self.half_space_solid.as_ref()
    }

    /// Set the half space solid
    pub fn set_half_space_solid(&mut self, half_space: HalfSpaceSolid) {
        self.half_space_solid = Some(half_space);
    }

    /// Get the CSG primitive
    pub fn csg_primitive(&self) -> Option<&CsgPrimitive> {
        self.csg_primitive.as_ref()
    }

    /// Set the CSG primitive
    pub fn set_csg_primitive(&mut self, csg: CsgPrimitive) {
        self.csg_primitive = Some(csg);
    }

    /// Get the boolean result
    pub fn boolean_result(&self) -> Option<&BooleanResult> {
        self.boolean_result.as_ref()
    }

    /// Set the boolean result
    pub fn set_boolean_result(&mut self, result: BooleanResult) {
        self.boolean_result = Some(result);
    }
}

impl Default for BooleanOperand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let operand = BooleanOperand::new();
        assert_eq!(operand.type_of_content(), 0);
        assert_eq!(operand.solid_model(), None);
        assert_eq!(operand.half_space_solid(), None);
    }

    #[test]
    fn test_set_type_of_content() {
        let mut operand = BooleanOperand::new();
        operand.set_type_of_content(1);
        assert_eq!(operand.type_of_content(), 1);
    }

    #[test]
    fn test_set_solid_model() {
        let mut operand = BooleanOperand::new();
        let solid = SolidModel {
            id: "solid1".to_string(),
        };
        operand.set_solid_model(solid.clone());
        assert_eq!(operand.solid_model(), Some(&solid));
    }

    #[test]
    fn test_set_half_space_solid() {
        let mut operand = BooleanOperand::new();
        let half_space = HalfSpaceSolid {
            id: "half1".to_string(),
        };
        operand.set_half_space_solid(half_space.clone());
        assert_eq!(operand.half_space_solid(), Some(&half_space));
    }
}
