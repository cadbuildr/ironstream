# IronStream

A from-scratch **CAD geometry kernel in pure Rust**, with **zero third-party
dependencies**. IronStream is an original, clean-room implementation whose public
API deliberately mirrors [OpenCascade](https://github.com/Open-Cascade-SAS/OCCT)'s
package boundary — a reader who knows OCCT will recognize the same shape — while
every line of the implementation is new Rust.

> **Status:** early and evolving. The kernel is a **tessellating** B-rep today
> (see [ARCHITECTURE.md](ARCHITECTURE.md)); an exact-geometry path is in progress
> and tracked in [OPENCASCADE_PARITY.md](OPENCASCADE_PARITY.md).

## Module boundary

| OCCT package | IronStream module | Contents |
|---|---|---|
| `gp` | `gp` | points, vectors, axes, transforms |
| `Geom` | `geom` | analytic surface/curve descriptors |
| `TopoDS` | `topods` | `Vertex` / `Wire` / `Face` / `Solid` / `Compound` |
| `BRepBuilderAPI` | `brep_builder_api` | polygon/face builders, transforms |
| `BRepPrimAPI` | `brep_prim_api` | box / cylinder / sphere / cone / torus, prism, revol |
| `BRepAlgoAPI` | `brep_algo_api` | fuse / cut / common (boolean) |
| `BRepMesh` | `mesh` | the triangle mesh + volume / bbox / area |
| `StlAPI` / STEP | `mesh_io` | binary STL and faceted STEP writers |

## Example

```rust
use ironstream::prelude::*;

// A 20mm cube minus a cylindrical hole, exported to STL.
let cube = make_box(Pnt::new(0.0, 0.0, 0.0), 20.0, 20.0, 20.0);
let drill = make_cylinder(5.0, 20.0, MeshParams::default());
let part = cut(&cube, &drill);

println!("volume = {:.1}", part.volume());
std::fs::write("part.stl", write_binary_stl(part.mesh())).unwrap();
```

## Python bindings & toolchain

The whole kernel is exposed to Python via PyO3, and a compiler turns
[CADbuildr foundation](https://github.com/cadbuildr) designs into IronStream
geometry:

| Repo | What it is |
|---|---|
| [`cadbuildr/ironstream-python`](https://github.com/cadbuildr/ironstream-python) | PyO3 bindings — the full prelude as `import ironstream`, with pyOCCT-style namespaces (`ironstream.gp`, `.prim`, `.algo`, `.io`) |
| [`cadbuildr/castiron`](https://github.com/cadbuildr/castiron) | foundation-DAG compiler — `compile(part, format="stl")`, fully offline on IronStream |

```python
import ironstream as ist

part = ist.cut(ist.make_box(ist.Pnt(0, 0, 0), 20, 20, 20),
               ist.make_cylinder(5, 20))
print(part.volume())
```

The [project page](https://cadbuildr.github.io/ironstream/) shows models forged
by IronStream in an interactive viewer — every STL on it was computed and
written by this kernel.

## Build & test

```bash
cargo build
cargo test -p ironstream --lib   # fast: inline unit tests (recommended loop)
cargo test                       # full: also builds the ported OCCT gtest suite
```

The crate carries a large corpus of OpenCascade unit tests ported to Rust under
`crates/ironstream/tests/occt_suite/` — one submodule per ported OCCT test
file, consolidated into a single `occt_suite` binary by
`parity/gen_occt_suite.py` so the kernel rlib links once. A bounded job count
is set in `.cargo/config.toml`; the `--lib` loop above skips the integration
binary for a fast iteration cycle. New top-level `tests/*.rs` files are
auto-discovered by cargo and run as their own targets.

## OpenCascade parity

IronStream treats OCCT as the reference. `parity/parity.py` compares the full
OCCT class inventory against the IronStream modules that mirror it — a class
counts as covered when a kernel source line carries a `// occt: <ClassName>`
marker, under a strict grammar (identifiers only, exactly one claiming file
per class; enforced by `parity/check_markers.py`, which CI runs) — and
reports per-package coverage:

```bash
python3 parity/parity.py            # per-package coverage summary
python3 parity/parity.py --missing TKMath
```

See [`parity/README.md`](parity/README.md).

## License

MIT — see [LICENSE](LICENSE).
