extern crate ironstream;
use ironstream::geom_array2::*;

#[test]
fn array1_pnt_indexing() {
    let mut a = Array1OfPnt::new(1, 5);
    assert_eq!(a.size(), 5);
    assert_eq!(a.lower(), 1);
    assert_eq!(a.upper(), 5);
    a.set_value(3, [7.0, 8.0, 9.0]);
    assert_eq!(a.value(3), Some([7.0, 8.0, 9.0]));
    assert_eq!(a.value(1), Some([0.0, 0.0, 0.0]));
}

#[test]
fn array1_pnt_from_vec() {
    let pts = vec![[1.0,0.0,0.0],[2.0,0.0,0.0],[3.0,0.0,0.0]];
    let a = Array1OfPnt::from_vec(pts);
    assert_eq!(a.size(), 3);
    assert_eq!(a.lower(), 1);
    assert_eq!(a.value(2), Some([2.0,0.0,0.0]));
    let collected: Vec<_> = a.iter().collect();
    assert_eq!(collected.len(), 3);
}

#[test]
fn array2_pnt_shape() {
    let mut a = Array2OfPnt::new(1, 4, 1, 3);
    assert_eq!(a.nb_rows(), 4);
    assert_eq!(a.nb_cols(), 3);
    assert_eq!(a.size(), 12);
    a.set_value(2, 2, [5.0, 5.0, 5.0]);
    assert_eq!(a.value(2, 2), Some([5.0, 5.0, 5.0]));
    assert_eq!(a.value(99, 99), None);
}

#[test]
fn array2_pnt_row_col() {
    let mut a = Array2OfPnt::new(1, 2, 1, 3);
    for c in 1..=3i32 {
        a.set_value(1, c, [c as f64, 0.0, 0.0]);
    }
    let row = a.row(1);
    assert_eq!(row.len(), 3);
    assert_eq!(row[0], [1.0, 0.0, 0.0]);
    assert_eq!(row[2], [3.0, 0.0, 0.0]);
    let col = a.col(2);
    assert_eq!(col.len(), 2);
}

#[test]
fn array1_vec_operations() {
    let mut a = Array1OfVec::new(0, 3);
    assert_eq!(a.size(), 4);
    a.set_value(2, [0.0, 1.0, 0.0]);
    assert_eq!(a.value(2), Some([0.0, 1.0, 0.0]));
    assert_eq!(a.value(0), Some([0.0, 0.0, 0.0]));
}
