#!/usr/bin/env python3
"""One-shot `// occt:` marker remediation for the IronStream kernel.

Rewrites ONLY marker comment lines (never code) in `crates/ironstream/src` so
that the strict grammar in `check_markers.py` passes:

  pass A — junk demotion: payloads that are prose ("loft result", "wire")
           become `// occt-note: <text>`;
  pass B — salvage: payloads whose first token is (or renames to) a known
           class keep the claim and move the prose behind a `//` separator:
           `// occt: AIS_ColorScale position` → `// occt: AIS_ColorScale // position`;
  pass C — renames: unknown identifiers with a documented OCCT rename are
           rewritten to the manifest name. Built-in rule: `AIS_X → PrsDim_X`
           when `PrsDim_X` is in the manifest (the OCCT 7.5 AIS→PrsDim move).
           Extra pairs can be added to `parity/occt_renames.json`.
           Unknown identifiers with no rename demote to `// occt-ref: <name>`;
  pass D — duplicate resolution: each known class keeps ONE canonical file;
           all other files' claims demote to `// occt-ref: <Class>`.
           Canonical-file heuristic, in order: hand-written core module,
           normalized filename match, file with `#[cfg(test)]`, larger file,
           alphabetical.

Dry run (default) writes `parity/marker_decisions.json` for review and touches
nothing. `--apply` rewrites the files. Gate any apply on:
  cargo build -p ironstream && cargo test -p ironstream --lib
"""

import argparse
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "occt_classes.json")
RENAMES = os.path.join(HERE, "occt_renames.json")
DECISIONS = os.path.join(HERE, "marker_decisions.json")
KERNEL_SRC = os.path.normpath(os.path.join(HERE, "..", "crates", "ironstream", "src"))

MARKER_RE = re.compile(r"^(?P<prefix>.*?)//\s*occt:\s*(?P<payload>.*)$")
IDENT_RE = re.compile(r"[A-Za-z][A-Za-z0-9_]*\Z")

# Hand-written core modules always win a duplicate claim.
CORE_FILES = [
    "gp.rs", "gp2d.rs", "geom.rs", "geom2d.rs", "topods.rs",
    "brep_prim_api.rs", "brep_algo_api.rs", "brep_builder_api.rs",
    "mesh.rs", "mesh_io.rs", "bsp.rs",
]


def norm(s: str) -> str:
    """Case/underscore-insensitive comparison key ('BSplCLib' == 'bspl_clib')."""
    return re.sub(r"[^a-z0-9]", "", s.lower())


def load_renames(known: set) -> dict:
    table = {}
    if os.path.exists(RENAMES):
        with open(RENAMES, encoding="utf-8") as f:
            table.update(json.load(f))
    bad = {old: new for old, new in table.items() if new not in known}
    if bad:
        raise SystemExit(f"occt_renames.json targets not in manifest: {bad}")
    return table


def resolve_name(name: str, known: set, renames: dict):
    """Return (final_name, kind): kind in {known, renamed, unknown}."""
    if name in known:
        return name, "known"
    if name in renames:
        return renames[name], "renamed"
    if name.startswith("AIS_") and ("PrsDim_" + name[4:]) in known:
        return "PrsDim_" + name[4:], "renamed"
    return name, "unknown"


class FileScan:
    def __init__(self, path):
        self.path = path
        self.rel = os.path.basename(path)
        with open(path, encoding="utf-8") as f:
            self.text = f.read()
        self.lines = self.text.split("\n")
        self.has_test = "#[cfg(test)]" in self.text
        self.size = len(self.text)
        # lineno (1-based) -> planned disposition
        self.marks = {}


def parse_line(payload: str, known: set, renames: dict):
    """Classify one marker payload.

    Returns dict with:
      claims:  [(final_name, orig_name, kind)] — names this line claims
      note:    payload text when the whole line demotes to occt-note
      trail:   preserved comment-in-comment text (after a second `//`)
    """
    body, sep, trail = payload.partition("//")
    body, trail = body.strip(), trail.strip()
    parts = [p.strip() for p in body.split(",") if p.strip()]

    if parts and all(IDENT_RE.match(p) for p in parts):
        return {"claims": [(*resolve_name(p, known, renames)[:1], p,
                            resolve_name(p, known, renames)[1]) for p in parts],
                "note": None, "trail": trail}

    # Salvage: first whitespace token (optionally Class::method / Class.method)
    tokens = body.split()
    if tokens:
        root = re.split(r"::|\.", tokens[0])[0]
        if IDENT_RE.match(root):
            final, kind = resolve_name(root, known, renames)
            if kind in ("known", "renamed"):
                rest = body[len(tokens[0]):].strip()
                extra = tokens[0][len(root):]  # e.g. '::D1'
                prose = " ".join(x for x in (extra, rest, trail) if x)
                return {"claims": [(final, root, kind)],
                        "note": None, "trail": prose}

    return {"claims": [], "note": (body + (" // " + trail if trail else "")),
            "trail": None}


