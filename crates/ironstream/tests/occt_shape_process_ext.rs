extern crate ironstream;

use ironstream::shape_process_ext::{OperLibrary, OperatorDef, Operator, ShapeContext};

#[test]
fn shape_context_set_and_get_param() {
    let mut ctx = ShapeContext::new("Cylinder");
    ctx.set_param("Tolerance", "1e-6");
    assert_eq!(ctx.get_param("Tolerance"), Some("1e-6"));
    // Update existing key
    ctx.set_param("Tolerance", "2e-6");
    assert_eq!(ctx.get_param("Tolerance"), Some("2e-6"));
}

#[test]
fn shape_context_missing_param() {
    let ctx = ShapeContext::new("Box");
    assert_eq!(ctx.get_param("NonExistent"), None);
}

#[test]
fn oper_library_register_and_find() {
    let mut lib = OperLibrary::new();
    lib.register(OperatorDef::new("MyOp", "does something"));
    assert_eq!(lib.nb_operators(), 1);
    assert!(lib.find("MyOp").is_some());
    assert!(lib.find("Other").is_none());
}

#[test]
fn oper_library_standard_operators() {
    let lib = OperLibrary::standard_operators();
    assert!(lib.nb_operators() >= 5);
    assert!(lib.find("DirectFaces").is_some());
    assert!(lib.find("BSplineRestriction").is_some());
}

#[test]
fn operator_perform_adds_message() {
    let mut ctx = ShapeContext::new("Shape");
    let def = OperatorDef::new("DirectFaces", "Convert indirect surfaces");
    let mut op = Operator::new(def);
    let ok = op.perform(&mut ctx);
    assert!(ok);
    assert!(op.executed);
    assert!(op.modified);
    assert!(!ctx.messages.is_empty());
}
