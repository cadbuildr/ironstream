use ironstream::stdselect_shape::*;

#[test]
fn brep_owner_select_deselect() {
    let mut o = StdSelectBRepOwner::new(1, StdSelectShapeType::Face, 5);
    assert!(!o.is_selected());
    o.select();
    assert!(o.is_selected());
    o.deselect();
    assert!(!o.is_selected());
    assert_eq!(o.priority(), 5);
}

#[test]
fn brep_owner_has_selectable() {
    let mut o = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
    assert!(!o.has_selectable());
    o.set_selectable(42);
    assert!(o.has_selectable());
    assert_eq!(o.selectable_id, 42);
}

#[test]
fn shape_type_filter() {
    let f = StdSelectShapeTypeFilter::new(StdSelectShapeType::Edge);
    let edge = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
    let face = StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0);
    assert!(f.is_ok(&edge));
    assert!(!f.is_ok(&face));
    // Any filter accepts all
    let any_f = StdSelectShapeTypeFilter::new(StdSelectShapeType::Any);
    assert!(any_f.is_ok(&face));
}

#[test]
fn edge_filter_accepts_only_edges() {
    let f = StdSelectEdgeFilter::new(false);
    let edge = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
    let face = StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0);
    assert!(f.is_ok(&edge));
    assert!(!f.is_ok(&face));
}

#[test]
fn owner_pool_selected_clear() {
    let mut pool = StdSelectOwnerPool::new();
    let mut o1 = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
    o1.select();
    pool.add(o1);
    pool.add(StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0));
    assert_eq!(pool.selected().len(), 1);
    pool.clear_selection();
    assert_eq!(pool.selected().len(), 0);
    assert_eq!(pool.nb_owners(), 2);
}

#[test]
fn owner_pool_find_by_shape() {
    let mut pool = StdSelectOwnerPool::new();
    pool.add(StdSelectBRepOwner::new(10, StdSelectShapeType::Solid, 3));
    pool.add(StdSelectBRepOwner::new(20, StdSelectShapeType::Vertex, 1));
    assert!(pool.find_by_shape(10).is_some());
    assert!(pool.find_by_shape(99).is_none());
}
