// FILE: shape_process_ext.rs
// occt: ShapeProcess_OperLibrary, ShapeProcess_ShapeContext, ShapeProcess_UOperator

// occt-ref: ShapeProcess_Context
/// Holds parameters and results for a shape processing sequence.
#[derive(Clone, Debug, Default)]
pub struct ShapeContext {
    pub shape_name: String,
    pub params: Vec<(String, String)>,
    pub messages: Vec<String>,
}

impl ShapeContext {
    pub fn new(shape_name: impl Into<String>) -> Self {
        Self { shape_name: shape_name.into(), params: Vec::new(), messages: Vec::new() }
    }

    pub fn set_param(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let k = key.into();
        if let Some(p) = self.params.iter_mut().find(|(pk, _)| pk == &k) {
            p.1 = value.into();
        } else {
            self.params.push((k, value.into()));
        }
    }

    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.params.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    pub fn add_message(&mut self, msg: impl Into<String>) {
        self.messages.push(msg.into());
    }
}

// occt: ShapeProcess_UOperator // (operator definition)
#[derive(Clone, Debug)]
pub struct OperatorDef {
    pub name: String,
    pub description: String,
}

impl OperatorDef {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self { name: name.into(), description: description.into() }
    }
}

// occt: ShapeProcess_OperLibrary
/// Registry of named shape-processing operators.
#[derive(Clone, Debug, Default)]
pub struct OperLibrary {
    operators: Vec<OperatorDef>,
}

impl OperLibrary {
    pub fn new() -> Self { Self { operators: Vec::new() } }

    pub fn register(&mut self, op: OperatorDef) {
        self.operators.push(op);
    }

    pub fn find(&self, name: &str) -> Option<&OperatorDef> {
        self.operators.iter().find(|o| o.name == name)
    }

    pub fn nb_operators(&self) -> usize { self.operators.len() }

    pub fn standard_operators() -> Self {
        let mut lib = Self::new();
        lib.register(OperatorDef::new("DirectFaces", "Convert indirect surfaces"));
        lib.register(OperatorDef::new("SameParameter", "Fix same-parameter on edges"));
        lib.register(OperatorDef::new("SetTolerance", "Set tolerances"));
        lib.register(OperatorDef::new("SplitAngle", "Split faces by angle"));
        lib.register(OperatorDef::new("BSplineRestriction", "Restrict B-spline degree"));
        lib.register(OperatorDef::new("ElementaryToRevolution", "Convert elementary surfaces"));
        lib.register(OperatorDef::new("SweptToElementary", "Convert swept surfaces"));
        lib.register(OperatorDef::new("SurfaceToBSpline", "Convert to B-spline"));
        lib
    }
}

// occt: ShapeProcess_Operator // (execution handle)
#[derive(Clone, Debug)]
pub struct Operator {
    pub def: OperatorDef,
    pub executed: bool,
    pub modified: bool,
}

impl Operator {
    pub fn new(def: OperatorDef) -> Self {
        Self { def, executed: false, modified: false }
    }

    pub fn perform(&mut self, ctx: &mut ShapeContext) -> bool {
        ctx.add_message(format!("Performed: {}", self.def.name));
        self.executed = true;
        self.modified = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_context_params() {
        let mut ctx = ShapeContext::new("Box_1");
        ctx.set_param("Tolerance", "1e-5");
        assert_eq!(ctx.get_param("Tolerance"), Some("1e-5"));
        assert_eq!(ctx.get_param("Missing"), None);
    }

    #[test]
    fn oper_library_standard() {
        let lib = OperLibrary::standard_operators();
        assert!(lib.nb_operators() > 0);
        assert!(lib.find("SameParameter").is_some());
        assert!(lib.find("DoesNotExist").is_none());
    }

    #[test]
    fn operator_perform() {
        let mut ctx = ShapeContext::new("Shape");
        let def = OperatorDef::new("TestOp", "test");
        let mut op = Operator::new(def);
        assert!(op.perform(&mut ctx));
        assert!(op.executed);
        assert!(!ctx.messages.is_empty());
    }
}
