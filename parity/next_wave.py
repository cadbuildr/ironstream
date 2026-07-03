#!/usr/bin/env python3
"""Emit the next wave of UNPORTED OCCT classes for a toolkit, gtest-first.
Usage: next_wave.py <toolkit> <count> [out.json]  (toolkit '*' = any)"""
import json, os, re, subprocess, sys
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
wq=json.load(open(os.path.join(ROOT,"parity","work_queue.json")))["rows"]
pj=json.loads(subprocess.run(["python3",os.path.join(ROOT,"parity","parity.py"),"--json"],
    capture_output=True,text=True).stdout)
impl=set(pj["implemented"]) if isinstance(pj["implemented"],list) else set(pj["implemented"].keys())
qpath=os.path.join(ROOT,"parity","quarantine.txt")
quar=set()
if os.path.exists(qpath):
    for ln in open(qpath):
        parts=ln.split()
        if len(parts)>=2: quar.add(parts[1])

existing={f[:-3] for f in os.listdir(os.path.join(ROOT,"crates/ironstream/src")) if f.endswith(".rs")}
def snake(c):
    out=[]
    for p in c.split('_'):
        s=re.sub(r'(.)([A-Z][a-z]+)',r'\1_\2',p); s=re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',s)
        out.append(s.lower())
    return '_'.join(out)
tk=sys.argv[1]; n=int(sys.argv[2]); out=sys.argv[3] if len(sys.argv)>3 else "/tmp/wave.json"
cand=[r for r in wq if r["class"] not in impl and r["class"] not in quar and (tk=='*' or r["tk"]==tk) and r["header"]]
cand.sort(key=lambda r: (r["gtest"] is None, r["class"]))  # gtest-first
wave=[]
for r in cand[:n]:
    m=snake(r["class"])
    if m in existing or any(w["module"]==m for w in wave): m+="_occt"
    wave.append({"class":r["class"],"module":m,"header":r["header"],"gtest":r["gtest"]})
json.dump(wave,open(out,"w"))
print(f"wave: {len(wave)} classes from {tk} -> {out} (gtest-backed: {sum(1 for w in wave if w['gtest'])})")
print(json.dumps(wave))
