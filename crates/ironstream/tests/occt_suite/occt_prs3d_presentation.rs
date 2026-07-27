extern crate ironstream;
use ironstream::prs3d_presentation::*;

#[test]
fn test_prs3d_presentation_basic() {
    let mut g = PresentationGroup::new();
    assert_eq!(g.num_primitives(), 0);
    g.add_primitive("triangle");
    g.add_primitive("quad");
    assert_eq!(g.num_primitives(), 2);
    g.set_color(1.0, 0.0, 0.0);
    assert_eq!(g.color, [1.0, 0.0, 0.0]);
    g.clear();
    assert_eq!(g.num_primitives(), 0);

    let mut pres = Presentation::new();
    assert!(pres.is_empty());
    pres.new_group();
    assert_eq!(pres.num_groups(), 1);
    assert!(!pres.is_empty());
    pres.set_visible(false);
    pres.clear();
    assert!(pres.is_empty());

    let mut mgr = PrsMgr::new();
    assert_eq!(mgr.count(), 0);
    mgr.add(Presentation::new());
    mgr.add(Presentation::new());
    assert_eq!(mgr.count(), 2);
    mgr.remove(0);
    assert_eq!(mgr.count(), 1);
}
