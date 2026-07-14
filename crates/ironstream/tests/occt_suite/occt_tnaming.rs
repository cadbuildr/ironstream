use ironstream::tnaming::*;

#[test]
fn tnaming_evolution_is_alive() {
    assert!(TNamingEvolution::Primitive.is_alive());
    assert!(TNamingEvolution::Generated.is_alive());
    assert!(TNamingEvolution::Modify.is_alive());
    assert!(TNamingEvolution::Replace.is_alive());
    assert!(TNamingEvolution::Selected.is_alive());
    assert!(!TNamingEvolution::Delete.is_alive());
}

#[test]
fn tnaming_shape_record_constructors() {
    let prim = TNamingShapeRecord::primitive(42);
    assert_eq!(prim.new_shape_id, Some(42));
    assert!(prim.old_shape_id.is_none());
    assert_eq!(prim.evolution, TNamingEvolution::Primitive);

    let modified = TNamingShapeRecord::modified(10, 20);
    assert_eq!(modified.old_shape_id, Some(10));
    assert_eq!(modified.new_shape_id, Some(20));

    let deleted = TNamingShapeRecord::deleted(5);
    assert_eq!(deleted.old_shape_id, Some(5));
    assert!(deleted.new_shape_id.is_none());
    assert_eq!(deleted.evolution, TNamingEvolution::Delete);

    let gen = TNamingShapeRecord::generated(1, 100);
    assert_eq!(gen.evolution, TNamingEvolution::Generated);
}

#[test]
fn tnaming_builder_primitive_and_current_shape() {
    let mut b = TNamingBuilder::new(1);
    assert!(b.named_shape().is_empty());
    b.primitive(42);
    let ns = b.into_named_shape();
    assert_eq!(ns.current_shape(), Some(42));
    assert_eq!(ns.evolution, TNamingEvolution::Primitive);
    assert_eq!(ns.nb_shapes(), 1);
}

#[test]
fn tnaming_builder_modify_delete_generated() {
    let mut b = TNamingBuilder::new(2);
    b.modify(10, 20);
    assert_eq!(b.named_shape().evolution, TNamingEvolution::Modify);

    let mut b2 = TNamingBuilder::new(3);
    b2.delete(5);
    assert!(!b2.named_shape().evolution.is_alive());

    let mut b3 = TNamingBuilder::new(4);
    b3.generated(1, 100);
    assert_eq!(b3.named_shape().evolution, TNamingEvolution::Generated);
}

#[test]
fn tnaming_iterator_traversal() {
    let mut b = TNamingBuilder::new(5);
    b.generated(1, 10);
    b.generated(2, 11);
    let ns = b.into_named_shape();
    let mut it = TNamingIterator::new(&ns);
    let mut count = 0;
    while it.more() {
        assert!(it.new_shape().is_some());
        assert_eq!(it.evolution(), Some(TNamingEvolution::Generated));
        it.next();
        count += 1;
    }
    assert_eq!(count, 2);
    // past end
    assert!(!it.more());
    assert!(it.new_shape().is_none());
}

#[test]
fn tnaming_registry_insert_find_upsert() {
    let mut reg = TNamingRegistry::new();
    let mut b = TNamingBuilder::new(10);
    b.primitive(99);
    reg.insert(b.into_named_shape());
    assert_eq!(reg.nb_named_shapes(), 1);

    let ns = reg.find(10).unwrap();
    assert_eq!(ns.current_shape(), Some(99));

    // upsert: same label_id replaces
    let mut b2 = TNamingBuilder::new(10);
    b2.primitive(200);
    reg.insert(b2.into_named_shape());
    assert_eq!(reg.nb_named_shapes(), 1);
    assert_eq!(reg.find(10).unwrap().current_shape(), Some(200));

    assert!(reg.find(99).is_none());
}
