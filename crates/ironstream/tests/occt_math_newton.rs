extern crate ironstream;
use ironstream::math_newton::*;

#[test]
fn test_math_newton_minimum_basic() {
    // f(x) = (x-2)^2, minimum at x=2
    let mut nm = NewtonMinimum::new(0.0);
    nm.perform(
        |x| (x - 2.0) * (x - 2.0),
        |x| 2.0 * (x - 2.0),
        |_x| 2.0,
        1e-10,
        100,
    );
    assert!(nm.is_done(), "should converge");
    assert!((nm.minimum() - 2.0).abs() < 1e-8, "minimum should be at x=2, got {}", nm.minimum());
}

#[test]
fn test_math_newton_root_basic() {
    // f(x) = x^2 - 4, root at x=2
    let mut nr = NewtonRoot::new(vec![3.0]);
    let root = nr.solve_1d(
        |x| x * x - 4.0,
        |x| 2.0 * x,
        1e-10,
        100,
    );
    assert!(root.is_some());
    assert!((root.unwrap() - 2.0).abs() < 1e-8, "root should be ~2, got {:?}", root);
}

#[test]
fn test_newton_1d_free_function() {
    // f(x) = sin(x), root near PI
    let root = newton_1d(
        |x: f64| x.sin(),
        |x: f64| x.cos(),
        3.0,
        1e-10,
    );
    assert!(root.is_some());
    let r = root.unwrap();
    assert!(r.sin().abs() < 1e-9, "sin(root) should be ~0");
}
