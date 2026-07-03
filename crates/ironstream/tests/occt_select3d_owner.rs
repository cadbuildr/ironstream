extern crate ironstream;
use ironstream::select3d_owner::*;

#[test]
fn test_select3d_owner_basic() {
    // EntityOwner construction
    let owner = EntityOwner::new(42);
    assert_eq!(owner.id(), 42);
    assert_eq!(owner.priority(), 0);
    assert!(!owner.is_selected());

    // with_priority builder
    let owner2 = EntityOwner::new(1).with_priority(5);
    assert_eq!(owner2.priority(), 5);

    // set_selected
    let mut o3 = EntityOwner::new(3);
    o3.set_selected(true);
    assert!(o3.is_selected());
    o3.set_selected(false);
    assert!(!o3.is_selected());

    // has_selectable is always false (no selectable attached)
    assert!(!EntityOwner::has_selectable());

    // SelectionManager
    let mut mgr = SelectionManager::new();
    assert_eq!(mgr.selected_count(), 0);

    mgr.add(EntityOwner::new(10).with_priority(2));
    mgr.add(EntityOwner::new(20).with_priority(3));
    assert_eq!(mgr.owners.len(), 2);

    assert!(mgr.find(10).is_some());
    assert!(mgr.find(99).is_none());

    // selected_count
    let mut o_sel = EntityOwner::new(30);
    o_sel.set_selected(true);
    mgr.add(o_sel);
    assert_eq!(mgr.selected_count(), 1);

    mgr.clear();
    assert!(mgr.owners.is_empty());
}
