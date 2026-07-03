export const meta = {
  name: 'occt-port-bigwave',
  description: 'Port a large wave of real OCCT classes to Rust (20 Haiku agents x 50 classes) from pinned OCCT headers',
  phases: [
    { title: 'Port', detail: '20 Haiku agents, each faithfully porting up to 50 OCCT classes from the real headers' },
  ],
}

const MIRROR = '/tmp/occt-mirror/src'
const SRC = '/home/user/monorepo/rust/ironstream/crates/ironstream/src'
const items = typeof args === 'string' ? JSON.parse(args) : args

const CHUNK = 50
const groups = []
for (let i = 0; i < items.length; i += CHUNK) groups.push(items.slice(i, i + CHUNK))

const SCHEMA = {
  type: 'object',
  properties: {
    results: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          class: { type: 'string' },
          module: { type: 'string' },
          wrote: { type: 'boolean' },
          notes: { type: 'string' },
        },
        required: ['class', 'module', 'wrote'],
      },
    },
  },
  required: ['results'],
}

phase('Port')
const out = await parallel(groups.map((g, gi) => () => {
  const list = g.map(it =>
    `- ${it.class}  | header: ${MIRROR}/${it.header}  | gtest: ${it.gtest ? MIRROR + '/' + it.gtest : 'none'}  | write: ${SRC}/${it.module}.rs`
  ).join('\n')
  const prompt = `You are doing a FAITHFUL 1:1 port of real OpenCascade (OCCT) C++ classes into pure Rust for the IronStream kernel. Zero external dependencies (std only).

You have ${g.length} classes to port. WORK THROUGH ALL OF THEM, one at a time. Do not stop early. Each is independent; if one is hard, do your best and move to the next. Keep each file small and focused so you have budget for all ${g.length}.

${list}

For EACH class:
1. Read the real OCCT header at the given absolute path (GROUND TRUTH for exact class name, method names, enums, semantics). Read its .cxx in the same dir for the algorithm if needed, and the gtest if one is listed.
2. Write a faithful Rust module at the given write path:
   - Line 1: \`// FILE: <module>.rs\`
   - Line 2: \`// occt: <ExactClassName>\` — the EXACT OCCT identifier, character-for-character (correct underscores/capitalization). This marker is matched against the OCCT inventory. Add markers for nested enums/types with their exact OCCT names too.
   - Rust struct gets the closest legal name; the exact OCCT name lives in the marker.
   - Implement REAL logic faithful to the header/.cxx (real algorithms), never stubs or fabricated return values.
   - std-only. Define tiny local helpers if needed.
   - Do NOT spawn OS threads (no std::thread / Mutex waits) — port threaded classes as sequential equivalents; threaded ports deadlock the test runner.
   - Avoid generic Fn/closure struct fields needing trait bounds threaded through impls; prefer concrete \`fn(...) -> ...\` pointers or a simple trait object.
3. Add an inline \`#[cfg(test)] mod tests { use super::*; ... }\` block. If a gtest exists, port its exact inputs/expected/tolerances. Otherwise write 2-4 small tests from the header's semantics. The test MUST compile: reference only items defined in this same file (no external crates, no other ironstream modules); verify every type/method used exists with that exact signature.

Use the Write tool for each file. Return the manifest for every class you attempted.`
  return agent(prompt, { label: `port:g${gi}`, phase: 'Port', model: 'haiku', effort: 'low' })
    .then(() => g.map(it => ({ class: it.class, module: it.module, wrote: true })))
    .catch(() => g.map(it => ({ class: it.class, module: it.module, wrote: false })))
}))

const flat = out.filter(Boolean).flat()
log(`Big wave dispatched ${groups.length} agents over ${items.length} classes`)
return { dispatched: items.length, agents: groups.length }
