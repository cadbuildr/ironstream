use ironstream::ocaf_func::*;

#[test]
fn function_execute_stub_succeeds() {
    let mut f = TFunctionFunction::new(1, 0xABCD);
    assert!(!f.is_done());
    assert_eq!(f.status, TFunctionExecutionStatus::NotExecuted);
    f.execute_stub();
    assert!(f.is_done());
    assert_eq!(f.status, TFunctionExecutionStatus::Succeeded);
}

#[test]
fn function_fail_sets_message() {
    let mut f = TFunctionFunction::new(2, 0x1234);
    f.fail("something broke");
    assert!(f.status.is_failed());
    assert_eq!(f.failure_message.as_deref(), Some("something broke"));
    assert!(!f.is_done());
}

#[test]
fn graph_node_deps_dedup_and_root_leaf() {
    let mut n = TFunctionGraphNode::new(10);
    assert!(n.is_root());
    assert!(n.is_leaf());
    n.add_previous(5);
    n.add_next(15);
    n.add_next(20);
    n.add_next(15); // duplicate
    assert_eq!(n.nb_previous(), 1);
    assert_eq!(n.nb_next(), 2);
    assert!(!n.is_root());
    assert!(!n.is_leaf());
}

#[test]
fn scope_add_and_execute_all_bfs() {
    let mut s = TFunctionScope::new();
    let i0 = s.add_function(1, 100);
    let i1 = s.add_function(2, 200);
    let i2 = s.add_function(3, 300);
    s.add_dependency(i0, i1);
    s.add_dependency(i1, i2);
    assert_eq!(s.nb_functions(), 3);
    assert_eq!(s.pending_functions().len(), 3);
    s.execute_all_stub();
    assert_eq!(s.pending_functions().len(), 0);
    assert!(s.function(i0).unwrap().is_done());
    assert!(s.function(i2).unwrap().is_done());
}

#[test]
fn iterator_visits_all_functions() {
    let mut s = TFunctionScope::new();
    s.add_function(1, 1);
    s.add_function(2, 2);
    s.add_function(3, 3);
    let mut it = TFunctionIterator::new(&s);
    let mut count = 0;
    while it.more() {
        assert!(it.current().is_some());
        count += 1;
        it.next();
    }
    assert_eq!(count, 3);
}

#[test]
fn ifunction_accessor_wraps_scope_index() {
    let mut s = TFunctionScope::new();
    let idx = s.add_function(42, 0xDEAD);
    s.function_mut(idx).unwrap().execute_stub();
    let ifc = TFunctionIFunction::new(&s, idx);
    assert!(ifc.is_done());
    assert_eq!(ifc.guid(), Some(0xDEAD));
    assert_eq!(ifc.label_id(), Some(42));
    // out-of-range index returns None
    let bad = TFunctionIFunction::new(&s, 999);
    assert!(!bad.is_done());
    assert_eq!(bad.guid(), None);
}
