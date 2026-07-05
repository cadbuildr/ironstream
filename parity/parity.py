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

A class counts as implemented when any Rust source line contains
`// occt: <ClassName>` (or several, comma-separated). This keeps the mapping
next to the code that reproduces the class.
"""
import argparse
import json
import os
import re
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "occt_classes.json")
SRC_ROOT = os.path.normpath(os.path.join(HERE, "..", "crates"))
OCCT_TREE_API = (
    "https://api.github.com/repos/Open-Cascade-SAS/OCCT/git/trees/master?recursive=1"
)

MARKER = re.compile(r"//\s*occt:\s*([A-Za-z0-9_.,\s]+)")


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


def discover_implemented() -> dict:
    """Map OCCT class name -> list of (file, lineno) where it is marked."""
    impl: dict[str, list] = {}
    for root, _dirs, files in os.walk(SRC_ROOT):
        for fn in files:
            if not fn.endswith(".rs"):
                continue
            path = os.path.join(root, fn)
            with open(path, encoding="utf-8", errors="replace") as f:
                for i, line in enumerate(f, 1):
                    m = MARKER.search(line)
                    if not m:
                        continue
                    for name in m.group(1).split(","):
                        name = name.strip()
                        if name:
                            impl.setdefault(name, []).append(
                                (os.path.relpath(path, SRC_ROOT), i)
                            )
    return impl


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
    impl = discover_implemented()
    impl_names = set(impl) & set(classes)
    # Markers that don't match any OCCT class are surfaced as a hygiene check.
    unknown = sorted(set(impl) - set(classes))

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
        }
        print(json.dumps(out, indent=2))
        return 0

    if args.implemented:
        for n in sorted(impl_names):
            locs = ", ".join(f"{f}:{ln}" for f, ln in impl[n][:2])
            print(f"  {n:32} {classes[n]['m']}/{classes[n]['tk']}  [{locs}]")
        print(f"\n{len(impl_names)} OCCT classes mirrored in IronStream.")
        if unknown:
            print(f"warning: {len(unknown)} markers don't match any OCCT class: {unknown}")
        return 0

    if args.missing is not None:
        missing = sorted(n for n in scoped if n not in impl_names)
        for n in missing:
            print(f"  {n:32} {classes[n]['m']}/{classes[n]['tk']}")
        label = args.module or args.missing or "all"
        print(f"\n{len(missing)} missing of {len(scoped)} OCCT classes in scope '{label}'.")
        return 0

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
    if unknown:
        print(f"\nwarning: {len(unknown)} unrecognized occt: markers: {unknown}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
