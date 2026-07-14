extern crate ironstream;
use ironstream::geom_spiral::*;

#[test]
fn occt_archimedean_smoke() {
    let s = ArchimedeanSpiral::new(0.0, 1.0, [0.0, 0.0, 0.0]);
    assert!((s.radius(0.0) - 0.0).abs() < 1e-10);
}

#[test]
fn occt_logarithmic_smoke() {
    let s = LogarithmicSpiral::new(1.0, 0.0, [0.0, 0.0, 0.0]);
    assert!((s.radius(10.0) - 1.0).abs() < 1e-10);
}

#[test]
fn occt_fermat_smoke() {
    let s = FermatSpiral::new(1.0, [0.0, 0.0, 0.0]);
    let p = s.point(1.0).unwrap();
    assert!((p[0] - 1.0_f64.cos()).abs() < 1e-10);
}
