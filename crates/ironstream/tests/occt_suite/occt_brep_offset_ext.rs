extern crate ironstream;
use ironstream::brep_offset_ext::*;

#[test]
fn test_brep_offset_ext_basic() {
    let mut analysis = OffsetAnalysis::new("box", 0.1);
    analysis.perform();
    assert!(analysis.is_valid());
    assert_eq!(analysis.error_count(), 0);

    let mut maker = MakeOffset::new("box", 0.5)
        .with_mode(OffsetMode::Skin)
        .as_solid(true);
    let result = maker.build();
    assert!(maker.is_done());
    assert!(!result.is_empty());
}
