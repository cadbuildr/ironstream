#!/usr/bin/env python3
"""Bulk-quarantine modules: quarantine_mods.py <reason> <mod1> <mod2> ..."""
import sys, os
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIB=os.path.join(ROOT,"crates/ironstream/src/lib.rs")
SRC=os.path.join(ROOT,"crates/ironstream/src")
QUAR=os.path.join(ROOT,"parity/quarantine.txt")
reason=sys.argv[1]; mods=sys.argv[2:]
s=open(LIB).read()
# map module -> class name from its // occt: marker (line 2) if file exists
def cls_of(m):
    p=os.path.join(SRC,m+".rs")
    if os.path.exists(p):
        for ln in open(p).read().splitlines()[:8]:
            if ln.startswith("// occt:"): return ln.split(":",1)[1].strip().split(",")[0].strip()
    return m
with open(QUAR,"a") as q:
    for m in mods:
        s=s.replace(f"pub mod {m};\n","")
        p=os.path.join(SRC,m+".rs")
        c=cls_of(m)
        if os.path.exists(p): os.remove(p)
        q.write(f"{m} {c} {reason}\n")
open(LIB,"w").write(s)
print(f"quarantined {len(mods)} modules: {reason}")
