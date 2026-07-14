#!/usr/bin/env python3
r"""Structural guardrails for the IronStream repo — fast, stdlib-only, run by CI.

Each check is one function; all failures are collected and reported together.
Exit 0 = clean, exit 1 = at least one guardrail violated.

Guardrails (hard failures):
  1. zero-dep kernel      — crates/ironstream has no [dependencies]/[build-dependencies]
  2. leakage deny-list    — kernel src imports only crate/std/core/alloc/self/super
                            (or local CamelCase items); no CADBuildr-stack tokens
  3. filename lint        — kernel src files are ^[a-z0-9_]+\.rs$ (no spaces, ever again)
  4. module bijection     — every src file has a `pub mod`, every `pub mod` a file
  5. tests layout         — every tests/occt_suite/*.rs is wired into main.rs or
                            excluded with a reason; top-level tests/ stays small
  6. marker hygiene       — delegates to check_markers.py --check

Metrics (reported, never failing): unsafe-file count, local primitive
re-definitions (struct Pnt / fn cross / fn dot / EPS consts outside gp).
These should only ratchet down; see the audit remediation plan.
"""

import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, ".."))
KERNEL = os.path.join(ROOT, "crates", "ironstream")
SRC = os.path.join(KERNEL, "src")
TESTS = os.path.join(KERNEL, "tests")
SUITE = os.path.join(TESTS, "occt_suite")

FAILURES: list = []


def fail(check: str, msg: str) -> None:
    FAILURES.append(f"[{check}] {msg}")


def src_files():
    return sorted(f for f in os.listdir(SRC) if f.endswith(".rs"))


def read(path: str) -> str:
    with open(path, encoding="utf-8", errors="replace") as f:
        return f.read()


# -- 1. zero-dep kernel -------------------------------------------------------

def check_zero_dep() -> None:
    text = read(os.path.join(KERNEL, "Cargo.toml"))
    section = None
    for line in text.splitlines():
        stripped = line.split("#", 1)[0].strip()
        if not stripped:
            continue
        if stripped.startswith("["):
            section = stripped
            continue
        if section in ("[dependencies]", "[build-dependencies]"):
            fail("zero-dep", f"kernel Cargo.toml {section} entry: `{stripped}`")


# -- 2. leakage deny-list -----------------------------------------------------

ALLOWED_ROOTS = {"crate", "std", "core", "alloc", "self", "super"}
USE_RE = re.compile(r"^\s*(?:pub\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)")
EXTERN_RE = re.compile(r"^\s*extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)")
DENY = [
    (re.compile(r"\bserde\b"), {}),
    (re.compile(r"\bcastiron\b"), {}),
    (re.compile(r"\bwasm_bindgen\b"), {}),
    (re.compile(r"\bironstream[_-]dag\b"), {}),
    (re.compile(r"use\s+foundation|extern\s+crate\s+foundation|\bfoundation::"), {}),
    # test URL in the ported VRML anchor tests is benign
    (re.compile(r"\bcadbuildr\b"), {"vrml_www_anchor.rs"}),
]


def check_leakage() -> None:
    for fn in src_files():
        text = read(os.path.join(SRC, fn))
        for i, line in enumerate(text.splitlines(), 1):
            m = USE_RE.match(line) or EXTERN_RE.match(line)
            if m:
                root = m.group(1)
                # lowercase roots must be std-ish; CamelCase = local item import
                if root not in ALLOWED_ROOTS and not root[0].isupper():
                    fail("leakage", f"{fn}:{i} import root `{root}`: {line.strip()}")
        for pattern, allow in DENY:
            if fn in allow:
                continue
            m = pattern.search(text)
            if m:
                fail("leakage", f"{fn}: denied token `{m.group(0)}`")


# -- 3 + 4. filenames and module bijection ------------------------------------

FILENAME_RE = re.compile(r"^[a-z0-9_]+\.rs$")
MOD_RE = re.compile(r"^\s*(?:pub\s+)?mod\s+([a-z0-9_]+)\s*;", re.M)


