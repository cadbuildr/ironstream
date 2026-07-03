extern crate ironstream;

use ironstream::step_construct::{StepAssembly, StepStyle, StepStyles, StepTool, UnitContext};

#[test]
fn step_tool_loaded_and_empty() {
    let t = StepTool::new("PartModel");
    assert!(t.is_loaded());
    assert_eq!(t.nb_entities(), 0);
    let empty = StepTool::new("");
    assert!(!empty.is_loaded());
}

#[test]
fn step_assembly_relations() {
    let mut asm = StepAssembly::new();
    asm.add_relation("Top", "Sub1", "ref1", 1);
    asm.add_relation("Top", "Sub2", "ref2", 2);
    asm.add_relation("Sub1", "Leaf", "ref3", 1);
    assert_eq!(asm.nb_relations(), 3);
    assert_eq!(asm.children_of("Top").len(), 2);
    assert_eq!(asm.children_of("Sub1").len(), 1);
    assert_eq!(asm.children_of("Leaf").len(), 0);
}

#[test]
fn step_styles_get_and_miss() {
    let mut styles = StepStyles::new();
    styles.add_style(StepStyle::new("Edge_1", [0.0, 0.0, 1.0], 1.0));
    let s = styles.get_style("Edge_1").unwrap();
    assert_eq!(s.color_rgb, [0.0, 0.0, 1.0]);
    assert!(styles.get_style("Edge_99").is_none());
}

#[test]
fn unit_context_mm_and_inch() {
    let mm = UnitContext::mm();
    assert!((mm.to_mm(10.0) - 10.0).abs() < 1e-10);
    let inch = UnitContext::inch();
    assert!((inch.to_mm(1.0) - 25.4).abs() < 1e-10);
}
