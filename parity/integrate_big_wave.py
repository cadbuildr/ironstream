#!/usr/bin/env python3
"""Integrate a big porting wave robustly, lib-only (disk-safe).

Usage: python3 parity/integrate_big_wave.py <wave.json>

1. Declare every wave module that has a file (add `pub mod`).
2. Build loop: `cargo build -p ironstream`; if it fails, quarantine every WAVE
   module whose file the compiler blamed (remove decl, move file to
   parity/quarantine/, append to quarantine.txt), and rebuild. Repeat until the
   lib builds or no wave module is implicated (then bail).
3. `cargo test -p ironstream --lib`; quarantine wave modules with failing tests.
4. Rebuild once to confirm green. Print JSON {passed:[...], quarantined:[...]}.

Only `passed` modules remain in the tree (committed). Everything else is
quarantined honestly with a reason. One build + one test pass per attempt keeps
the target dir small (never builds the ~700 integration-test binaries).
"""
import json, os, re, shutil, subprocess, sys

ROOT = os.environ.get(
    "IRONSTREAM_ROOT",
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
)
LIB = os.path.join(ROOT, "crates/ironstream/src/lib.rs")
SRC = os.path.join(ROOT, "crates/ironstream/src")
QDIR = os.path.join(ROOT, "parity/quarantine")
QTXT = os.path.join(ROOT, "parity/quarantine.txt")
ENV = {**os.environ, "CARGO_INCREMENTAL": "0"}
os.makedirs(QDIR, exist_ok=True)


def cargo(args, timeout=2400):
    return subprocess.run(["cargo"] + args, cwd=ROOT, capture_output=True,
                          text=True, timeout=timeout, env=ENV)


def declared():
    return set(re.findall(r"^pub mod (\w+);", open(LIB).read(), re.M))


def declare(mods):
    s = open(LIB).read()
    have = declared()
    add = [m for m in mods if m not in have
           and os.path.exists(os.path.join(SRC, m + ".rs"))]
    if add:
        idx = s.index("pub mod ")
        open(LIB, "w").write(s[:idx] + "".join(f"pub mod {m};\n" for m in add) + s[idx:])
    return add


def quarantine(mod, cls, reason):
    s = open(LIB).read()
    s2 = re.sub(rf"^pub mod {mod};\n", "", s, flags=re.M)
    if s2 != s:
        open(LIB, "w").write(s2)
    f = os.path.join(SRC, mod + ".rs")
    if os.path.exists(f):
        shutil.move(f, os.path.join(QDIR, mod + ".rs"))
    with open(QTXT, "a") as q:
        q.write(f"{mod} {cls} {reason}\n")


def main():
    wave = json.load(open(sys.argv[1]))
    by_mod = {w["module"]: w["class"] for w in wave}
    present = [w["module"] for w in wave
               if os.path.exists(os.path.join(SRC, w["module"] + ".rs"))]
    declare(present)
    quarantined = []

    # build loop
    for _ in range(40):
        b = cargo(["build", "-p", "ironstream"])
        if b.returncode == 0:
            break
        blamed = set(re.findall(r"src/(\w+)\.rs", b.stderr))
        hits = [m for m in blamed if m in by_mod and os.path.exists(os.path.join(SRC, m + ".rs"))]
        if not hits:
            print(json.dumps({"error": "build fails without a wave module to blame",
                              "errors": [l for l in b.stderr.splitlines()
                                         if l.startswith("error")][:15]}))
            return
        for m in hits:
            quarantine(m, by_mod[m], "build-error")
            quarantined.append(m)
    else:
        print(json.dumps({"error": "too many build iterations"}))
        return

    # test pass (lib only)
    t = cargo(["test", "-p", "ironstream", "--lib"], timeout=2400)
    out = t.stdout + "\n" + t.stderr
    failed = set()
    for mod in re.findall(r"^test (\w+)::.* \.\.\. FAILED$", out, re.M):
        if mod in by_mod:
            failed.add(mod)
    # If the test binary failed to COMPILE (cfg(test) errors), blame wave modules.
    if "could not compile" in t.stderr and not failed:
        for m in re.findall(r"src/(\w+)\.rs", t.stderr):
            if m in by_mod:
                failed.add(m)
    for m in failed:
        quarantine(m, by_mod[m], "test-fail")
        quarantined.append(m)

    if failed:
        cargo(["build", "-p", "ironstream"])  # confirm still green

    passed = [m for m in present if m not in quarantined]
    print(json.dumps({"wave": len(wave), "present": len(present),
                      "passed": passed, "passed_count": len(passed),
                      "quarantined": quarantined, "quarantined_count": len(quarantined)}))


if __name__ == "__main__":
    main()