def choose_canonical(name: str, scans_by_rel: dict, claimants: list) -> str:
    """Pick the one file that keeps the `// occt: <name>` claim."""
    for core in CORE_FILES:
        if core in claimants:
            return core
    matches = [r for r in claimants if norm(os.path.splitext(r)[0]) == norm(name)]
    pool = matches or claimants
    pool = sorted(pool, key=lambda r: (
        not scans_by_rel[r].has_test,   # tested file first
        -scans_by_rel[r].size,          # then larger
        r,                              # then alphabetical
    ))
    return pool[0]


def main() -> int:
    ap = argparse.ArgumentParser(description="rewrite // occt: markers to strict form")
    ap.add_argument("--apply", action="store_true",
                    help="write changes (default: dry run + decisions log)")
    args = ap.parse_args()

    with open(MANIFEST, encoding="utf-8") as f:
        known = set(json.load(f)["classes"])
    renames = load_renames(known)

    scans = []
    for fn in sorted(os.listdir(KERNEL_SRC)):
        if fn.endswith(".rs"):
            scans.append(FileScan(os.path.join(KERNEL_SRC, fn)))
    by_rel = {s.rel: s for s in scans}

    # Phase 1: parse every marker line.
    claims_by_name: dict = {}   # final name -> {rel: [lineno,...]}
    for s in scans:
        for i, line in enumerate(s.lines, 1):
            m = MARKER_RE.match(line)
            if not m:
                continue
            info = parse_line(m.group("payload"), known, renames)
            s.marks[i] = {"m": m, **info}
            for final, _orig, kind in info["claims"]:
                if kind in ("known", "renamed"):
                    claims_by_name.setdefault(final, {}).setdefault(s.rel, []).append(i)

    # Phase 2: canonical file per multi-claimed class.
    canonical = {}
    for name, files in claims_by_name.items():
        if len(files) > 1:
            canonical[name] = choose_canonical(name, by_rel, sorted(files))

    # Phase 3: rewrite lines.
    decisions, changed_files = [], 0
    for s in scans:
        new_lines, changed = list(s.lines), False
        for i, mark in s.marks.items():
            m = mark["m"]
            prefix = m.group("prefix")
            indent = re.match(r"\s*", prefix).group(0) if prefix.strip() == "" else prefix
            keep, refs = [], []
            for final, orig, kind in mark["claims"]:
                if kind == "unknown":
                    refs.append((final, orig, "unknown_class"))
                elif len(claims_by_name.get(final, {})) > 1 and canonical[final] != s.rel:
                    refs.append((final, orig, "duplicate_claim"))
                else:
                    keep.append((final, orig, kind))
            out = []
            if mark["note"] is not None:
                out.append(f"{indent}// occt-note: {mark['note']}")
            else:
                trail = f" // {mark['trail']}" if mark["trail"] else ""
                if keep:
                    out.append(f"{indent}// occt: {', '.join(k[0] for k in keep)}{trail}")
                    trail = ""
                if refs:
                    ws = re.match(r"\s*", indent).group(0)
                    out.append(f"{ws}// occt-ref: {', '.join(r[0] for r in refs)}{trail}")
            before = s.lines[i - 1]
            after = "\n".join(out)
            if before != after:
                changed = True
                new_lines[i - 1] = after
                kinds = ([("note", None)] if mark["note"] is not None else
                         [(r[2], r[0]) for r in refs] +
                         [("renamed", k[0]) for k in keep if k[2] == "renamed"] +
                         ([("salvaged", keep[0][0])] if keep and mark["trail"] else []))
                decisions.append({
                    "file": s.rel, "line": i,
                    "kinds": sorted({k for k, _ in kinds}),
                    "names": sorted({n for _, n in kinds if n}),
                    "before": before.strip(), "after": after.strip(),
                })
        if changed:
            changed_files += 1
            if args.apply:
                with open(s.path, "w", encoding="utf-8") as f:
                    f.write("\n".join(new_lines))

    dup_summary = {n: {"canonical": canonical[n], "claimants": sorted(files)}
                   for n, files in sorted(claims_by_name.items()) if len(files) > 1}
    with open(DECISIONS, "w", encoding="utf-8") as f:
        json.dump({
            "applied": args.apply,
            "changed_lines": len(decisions),
            "changed_files": changed_files,
            "duplicates": dup_summary,
            "decisions": decisions,
        }, f, indent=1)

    from collections import Counter
    kinds = Counter(k for d in decisions for k in d["kinds"])
    print(f"{'APPLIED' if args.apply else 'DRY RUN'}: {len(decisions)} marker lines "
          f"in {changed_files} files -> {dict(sorted(kinds.items()))}")
    print(f"duplicate classes resolved: {len(dup_summary)}  "
          f"(decisions log: {os.path.relpath(DECISIONS)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
