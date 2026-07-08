# ironstream-wasm

Browser surface of the IronStream kernel — the kernel backbone of the CADmium rewrite
(see `docs/REWRITE.md`, Phase 1). One exported function:

```ts
compile(dagJson: string): {
  parts: { name: string; positions: number[]; indices: number[]; volume: number }[];
  errors: string[];
  unsupported: string[];
}
```

Input is a serialized CADbuildr foundation DAG; output is three.js-ready geometry
(`positions` flat xyz, `indices` flat triangles) plus exact per-part volume.

## Build

Until `ironstream-dag` merges to the ironstream repo's main and is consumable as a git
dependency, point `.ironstream` at repo root (gitignored) to an ironstream checkout that
contains `crates/ironstream-dag`:

```sh
ln -s <ironstream-checkout> ../../.ironstream   # from this directory
wasm-pack build --target web --release           # → pkg/ (~270 KB wasm)
```

`.cargo/config.toml` raises the wasm shadow stack to 64 MB: BSP CSG booleans recurse deeply
and run on the caller's stack under wasm (no threads).

## Demo / verification

```sh
python3 -m http.server 8741 --directory .   # from this directory
# open http://localhost:8741/demo/index.html
```

The demo compiles canonical monorepo fixtures (from
`tsjs/packages/cad/kernel-truck/src/__tests__/fixtures/`) and renders them with three.js.
Cross-check any fixture against the native build with:

```sh
cargo run --example native_check demo/fixtures/<name>.json
```

Volumes must match the browser output exactly (same kernel, same tessellation params).

Measured on 2026-07-06: gold_star 20 ms, cube 4 ms, donut 5 ms, push_pin 9 ms,
chess_knight 80 ms (M-series MacBook, release build).

Known kernel-side gaps (tracked for Phase 6): no face/edge IDs (no picking), no vertex
normals (demo flat-shades), full-DAG recompute (no caching yet), some fixtures compile
empty (`plate`), BSP output can carry inconsistent winding (demo renders `DoubleSide`).
