// FILE: tests/occt_brepfill_ext.rs
extern crate ironstream;
use ironstream::brepfill_ext::{
    BRepFillFilling, BRepFillGenerator, BRepFillSection, BRepFillSectionLaw,
    BRepFillSweepOptions,
};

#[test]
fn section_law_constant_debug() {
    let law = BRepFillSectionLaw::Constant;
    assert_eq!(format!("{:?}", law), "Constant");
}

#[test]
fn section_law_linear_eq() {
    assert_eq!(BRepFillSectionLaw::Linear, BRepFillSectionLaw::Linear);
    assert_ne!(BRepFillSectionLaw::Linear, BRepFillSectionLaw::EvolvedSection);
}

#[test]
fn section_new_stores_idx_and_param() {
    let s = BRepFillSection::new(7, 0.75);
    assert_eq!(s.section_idx, 7);
    assert!((s.parameter - 0.75).abs() < 1e-15);
    assert!(!s.is_punctual);
}

#[test]
fn section_set_punctual_roundtrip() {
    let mut s = BRepFillSection::new(2, 0.5);
    assert!(!s.is_punctual);
    s.set_punctual(true);
    assert!(s.is_punctual);
    s.set_punctual(false);
    assert!(!s.is_punctual);
}

#[test]
fn sweep_options_default_law_is_linear() {
    let opts = BRepFillSweepOptions::default();
    assert_eq!(opts.law, BRepFillSectionLaw::Linear);
}

#[test]
fn sweep_options_default_nb_sections_and_continuity() {
    let opts = BRepFillSweepOptions::default();
    assert_eq!(opts.nb_sections, 2);
    assert_eq!(opts.continuity, 1);
    assert!((opts.tolerance - 1e-4).abs() < 1e-15);
}

#[test]
fn filling_not_done_before_build() {
    let f = BRepFillFilling::new();
    assert!(!f.is_done());
    assert_eq!(f.nb_constraints(), 0);
}

#[test]
fn filling_two_constraints_build_not_done() {
    let mut f = BRepFillFilling::new();
    f.add_constraint();
    f.add_constraint();
    f.build();
    assert!(!f.is_done());
}

#[test]
fn filling_three_constraints_build_done() {
    let mut f = BRepFillFilling::new();
    f.add_constraint();
    f.add_constraint();
    f.add_constraint();
    f.build();
    assert!(f.is_done());
    assert_eq!(f.nb_constraints(), 3);
}

#[test]
fn filling_five_constraints_build_done() {
    let mut f = BRepFillFilling::new();
    for _ in 0..5 {
        f.add_constraint();
    }
    f.build();
    assert!(f.is_done());
}

#[test]
fn filling_g0_and_g1_error_zero_after_build() {
    let mut f = BRepFillFilling::new();
    f.add_constraint();
    f.add_constraint();
    f.add_constraint();
    f.build();
    assert_eq!(f.g0_error(), 0.0);
    assert_eq!(f.g1_error(), 0.0);
}

#[test]
fn filling_set_approx_param_updates_fields() {
    let mut f = BRepFillFilling::new();
    f.set_approx_param(6, 15, 4);
    f.add_constraint();
    f.add_constraint();
    f.add_constraint();
    f.build();
    assert!(f.is_done());
}

#[test]
fn generator_perform_with_one_section_not_done() {
    let opts = BRepFillSweepOptions::default();
    let mut gen = BRepFillGenerator::new(opts);
    gen.add_section(BRepFillSection::new(0, 0.0));
    gen.perform();
    assert!(!gen.is_done());
    assert_eq!(gen.nb_sections(), 1);
}

#[test]
fn generator_perform_with_two_sections_done() {
    let opts = BRepFillSweepOptions::default();
    let mut gen = BRepFillGenerator::new(opts);
    gen.add_section(BRepFillSection::new(0, 0.0));
    gen.add_section(BRepFillSection::new(1, 1.0));
    gen.perform();
    assert!(gen.is_done());
    assert_eq!(gen.nb_sections(), 2);
}

#[test]
fn generator_three_sections_done() {
    let opts = BRepFillSweepOptions::default();
    let mut gen = BRepFillGenerator::new(opts);
    gen.add_section(BRepFillSection::new(0, 0.0));
    gen.add_section(BRepFillSection::new(1, 0.5));
    gen.add_section(BRepFillSection::new(2, 1.0));
    gen.perform();
    assert!(gen.is_done());
    assert_eq!(gen.nb_sections(), 3);
}
