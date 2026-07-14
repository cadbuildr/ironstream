use ironstream::expr_intrp::*;

#[test]
fn expr_numeric_eval() {
    let e = Expr::num(3.14);
    assert!((e.eval(&[]).unwrap() - 3.14).abs() < 1e-10);

    let zero = Expr::num(0.0);
    assert!((zero.eval(&[]).unwrap()).abs() < 1e-10);
}

#[test]
fn expr_sum_difference_product_division() {
    let sum = Expr::sum(Expr::num(3.0), Expr::num(4.0));
    assert!((sum.eval(&[]).unwrap() - 7.0).abs() < 1e-10);

    let diff = Expr::diff(Expr::num(10.0), Expr::num(3.0));
    assert!((diff.eval(&[]).unwrap() - 7.0).abs() < 1e-10);

    let prod = Expr::prod(Expr::num(6.0), Expr::num(7.0));
    assert!((prod.eval(&[]).unwrap() - 42.0).abs() < 1e-10);

    let div = Expr::div(Expr::num(10.0), Expr::num(4.0));
    assert!((div.eval(&[]).unwrap() - 2.5).abs() < 1e-10);
}

#[test]
fn expr_variable_and_unbound_error() {
    let e = Expr::sum(Expr::var("x"), Expr::num(1.0));
    assert!((e.eval(&[("x", 4.0)]).unwrap() - 5.0).abs() < 1e-10);

    // unbound variable should return Err
    assert!(Expr::var("z").eval(&[("x", 1.0)]).is_err());
    // division by zero
    let div_zero = Expr::div(Expr::num(1.0), Expr::num(0.0));
    assert!(div_zero.eval(&[]).is_err());
}

#[test]
fn expr_trig_sqrt_abs_log() {
    use std::f64::consts::PI;
    let s = Expr::sin(Expr::num(PI / 2.0));
    assert!((s.eval(&[]).unwrap() - 1.0).abs() < 1e-10);

    let c = Expr::cos(Expr::num(0.0));
    assert!((c.eval(&[]).unwrap() - 1.0).abs() < 1e-10);

    let sq = Expr::sqrt(Expr::num(9.0));
    assert!((sq.eval(&[]).unwrap() - 3.0).abs() < 1e-10);

    // sqrt of negative => error
    assert!(Expr::sqrt(Expr::num(-1.0)).eval(&[]).is_err());

    // log
    assert!((Expr::Log(Box::new(Expr::num(1.0))).eval(&[]).unwrap()).abs() < 1e-10);
    assert!(Expr::Log(Box::new(Expr::num(0.0))).eval(&[]).is_err());
}

#[test]
fn expr_simplify_constant_folding() {
    // Sum of two numerics -> folded
    let s = Expr::sum(Expr::num(3.0), Expr::num(4.0)).simplify();
    assert_eq!(s, Expr::Numeric(7.0));

    // Product of two numerics -> folded
    let p = Expr::prod(Expr::num(6.0), Expr::num(7.0)).simplify();
    assert_eq!(p, Expr::Numeric(42.0));

    // Product with 0 -> 0
    let pz = Expr::prod(Expr::num(0.0), Expr::var("x")).simplify();
    assert_eq!(pz, Expr::Numeric(0.0));

    let pz2 = Expr::prod(Expr::var("x"), Expr::num(0.0)).simplify();
    assert_eq!(pz2, Expr::Numeric(0.0));
}

#[test]
fn expr_contains_and_node_count() {
    let e = Expr::sum(Expr::var("x"), Expr::prod(Expr::var("y"), Expr::num(2.0)));
    assert!(e.contains("x"));
    assert!(e.contains("y"));
    assert!(!e.contains("z"));
    // 1(sum) + 1(x) + 1(prod) + 1(y) + 1(2) = 5
    assert_eq!(e.node_count(), 5);

    let vs = {
        let mut s = ExprVariableStore::new();
        s.bind("x", 3.0);
        s.bind("y", 4.0);
        s
    };
    // x + y*2 = 3 + 8 = 11
    assert!((vs.eval(&e).unwrap() - 11.0).abs() < 1e-10);
}
