extern crate ironstream;

use ironstream::iges_construct::{
    IgesController, IgesEntity, IgesMode, IgesModel, IgesReaderTool, IgesStatus,
};

#[test]
fn iges_model_entities_of_type() {
    let mut m = IgesModel::new();
    m.add_entity(IgesEntity::new(110, 0)); // Line
    m.add_entity(IgesEntity::new(110, 0));
    m.add_entity(IgesEntity::new(100, 0)); // Circle arc
    assert_eq!(m.nb_entities(), 3);
    assert_eq!(m.entities_of_type(110).len(), 2);
    assert_eq!(m.entities_of_type(999).len(), 0);
}

#[test]
fn iges_entity_fields() {
    let e = IgesEntity::new(116, 1);
    assert_eq!(e.type_number(), 116);
    assert_eq!(e.form_number(), 1);
    assert!(e.is_ok());
    assert_eq!(e.status, IgesStatus::Ok);
}

#[test]
fn iges_controller_read_returns_empty() {
    let ctrl = IgesController::new(IgesMode::Read);
    let model = ctrl.read_file("shape.igs");
    assert_eq!(model.nb_entities(), 0);
}

#[test]
fn iges_controller_write_mode() {
    let rw = IgesController::new(IgesMode::ReadWrite);
    let model = IgesModel::new();
    assert!(rw.write_model(&model, "out.igs"));
    let ro = IgesController::new(IgesMode::Read);
    assert!(!ro.write_model(&model, "out.igs"));
}

#[test]
fn iges_reader_tool_load() {
    let mut model = IgesModel::new();
    model.add_entity(IgesEntity::new(100, 0));
    model.add_entity(IgesEntity::new(100, 0));
    let mut tool = IgesReaderTool::new();
    tool.load(&model);
    assert_eq!(tool.nb_read, 2);
}
