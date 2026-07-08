# ironstream-dag (integration crate)

## Summary

Integration layer between CADbuildr's kernel-agnostic foundation DAG and the
`ironstream` kernel. Walks a serialized `CompilerInputDAG` node-by-node (the same
processing model as the replicad/truck adapters), resolves each node's deps,
emits IronStream solids, and combines per-part operations (fuse additive, cut
subtractive). Also provides bbox + voxel IOU metrics.

## Tags

cad, rust, kernel, dag, iou

## Status

green

## Guidelines

- Dispatch on the node **type name** (resolved per-DAG via `serializableNodes`),
  never the raw type id.
- Keep `tests/iou_parity.rs` as the end-to-end regression bar.

## Dependencies

### Upstream

- `ironstream`, `serde`, `serde_json`

### Downstream

- `kernel-ironstream-native`, `@buildr/kernel-ironstream` (via the binary)
