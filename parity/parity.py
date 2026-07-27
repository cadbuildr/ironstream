#!/usr/bin/env python3
"""IronStream ⇄ OpenCascade class-parity tool.

Grabs the full OCCT class inventory (from a committed manifest, refreshable from
GitHub) and the set of classes IronStream currently mirrors (discovered from
`// occt: <ClassName>` markers in the Rust source), then reports what is
implemented and what is missing — per OCCT module/toolkit.

Usage:
  python3 parity.py                      # summary coverage report
  python3 parity.py --missing TKMath     # list missing classes in a toolkit
  python3 parity.py --module ModelingData --missing
  python3 parity.py --implemented        # list what IronStream covers
  python3 parity.py --refresh            # re-fetch OCCT inventory from GitHub
  python3 parity.py --json               # machine-readable output

A class counts as implemented when a kernel source line
(crates/ironstream/src) contains `// occt: <ClassName>` (or several,
comma-separated). This keeps the mapping next to the code that reproduces the
class. Marker grammar is STRICT (see check_markers.py): identifiers only, one
claiming file per class — the tool exits non-zero on any violation, so
coverage numbers cannot be inflated by prose or duplicate markers.
Related annotations `// occt-ref:` / `// occt-note:` are never counted.
"""
import argparse
import json
import os
import sys
import urllib.request

import check_markers

HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "occt_classes.json")
OCCT_TREE_API = (
    "https://api.github.com/repos/Open-Cascade-SAS/OCCT/git/trees/master?recursive=1"
)


def refresh_manifest() -> None:
    """Re-fetch the OCCT file tree and regenerate the committed manifest."""
    print(f"fetching {OCCT_TREE_API} ...", file=sys.stderr)
    with urllib.request.urlopen(OCCT_TREE_API, timeout=60) as r:
        tree = json.load(r)
    if tree.get("truncated"):
        print("warning: OCCT tree response was truncated", file=sys.stderr)
    classes = {}
    for t in tree["tree"]:
        if t["type"] != "blob":
            continue
        p = t["path"]
        if not (p.startswith("src/") and p.endswith(".hxx")):
            continue
        parts = p.split("/")
        name = os.path.basename(p)[:-4]
        classes[name] = {
            "m": parts[1] if len(parts) > 1 else "?",
            "tk": parts[2] if len(parts) > 3 else "",
        }
    manifest = {
        "source": "https://github.com/Open-Cascade-SAS/OCCT",
        "commit": tree["sha"],
        "class_count": len(classes),
        "classes": dict(sorted(classes.items())),
    }
    with open(MANIFEST, "w", encoding="utf-8") as f:
        json.dump(manifest, f, separators=(",", ":"), ensure_ascii=False)
    print(f"wrote {MANIFEST}: {len(classes)} classes @ {tree['sha'][:10]}", file=sys.stderr)


def load_manifest() -> dict:
    with open(MANIFEST, encoding="utf-8") as f:
        return json.load(f)


def discover_implemented():
    """Map OCCT class name -> list of (file, lineno), plus strict violations."""
    claims, parse_errors = check_markers.scan()
    impl = {
        name: [(rel, ln) for rel in sorted(files) for ln in files[rel]]
        for name, files in claims.items()
    }
    viols = check_markers.violations(
        claims, parse_errors, check_markers.load_manifest_names()
    )
    return impl, viols


def main() -> int:
    ap = argparse.ArgumentParser(description="IronStream ⇄ OCCT class parity")
    ap.add_argument("--refresh", action="store_true", help="re-fetch OCCT inventory")
    ap.add_argument("--module", help="filter to an OCCT module")
    ap.add_argument("--missing", nargs="?", const="", help="list missing classes (optionally in a toolkit)")
    ap.add_argument("--implemented", action="store_true", help="list implemented classes")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    args = ap.parse_args()

    if args.refresh:
        refresh_manifest()

    manifest = load_manifest()
    classes = manifest["classes"]
    impl, viols = discover_implemented()
    impl_names = set(impl) & set(classes)
    # Strict hygiene: unknown / duplicate / unparseable markers fail the run.
    unknown = sorted(set(impl) - set(classes))

    def fail_on_violations() -> int:
        if viols:
            print(f"\nFAIL: {len(viols)} marker violation(s) — coverage above "
                  f"is not trustworthy. Run `python3 parity/check_markers.py "
                  f"--report`; remediate via parity/fix_markers.py.",
                  file=sys.stderr)
            return 1
        return 0

    def in_scope(name: str) -> bool:
        info = classes[name]
        if args.module and info["m"] != args.module:
            return False
        if args.missing not in (None, "") and info["tk"] != args.missing and args.missing not in (info["m"],):
            return False
        return True

    scoped = [n for n in classes if in_scope(n)]
    scoped_impl = [n for n in scoped if n in impl_names]

    if args.json:
        out = {
            "occt_commit": manifest["commit"],
            "occt_total": len(classes),
            "implemented": sorted(impl_names),
            "implemented_count": len(impl_names),
            "unknown_markers": unknown,
            "violations": [v["id"] for v in viols],
        }
        print(json.dumps(out, indent=2))
        return fail_on_violations()

    if args.implemented:
        for n in sorted(impl_names):
            locs = ", ".join(f"{f}:{ln}" for f, ln in impl[n][:2])
            print(f"  {n:32} {classes[n]['m']}/{classes[n]['tk']}  [{locs}]")
        print(f"\n{len(impl_names)} OCCT classes mirrored in IronStream.")
        return fail_on_violations()

    if args.missing is not None:
        missing = sorted(n for n in scoped if n not in impl_names)
        for n in missing:
            print(f"  {n:32} {classes[n]['m']}/{classes[n]['tk']}")
        label = args.module or args.missing or "all"
        print(f"\n{len(missing)} missing of {len(scoped)} OCCT classes in scope '{label}'.")
        return fail_on_violations()

    # Default: per-module coverage summary.
    import collections

    by_module = collections.defaultdict(lambda: [0, 0])
    for n, info in classes.items():
        by_module[info["m"]][1] += 1
        if n in impl_names:
            by_module[info["m"]][0] += 1
    print(f"OCCT @ {manifest['commit'][:10]}  —  {len(classes)} classes, {len(impl_names)} mirrored\n")
    print(f"{'module':28} {'done':>6} {'total':>6}  coverage")
    for m in sorted(by_module):
        done, total = by_module[m]
        bar = "#" * int(20 * done / total) if total else ""
        print(f"{m:28} {done:>6} {total:>6}  {100*done/total:5.1f}%  {bar}")
    print(f"\n{'TOTAL':28} {len(impl_names):>6} {len(classes):>6}  {100*len(impl_names)/len(classes):5.1f}%")
    return fail_on_violations()


if __name__ == "__main__":
    raise SystemExit(main())
