# IronStream ⇄ OpenCascade parity tooling

Reproducing OpenCascade means treating OCCT as the **driver**: every IronStream
class mirrors a real OCCT class, and OCCT's own unit tests are ported and must
pass. This directory holds the tooling that measures that.

## Files

- `occt_classes.json` — the full OCCT class inventory (7084 classes, generated
  from the OCCT GitHub tree; pinned to a commit for reproducibility).
- `parity.py` — the class-comparison tool: grabs all OCCT classes and all
  IronStream classes and reports what is mirrored / missing.

## Workflow (one class at a time)

1. Pick an OCCT class (e.g. `gp_Pnt`). Read its header + GTest in the OCCT repo.
2. Reproduce its API and semantics in the matching IronStream module.
3. Mark it: add `// occt: gp_Pnt` next to the Rust type/fn so the tool counts it.
4. Port its OCCT GTest faithfully into `crates/ironstream/tests/occt_*.rs`
   (same inputs, expected values, tolerances). It must pass.
5. Re-run `python3 parity/parity.py` — coverage goes up by one.

## Usage

```bash
python3 parity/parity.py                  # per-module coverage summary
python3 parity/parity.py --missing TKMath # what's still missing in a toolkit
python3 parity/parity.py --implemented    # what IronStream covers, with file:line
python3 parity/parity.py --refresh        # re-pull the OCCT inventory from GitHub
python3 parity/parity.py --json           # machine-readable
```

A class counts as implemented when a Rust source line contains
`// occt: <ClassName>` (comma-separated for several). The tool also flags
markers that don't match any real OCCT class (typo guard).

## Ported OCCT unit tests

OCCT's GoogleTest suites live under `src/**/GTests/*.cxx` (476 files). Ports so
far live in `crates/ironstream/tests/occt_*.rs` (each names the OCCT source it
mirrors):

**gp primitives**
- `occt_gp.rs` — `gp_Pnt_Test`, `gp_Dir_Test`.
- `occt_gp_prim.rs` — `gp_Lin_Test`, `gp_Circ_Test`, `gp_Pln_Test`.
- `occt_gp_mat.rs` — `gp_Mat_Test`.

**Bnd (bounding volumes)**
- `occt_bnd_box.rs` — `Bnd_Box_Test`.
- `occt_bnd_range.rs` — `Bnd_Range_Test`.
- `occt_bnd_sphere.rs` — `Bnd_Sphere_Test`.
- `occt_bnd_b3.rs` — `Bnd_B3_Test`.
- `occt_bnd_b2.rs` — `Bnd_B2_Test`.
- `occt_bnd_box2d.rs` — `Bnd_Box2d_Test`.

**Geom 3D curves**
- `occt_geom_line.rs` — `Geom_Line_Test`.
- `occt_geom_circle.rs` — `Geom_Circle_Test`.
- `occt_geom_bezier.rs` — `Geom_BezierCurve_Test`.
- `occt_geom_bspline_curve.rs` — `Geom_BSplineCurve_Test`.
- `occt_geom_bspline_surface.rs` — `Geom_BSplineSurface_Test`.

**Geom2d curves**
- `occt_geom2d_line.rs` — `Geom2d_Line_Test`.
- `occt_geom2d_circle.rs` — `Geom2d_Circle_Test`.
- `occt_geom2d_ellipse.rs` — `Geom2d_Ellipse_Test`.
- `occt_geom2d_bezier.rs` — `Geom2d_BezierCurve_Test`.
- `occt_geom2d_bspline.rs` — `Geom2d_BSplineCurve_Test`.

**math (linear algebra, root-finding)**
- `occt_math_crout.rs` — `math_Crout_Test`.
- `occt_math_gauss.rs` — `math_Gauss_Test`.
- `occt_math_householder.rs` — `math_Householder_Test`.
- `occt_math_jacobi.rs` — `math_Jacobi_Test`.
- `occt_math_poly_roots.rs` — `math_DirectPolynomialRoots_Test`.
- `occt_math_trig_roots.rs` — `math_TrigonometricFunctionRoots_Test`.

**Total: ~674 ported OCCT test cases, all green.** Produced by the
`ironstream-occt-classes` workflow — one Sonnet agent per class,
each self-verifying with `cargo test` before its module was integrated.
