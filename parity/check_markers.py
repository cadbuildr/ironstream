#!/usr/bin/env python3
"""Strict `// occt:` marker checker for the IronStream kernel.

Single arbiter for marker hygiene — run by CI (`--check`), imported by
`parity.py`, and reused by `fix_markers.py`. Rules:

  - grammar: `// occt:` is followed by comma-separated OCCT class identifiers
    matching `[A-Za-z][A-Za-z0-9_]*`; anything else on the marker payload is a
    parse error (prose, dots, stray punctuation). A second `//` on the line
    ends the payload, so `// occt: AIS_ColorScale // position note` is valid;
  - scope: coverage markers live in kernel src only (`crates/ironstream/src`);
  - known: every claimed identifier must exist in `parity/occt_classes.json`;
  - unique: a class is claimed by at most ONE file (repeated markers inside a
    single file collapse to one claim).

Non-claim annotations (ignored here, never counted as coverage):
  `// occt-ref: <Name>`  — file relates to a class canonically claimed elsewhere
  `// occt-note: <text>` — free-form porting note

Usage:
  python3 parity/check_markers.py --report                       # human summary
  python3 parity/check_markers.py --json                         # machine output
  python3 parity/check_markers.py --check                        # exit 1 on violations
  python3 parity/check_markers.py --check --baseline parity/marker_baseline.json
  python3 parity/check_markers.py --write-baseline parity/marker_baseline.json
"""

import argparse
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "occt_classes.json")
KERNEL_SRC = os.path.normpath(os.path.join(HERE, "..", "crates", "ironstream", "src"))

# A coverage marker anywhere on a line. `occt-ref:` / `occt-note:` do not match.
MARKER_RE = re.compile(r"//\s*occt:\s*(.*)$")
IDENT_RE = re.compile(r"[A-Za-z][A-Za-z0-9_]*\Z")


def load_manifest_names() -> set:
    with open(MANIFEST, encoding="utf-8") as f:
        return set(json.load(f)["classes"])


def parse_payload(payload: str):
    """Split a marker payload into identifiers.

    Returns (names, ok). A trailing `// ...` comment-in-comment is allowed and
    ignored. ok=False when any segment is not a bare identifier.
    """
    payload = payload.split("//", 1)[0].strip()
    if not payload:
        return [], False
    names, ok = [], True
    for part in payload.split(","):
        part = part.strip()
        if IDENT_RE.match(part):
            names.append(part)
        else:
            ok = False
    return names, ok


def scan(src_root: str = KERNEL_SRC):
    """Scan kernel src for coverage markers.

    Returns (claims, parse_errors):
      claims:       {class_name: {relpath: [line, ...]}}
      parse_errors: [(relpath, line, raw_payload)]
    """
    claims: dict = {}
    parse_errors = []
    for root, _dirs, files in os.walk(src_root):
        for fn in sorted(files):
            if not fn.endswith(".rs"):
                continue
            path = os.path.join(root, fn)
            rel = os.path.relpath(path, src_root)
            with open(path, encoding="utf-8", errors="replace") as f:
                for i, line in enumerate(f, 1):
                    m = MARKER_RE.search(line)
                    if not m:
                        continue
                    names, ok = parse_payload(m.group(1))
                    if not ok:
                        parse_errors.append((rel, i, m.group(1).rstrip()))
                    for name in names:
                        claims.setdefault(name, {}).setdefault(rel, []).append(i)
    return claims, parse_errors


def violations(claims, parse_errors, known: set):
    """Flatten scan results into a stable, baseline-able violation list.

    Each violation is a dict with a stable `id` string:
      parse:<file>:<line>   payload not comma-separated identifiers
      unknown:<name>        claimed identifier not in the OCCT manifest
      duplicate:<name>      class claimed by more than one file
    """
    out = []
    for rel, line, payload in parse_errors:
        out.append({
            "id": f"parse:{rel}:{line}",
            "kind": "parse_error",
            "file": rel,
            "line": line,
            "payload": payload,
        })
    for name in sorted(claims):
        files = sorted(claims[name])
        if name not in known:
            out.append({
                "id": f"unknown:{name}",
                "kind": "unknown_class",
                "name": name,
                "files": files,
            })
        if len(files) > 1:
            out.append({
                "id": f"duplicate:{name}",
                "kind": "duplicate_claim",
                "name": name,
                "files": files,
            })
    return out


def load_baseline(path: str) -> set:
    if not path or not os.path.exists(path):
        return set()
    with open(path, encoding="utf-8") as f:
        return set(json.load(f)["ignored"])


def main() -> int:
    ap = argparse.ArgumentParser(description="strict // occt: marker checker")
    ap.add_argument("--report", action="store_true", help="human-readable summary")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    ap.add_argument("--check", action="store_true", help="exit 1 on violations")
    ap.add_argument("--baseline", help="baseline JSON of violation ids to ignore")
    ap.add_argument("--write-baseline", metavar="PATH",
                    help="write current violations as the baseline and exit")
    args = ap.parse_args()

    known = load_manifest_names()
    claims, parse_errors = scan()
    viols = violations(claims, parse_errors, known)

    if args.write_baseline:
        with open(args.write_baseline, "w", encoding="utf-8") as f:
            json.dump({"ignored": sorted(v["id"] for v in viols)}, f, indent=1)
        print(f"wrote {args.write_baseline}: {len(viols)} violations baselined")
        return 0

    baseline = load_baseline(args.baseline)
    fresh = [v for v in viols if v["id"] not in baseline]
    claimed_known = sorted(n for n in claims if n in known)

    if args.json:
        print(json.dumps({
            "claimed_known_classes": len(claimed_known),
            "violations": viols,
            "fresh_violations": [v["id"] for v in fresh],
            "baselined": len(viols) - len(fresh),
        }, indent=2))
    else:
        by_kind: dict = {}
        for v in viols:
            by_kind.setdefault(v["kind"], []).append(v)
        print(f"claimed OCCT classes (known, unique-or-not): {len(claimed_known)}")
        for kind in ("parse_error", "unknown_class", "duplicate_claim"):
            vs = by_kind.get(kind, [])
            print(f"{kind:16} {len(vs)}")
            if args.report:
                for v in vs[:40]:
                    if kind == "parse_error":
                        print(f"    {v['file']}:{v['line']}  `{v['payload']}`")
                    else:
                        print(f"    {v['name']}  ({', '.join(v['files'][:6])}"
                              f"{', …' if len(v['files']) > 6 else ''})")
                if len(vs) > 40:
                    print(f"    … and {len(vs) - 40} more")
        if baseline:
            print(f"baselined: {len(viols) - len(fresh)}  fresh: {len(fresh)}")

    if args.check and fresh:
        print(f"\nFAIL: {len(fresh)} marker violation(s) beyond baseline. "
              f"Run `python3 parity/check_markers.py --report` for details; "
              f"see parity/fix_markers.py to remediate.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
