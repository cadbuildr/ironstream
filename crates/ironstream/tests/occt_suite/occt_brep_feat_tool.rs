extern crate ironstream;
use ironstream::brep_feat_tool::*;

#[test]
fn test_brep_feat_tool_basic() {
    let mut dp = DPrism::new("base", "face", [0.0, 0.0, 1.0], 5.0);
    let result = dp.build();
    assert!(!result.is_empty());
    assert!(dp.is_done());

    let mut lf = LinearForm::new("base", "profile", [1.0, 0.0, 0.0]);
    let result2 = lf.build();
    assert!(!result2.is_empty());
    assert!(lf.is_done());

    let mut ft = FeatureTool::new("base");
    assert_eq!(ft.feature_count(), 0);
    ft.add_feature("feat1");
    ft.add_feature("feat2");
    assert_eq!(ft.feature_count(), 2);
    let built = ft.build();
    assert!(!built.is_empty());
}