def check_files_and_mods() -> None:
    files = src_files()
    for fn in files:
        if not FILENAME_RE.match(fn):
            fail("filename", f"src/{fn!r} violates ^[a-z0-9_]+\\.rs$")
    declared = set(MOD_RE.findall(read(os.path.join(SRC, "lib.rs"))))
    on_disk = {fn[:-3] for fn in files if fn != "lib.rs"}
    for missing in sorted(on_disk - declared):
        fail("bijection", f"src/{missing}.rs exists but lib.rs declares no `pub mod {missing};`")
    for ghost in sorted(declared - on_disk):
        fail("bijection", f"lib.rs declares `mod {ghost};` but src/{ghost}.rs is missing")


# -- 5. tests layout -----------------------------------------------------------

TOP_LEVEL_TEST_BUDGET = 10


def check_tests_layout() -> None:
    exclusions = set()
    excl_path = os.path.join(HERE, "suite_exclusions.txt")
    if os.path.exists(excl_path):
        for line in read(excl_path).splitlines():
            line = line.strip()
            if line and not line.startswith("#"):
                exclusions.add(line.split()[0])

    main_rs = os.path.join(SUITE, "main.rs")
    if not os.path.exists(main_rs):
        fail("tests-layout", "tests/occt_suite/main.rs missing")
        return
    wired = set(MOD_RE.findall(read(main_rs)))
    members = {f[:-3] for f in os.listdir(SUITE)
               if f.endswith(".rs") and f != "main.rs"}
    for dead in sorted(members - wired - exclusions):
        fail("tests-layout",
             f"tests/occt_suite/{dead}.rs is neither wired into main.rs nor "
             f"listed in parity/suite_exclusions.txt (dead test)")
    for ghost in sorted(wired - members):
        fail("tests-layout", f"main.rs declares `mod {ghost};` with no file")

    top = [f for f in os.listdir(TESTS) if f.endswith(".rs")]
    if len(top) > TOP_LEVEL_TEST_BUDGET:
        fail("tests-layout",
             f"{len(top)} top-level tests/*.rs files (> {TOP_LEVEL_TEST_BUDGET}); "
             f"fold them into tests/occt_suite/ via parity/gen_occt_suite.py")


# -- 6. marker hygiene ---------------------------------------------------------

def check_markers() -> None:
    r = subprocess.run(
        [sys.executable, os.path.join(HERE, "check_markers.py"), "--check"],
        capture_output=True, text=True,
    )
    if r.returncode != 0:
        fail("markers", (r.stderr.strip() or r.stdout.strip()).splitlines()[-1])


# -- metrics (never fail) ------------------------------------------------------

def metrics() -> None:
    unsafe_files = local_pnt = local_cross = local_dot = local_eps = 0
    for fn in src_files():
        text = read(os.path.join(SRC, fn))
        if "unsafe " in text:
            unsafe_files += 1
        if fn not in ("gp.rs", "gp2d.rs"):
            local_pnt += bool(re.search(r"\bstruct Pnt\b", text))
            local_cross += bool(re.search(r"\bfn cross\(", text))
            local_dot += bool(re.search(r"\bfn dot\(", text))
            local_eps += bool(re.search(r"\bconst (EPS|TOLERANCE|RESOLUTION)\b", text))
    print(f"metrics (ratchet-down, informational): unsafe_files={unsafe_files} "
          f"local_struct_pnt={local_pnt} local_fn_cross={local_cross} "
          f"local_fn_dot={local_dot} local_eps_consts={local_eps}")


def main() -> int:
    check_zero_dep()
    check_leakage()
    check_files_and_mods()
    check_tests_layout()
    check_markers()
    metrics()
    if FAILURES:
        print(f"\n{len(FAILURES)} guardrail violation(s):", file=sys.stderr)
        for f in FAILURES[:50]:
            print(f"  {f}", file=sys.stderr)
        if len(FAILURES) > 50:
            print(f"  … and {len(FAILURES) - 50} more", file=sys.stderr)
        return 1
    print("guardrails: all clean")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
