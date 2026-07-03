#!/usr/bin/env python3
"""Emit next N unported OCCT classes across ALL toolkits in dependency order."""
import json, os, re, subprocess, sys
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
wq=json.load(open(os.path.join(ROOT,"parity","work_queue.json")))["rows"]
pj=json.loads(subprocess.run(["python3",os.path.join(ROOT,"parity","parity.py"),"--json"],
    capture_output=True,text=True).stdout)
impl=set(pj["implemented"]) if isinstance(pj["implemented"],list) else set(pj["implemented"].keys())
qpath=os.path.join(ROOT,"parity","quarantine.txt")
quar=set(l.split()[1] for l in open(qpath) if len(l.split())>=2) if os.path.exists(qpath) else set()
existing={f[:-3] for f in os.listdir(os.path.join(ROOT,"crates/ironstream/src")) if f.endswith(".rs")}
# toolkit dependency priority (lower = earlier)
PRIO=["TKMath","TKernel","TKG2d","TKG3d","TKGeomBase","TKBRep","TKGeomAlgo","TKTopAlgo",
      "TKPrim","TKBO","TKBool","TKShHealing","TKHLR","TKFillet","TKOffset","TKFeat","TKMesh",
      "TKService","TKV3d","TKMeshVS","TKCAF","TKLCAF","TKVCAF","TKXCAF","TKCDF","TKStdL","TKStd",
      "TKXSBase","TKDESTEP","TKDEIGES","TKXMesh","TKRWMesh","TKBin","TKBinL","TKXml","TKXmlL"]
def pk(tk): 
    return PRIO.index(tk) if tk in PRIO else len(PRIO)
def snake(c):
    out=[]
    for p in c.split('_'):
        s=re.sub(r'(.)([A-Z][a-z]+)',r'\1_\2',p); s=re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',s)
        out.append(s.lower())
    return '_'.join(out)
n=int(sys.argv[1]); out=sys.argv[2] if len(sys.argv)>2 else "/tmp/bigwave.json"
cand=[r for r in wq if r["class"] not in impl and r["class"] not in quar and r["header"]]
cand.sort(key=lambda r:(pk(r["tk"]), r["gtest"] is None, r["class"]))
wave=[]; used=set(existing)
for r in cand[:n]:
    m=snake(r["class"])
    while m in used: m+="_o"
    used.add(m)
    wave.append({"class":r["class"],"module":m,"header":r["header"],"gtest":r["gtest"]})
json.dump(wave,open(out,"w"))
from collections import Counter
c=Counter("" for r in wave)
print(f"big wave: {len(wave)} classes -> {out}")
print("by toolkit:", dict(c.most_common(10)))
