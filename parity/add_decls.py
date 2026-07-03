#!/usr/bin/env python3
"""Add `pub mod <m>;` for each wave module file that exists but isn't declared.
Usage: add_decls.py <wave.json>"""
import sys, os, json
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIB=os.path.join(ROOT,"crates/ironstream/src/lib.rs")
SRC=os.path.join(ROOT,"crates/ironstream/src")
wave=json.load(open(sys.argv[1]))
s=open(LIB).read()
add=[]
for it in wave:
    m=it["module"]
    if os.path.exists(os.path.join(SRC,m+".rs")) and f"pub mod {m};\n" not in s:
        add.append(m)
# also catch extra files agents created that aren't in the wave list? skip; only declared wave mods.
if add:
    # insert after the first `pub mod` line to keep it simple
    idx=s.index("pub mod ")
    block="".join(f"pub mod {m};\n" for m in add)
    s=s[:idx]+block+s[idx:]
    open(LIB,"w").write(s)
print(f"added {len(add)} declarations")
