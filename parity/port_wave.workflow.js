export const meta = {
  name: 'occt-port-wave',
  description: 'Faithfully port a wave of real OCCT classes to Rust via Haiku agents reading the pinned OCCT mirror',
  phases: [{ title: 'Port' }],
}
// args: { file: "/tmp/wave.json", total: N, per: K }  (a JSON string is tolerated)
const a = typeof args === 'string' ? JSON.parse(args) : args
const FILE = a.file
const TOTAL = a.total
const PER = a.per || 4
const SRC = '/home/user/monorepo/rust/ironstream/crates/ironstream/src'
const MIR = '/tmp/occt-mirror/src'

const ranges = []
for (let s = 0; s < TOTAL; s += PER) ranges.push([s, Math.min(s + PER, TOTAL)])

phase('Port')
const res = await parallel(ranges.map(([s, e], gi) => () =>
  agent(
    `You are doing a FAITHFUL 1:1 port of real OpenCascade (OCCT) C++ classes to pure Rust for the IronStream kernel (crate: /home/user/monorepo/rust/ironstream/crates/ironstream). std-only, zero deps.

Step 1: Read the JSON file ${FILE} (an array of work items). Process ONLY items at indices ${s} through ${e - 1} inclusive.

Each item has: class (exact OCCT name), module (snake_case file stem), header (path relative to ${MIR}), gtest (path relative to ${MIR}, or null).

Step 2: For EACH of your assigned items:
  a. Read the real OCCT header at ${MIR}/<header> — this is GROUND TRUTH for the exact class name, methods, enums. Read the sibling .cxx (same path, .cxx instead of .hxx) if you need the algorithm. If gtest is non-null, read ${MIR}/<gtest> and port its assertions faithfully.
  b. Write ${SRC}/<module>.rs:
     - First line: \`// FILE: <module>.rs\`
     - A marker line with the EXACT OCCT class name, character-for-character: \`// occt: <class>\` (e.g. \`// occt: BRepFill_Draft\`). Add markers for nested enums/helper types using their exact OCCT names too. This marker is matched against the OCCT inventory, so it MUST be exact.
     - Mirror the public API faithfully (Rust-legal struct name close to the OCCT name; snake_case fn names are fine). Implement REAL logic from the header/.cxx — NEVER stubs, fabricated return values (no \`id+1000\`), or hardcoded \`true\`.
     - Add an inline \`#[cfg(test)] mod tests { use super::*; ... }\`. If a gtest existed, mirror its inputs/expected/tolerances. If not, write a derived test that genuinely exercises real behavior.
     - Self-contained, std-only, so it compiles in the crate (define tiny local point/vector helpers if needed).

HONESTY RULE: a test passes only because the code is correct, never because a value was faked. If you cannot faithfully satisfy an assertion, leave a TODO comment rather than faking it.

Use the Write tool for each file. Reply with one line per item: \`<class> -> wrote yes|no\`.`,
    { label: `port:${s}-${e}`, phase: 'Port', model: 'haiku' })
))
return { ranges: ranges.length, replies: res.filter(Boolean).length, total: TOTAL }
