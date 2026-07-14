extern crate ironstream;
use ironstream::geom_array_tools::*;

#[test]
fn test_geom_array_tools_basic() {
    // Array1OfPnt
    let mut arr = Array1OfPnt::new(1, 3);
    assert_eq!(arr.length(), 3);
    assert!(!arr.is_empty());
    arr.set_value(1, [1.0, 2.0, 3.0]);
    arr.set_value(2, [4.0, 5.0, 6.0]);
    arr.set_value(3, [7.0, 8.0, 9.0]);
    assert_eq!(arr.value(1), [1.0, 2.0, 3.0]);
    assert_eq!(arr.value(3), [7.0, 8.0, 9.0]);

    // Array1OfReal
    let mut rarr = Array1OfReal::new(0, 4);
    assert_eq!(rarr.length(), 5);
    rarr.set_value(0, 1.5);
    rarr.set_value(4, 9.5);
    assert!((rarr.value(0) - 1.5).abs() < 1e-12);
    assert!((rarr.value(4) - 9.5).abs() < 1e-12);

    // Array2OfPnt
    let mut arr2 = Array2OfPnt::new(2, 3);
    assert_eq!(arr2.rows, 2);
    assert_eq!(arr2.cols, 3);
    arr2.set_value(0, 0, [1.0, 0.0, 0.0]);
    arr2.set_value(1, 2, [0.0, 1.0, 0.0]);
    assert_eq!(arr2.value(0, 0), [1.0, 0.0, 0.0]);
    assert_eq!(arr2.value(1, 2), [0.0, 1.0, 0.0]);
}
