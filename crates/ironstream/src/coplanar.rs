//! Post-boolean mesh cleanup: re-merge coplanar fragments.
//!
//! The BSP boolean cuts every boundary polygon along the other solid's
//! partition planes, so one input triangle often leaves as dozens of coplanar
//! slivers. OCCT's mesher works from exact faces and never has them. This pass
//! groups edge-connected coplanar triangles, walks each group's boundary,
//! drops boundary vertices that are collinear and used by no triangle outside
//! the group (so no crack can open), and re-triangulates the region with
//! earcut. Groups that cannot be re-triangulated exactly keep their triangles.

use crate::gp::Pnt;
use crate::mesh::TriMesh;
use std::collections::HashMap;

fn find(p: &mut [usize], mut x: usize) -> usize {
    while p[x] != x {
        p[x] = p[p[x]];
        x = p[x];
    }
    x
}

/// Merge coplanar fragments of a welded mesh (vertex indices must be shared).
pub fn merge_coplanar(m: &TriMesh) -> TriMesh {
    let nt = m.tris.len();
    if nt < 4 {
        return m.clone();
    }
    let bb = m.bbox();
    let scale = (0..3).map(|k| bb.max[k] - bb.min[k]).fold(1.0f64, f64::max);
    let d_tol = 1e-7 * scale;
    // per-triangle plane
    let mut planes: Vec<Option<(Pnt, f64, f64)>> = Vec::with_capacity(nt);
    for t in &m.tris {
        let (a, b, c) = (m.verts[t[0]], m.verts[t[1]], m.verts[t[2]]);
        let n = (b - a).cross(c - a);
        let area2 = n.norm();
        if area2 < 1e-14 * scale * scale {
            planes.push(None);
        } else {
            let n = n * (1.0 / area2);
            planes.push(Some((n, n.dot(a), area2)));
        }
    }
    // edge -> triangles
    let mut edge_tris: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (ti, t) in m.tris.iter().enumerate() {
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            edge_tris.entry((a.min(b), a.max(b))).or_default().push(ti);
        }
    }
    let mut parent: Vec<usize> = (0..nt).collect();
    for tris in edge_tris.values() {
        if tris.len() != 2 {
            continue;
        }
        let (i, j) = (tris[0], tris[1]);
        if let (Some((ni, di, _)), Some((nj, dj, _))) = (planes[i], planes[j]) {
            if ni.dot(nj) > 1.0 - 1e-10 && (di - dj).abs() < d_tol {
                let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
    }
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..nt {
        let r = find(&mut parent, i);
        groups.entry(r).or_default().push(i);
    }
    // vertex -> number of triangles using it
    let mut vuse = vec![0usize; m.verts.len()];
    for t in &m.tris {
        for &v in t {
            vuse[v] += 1;
        }
    }

    let mut out = TriMesh { verts: m.verts.clone(), tris: Vec::with_capacity(nt) };
    for (_, g) in groups {
        if g.len() < 3 || planes[g[0]].is_none() {
            out.tris.extend(g.iter().map(|&i| m.tris[i]));
            continue;
        }
        match retriangulate(m, &g, planes[g[0]].unwrap().0, &vuse) {
            Some(tris) if tris.len() < g.len() => out.tris.extend(tris),
            _ => out.tris.extend(g.iter().map(|&i| m.tris[i])),
        }
    }
    compact(out)
}

/// Drop vertices no triangle references any more.
fn compact(m: TriMesh) -> TriMesh {
    let mut map = vec![usize::MAX; m.verts.len()];
    let mut verts = Vec::new();
    let mut tris = Vec::with_capacity(m.tris.len());
    for t in &m.tris {
        let mut nt = [0usize; 3];
        for k in 0..3 {
            let v = t[k];
            if map[v] == usize::MAX {
                map[v] = verts.len();
                verts.push(m.verts[v]);
            }
            nt[k] = map[v];
        }
        tris.push(nt);
    }
    TriMesh { verts, tris }
}

fn retriangulate(m: &TriMesh, g: &[usize], n: Pnt, vuse: &[usize]) -> Option<Vec<[usize; 3]>> {
    // directed edges inside the group; boundary = no reverse twin
    let mut dir: HashMap<(usize, usize), usize> = HashMap::new();
    let mut own_use: HashMap<usize, usize> = HashMap::new();
    for &ti in g {
        let t = m.tris[ti];
        for k in 0..3 {
            *dir.entry((t[k], t[(k + 1) % 3])).or_insert(0) += 1;
            *own_use.entry(t[k]).or_insert(0) += 1;
        }
    }
    let mut next: HashMap<usize, usize> = HashMap::new();
    for (&(a, b), &c) in &dir {
        if c != 1 {
            return None;
        }
        if !dir.contains_key(&(b, a)) && next.insert(a, b).is_some() {
            return None; // pinch vertex: leave the group alone
        }
    }
    if next.is_empty() {
        return None;
    }
    // chain loops
    let mut loops: Vec<Vec<usize>> = Vec::new();
    let mut seen: HashMap<usize, bool> = HashMap::new();
    let mut starts: Vec<usize> = next.keys().copied().collect();
    starts.sort_unstable();
    for s in starts {
        if seen.contains_key(&s) {
            continue;
        }
        let mut lp = vec![s];
        seen.insert(s, true);
        let mut cur = next[&s];
        let mut guard = 0;
        while cur != s {
            if seen.contains_key(&cur) {
                return None;
            }
            seen.insert(cur, true);
            lp.push(cur);
            cur = *next.get(&cur)?;
            guard += 1;
            if guard > next.len() + 1 {
                return None;
            }
        }
        loops.push(lp);
    }
    // plane basis
    let u = n.any_perpendicular().normalized();
    let v = n.cross(u);
    let p2 = |i: usize| -> [f64; 2] {
        let p = m.verts[i];
        [p.dot(u), p.dot(v)]
    };
    // drop collinear boundary vertices nobody outside the group uses
    for lp in loops.iter_mut() {
        let mut changed = true;
        while changed && lp.len() > 3 {
            changed = false;
            let k = lp.len();
            for i in 0..k {
                let (a, b, c) = (lp[(i + k - 1) % k], lp[i], lp[(i + 1) % k]);
                let outside = vuse[b] - own_use.get(&b).copied().unwrap_or(0);
                if outside != 0 {
                    continue;
                }
                let (pa, pb, pc) = (p2(a), p2(b), p2(c));
                let cr = (pb[0] - pa[0]) * (pc[1] - pa[1]) - (pb[1] - pa[1]) * (pc[0] - pa[0]);
                let len = ((pc[0] - pa[0]).powi(2) + (pc[1] - pa[1]).powi(2)).sqrt();
                if cr.abs() <= 1e-9 * len * len.max(1.0) {
                    lp.remove(i);
                    changed = true;
                    break;
                }
            }
        }
    }
    // outer loop = largest |area|; the others are holes of this connected region
    let area = |lp: &[usize]| -> f64 {
        let mut s = 0.0;
        for i in 0..lp.len() {
            let (a, b) = (p2(lp[i]), p2(lp[(i + 1) % lp.len()]));
            s += a[0] * b[1] - b[0] * a[1];
        }
        0.5 * s
    };
    loops.sort_by(|a, b| area(b).abs().partial_cmp(&area(a).abs()).unwrap_or(std::cmp::Ordering::Equal));
    let mut flat: Vec<[f64; 2]> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let mut hole_starts = Vec::new();
    for (k, lp) in loops.iter().enumerate() {
        if k > 0 {
            hole_starts.push(flat.len());
        }
        for &i in lp {
            flat.push(p2(i));
            ids.push(i);
        }
    }
    let tris = crate::earcut::earcut(&flat, &hole_starts);
    // exactness guard: same area as the group
    let want: f64 = g
        .iter()
        .map(|&ti| {
            let t = m.tris[ti];
            (m.verts[t[1]] - m.verts[t[0]]).cross(m.verts[t[2]] - m.verts[t[0]]).norm() * 0.5
        })
        .sum();
    let got: f64 = tris
        .iter()
        .map(|t| {
            let (a, b, c) = (flat[t[0]], flat[t[1]], flat[t[2]]);
            ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() * 0.5
        })
        .sum();
    if (got - want).abs() > 1e-6 * want.max(1e-12) {
        return None;
    }
    Some(
        tris.into_iter()
            .map(|t| {
                let (a, b, c) = (ids[t[0]], ids[t[1]], ids[t[2]]);
                let nn = (m.verts[b] - m.verts[a]).cross(m.verts[c] - m.verts[a]);
                if nn.dot(n) >= 0.0 { [a, b, c] } else { [a, c, b] }
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_algo_api::{cut, fuse};
    use crate::brep_prim_api::{make_box, make_cylinder, MeshParams};

    #[test]
    fn merging_keeps_volume_and_shrinks_mesh() {
        let a = make_box(Pnt::new(-5.0, -5.0, 0.0), 10.0, 10.0, 10.0);
        let mut c = make_cylinder(2.0, 20.0, MeshParams::default());
        c.transform(&crate::gp::Trsf::translation(Pnt::new(0.0, 0.0, -5.0)));
        let raw = crate::bsp::subtract(a.mesh(), c.mesh()).welded(1e-6);
        let merged = merge_coplanar(&raw);
        assert!((merged.volume() - raw.volume()).abs() < 1e-6 * raw.volume());
        assert!(merged.tris.len() <= raw.tris.len());
        let u = fuse(&cut(&a, &c), &make_box(Pnt::new(4.0, -1.0, 2.0), 3.0, 2.0, 2.0));
        assert!(u.volume() > 0.0);
    }
}
