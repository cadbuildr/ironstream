//! Polygon-with-holes triangulation: a port of mapbox/earcut (ISC), the
//! ear-slicing algorithm with hole elimination by visible bridges, collinear /
//! duplicate filtering, local-intersection curing and diagonal splitting as
//! fallbacks. Replaces the ad-hoc keyhole + ear clip that bridged holes to the
//! nearest (not necessarily visible) outer vertex and fanned on failure.

#[derive(Clone, Copy)]
struct Node {
    i: usize,
    x: f64,
    y: f64,
    prev: usize,
    next: usize,
    steiner: bool,
}

struct Ctx<'a> {
    n: Vec<Node>,
    out: &'a mut Vec<[usize; 3]>,
}

fn area(p: &Node, q: &Node, r: &Node) -> f64 {
    (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
}

fn equals(a: &Node, b: &Node) -> bool {
    a.x == b.x && a.y == b.y
}

fn sign(v: f64) -> i32 {
    if v > 0.0 {
        1
    } else if v < 0.0 {
        -1
    } else {
        0
    }
}

fn on_segment(p: &Node, q: &Node, r: &Node) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

fn intersects(p1: &Node, q1: &Node, p2: &Node, q2: &Node) -> bool {
    let o1 = sign(area(p1, q1, p2));
    let o2 = sign(area(p1, q1, q2));
    let o3 = sign(area(p2, q2, p1));
    let o4 = sign(area(p2, q2, q1));
    (o1 != o2 && o3 != o4)
        || (o1 == 0 && on_segment(p1, p2, q1))
        || (o2 == 0 && on_segment(p1, q2, q1))
        || (o3 == 0 && on_segment(p2, p1, q2))
        || (o4 == 0 && on_segment(p2, q1, q2))
}

#[allow(clippy::too_many_arguments)]
fn point_in_triangle(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, px: f64, py: f64) -> bool {
    (cx - px) * (ay - py) >= (ax - px) * (cy - py)
        && (ax - px) * (by - py) >= (bx - px) * (ay - py)
        && (bx - px) * (cy - py) >= (cx - px) * (by - py)
}

impl<'a> Ctx<'a> {
    fn insert(&mut self, i: usize, x: f64, y: f64, last: Option<usize>) -> usize {
        let id = self.n.len();
        let mut node = Node { i, x, y, prev: id, next: id, steiner: false };
        if let Some(l) = last {
            let ln = self.n[l].next;
            node.next = ln;
            node.prev = l;
            self.n.push(node);
            self.n[ln].prev = id;
            self.n[l].next = id;
        } else {
            self.n.push(node);
        }
        id
    }

    fn remove(&mut self, p: usize) {
        let (pv, nx) = (self.n[p].prev, self.n[p].next);
        self.n[nx].prev = pv;
        self.n[pv].next = nx;
    }

    fn a3(&self, p: usize, q: usize, r: usize) -> f64 {
        area(&self.n[p], &self.n[q], &self.n[r])
    }

    fn eq(&self, a: usize, b: usize) -> bool {
        equals(&self.n[a], &self.n[b])
    }

    /// Doubly linked ring of `pts[range]`, oriented so the ring is clockwise
    /// (`clockwise`) or counter-clockwise in earcut's sense.
    fn linked_list(&mut self, pts: &[[f64; 2]], start: usize, end: usize, clockwise: bool) -> Option<usize> {
        let mut sum = 0.0;
        let mut j = end - 1;
        for i in start..end {
            sum += (pts[j][0] - pts[i][0]) * (pts[i][1] + pts[j][1]);
            j = i;
        }
        let mut last = None;
        if clockwise == (sum > 0.0) {
            for i in start..end {
                last = Some(self.insert(i, pts[i][0], pts[i][1], last));
            }
        } else {
            for i in (start..end).rev() {
                last = Some(self.insert(i, pts[i][0], pts[i][1], last));
            }
        }
        if let Some(l) = last {
            let nx = self.n[l].next;
            if self.eq(l, nx) {
                self.remove(l);
                last = Some(nx);
            }
        }
        last
    }

    fn filter_points(&mut self, start: usize, end: Option<usize>) -> usize {
        let mut end = end.unwrap_or(start);
        let mut p = start;
        loop {
            let mut again = false;
            let (pv, nx) = (self.n[p].prev, self.n[p].next);
            if !self.n[p].steiner && (self.eq(p, nx) || self.a3(pv, p, nx) == 0.0) {
                self.remove(p);
                p = pv;
                end = pv;
                if p == self.n[p].next {
                    break;
                }
                again = true;
            } else {
                p = nx;
            }
            if !(again || p != end) {
                break;
            }
        }
        end
    }

    fn is_ear(&self, ear: usize) -> bool {
        let (a, b, c) = (self.n[ear].prev, ear, self.n[ear].next);
        if self.a3(a, b, c) >= 0.0 {
            return false;
        }
        let (na, nb, nc) = (&self.n[a], &self.n[b], &self.n[c]);
        let x0 = na.x.min(nb.x).min(nc.x);
        let y0 = na.y.min(nb.y).min(nc.y);
        let x1 = na.x.max(nb.x).max(nc.x);
        let y1 = na.y.max(nb.y).max(nc.y);
        let mut p = nc.next;
        while p != a {
            let np = &self.n[p];
            if np.x >= x0
                && np.x <= x1
                && np.y >= y0
                && np.y <= y1
                && point_in_triangle(na.x, na.y, nb.x, nb.y, nc.x, nc.y, np.x, np.y)
                && self.a3(np.prev, p, np.next) >= 0.0
            {
                return false;
            }
            p = np.next;
        }
        true
    }

    fn earcut_linked(&mut self, ear: Option<usize>, pass: u8) {
        let Some(mut ear) = ear else { return };
        let mut stop = ear;
        while self.n[ear].prev != self.n[ear].next {
            let (prev, next) = (self.n[ear].prev, self.n[ear].next);
            if self.is_ear(ear) {
                self.out.push([self.n[prev].i, self.n[ear].i, self.n[next].i]);
                self.remove(ear);
                ear = self.n[next].next;
                stop = ear;
                continue;
            }
            ear = next;
            if ear == stop {
                match pass {
                    0 => {
                        let f = self.filter_points(ear, None);
                        self.earcut_linked(Some(f), 1);
                    }
                    1 => {
                        let f = self.filter_points(ear, None);
                        let e = self.cure_local_intersections(f);
                        self.earcut_linked(Some(e), 2);
                    }
                    _ => self.split_earcut(ear),
                }
                break;
            }
        }
    }

    fn locally_inside(&self, a: usize, b: usize) -> bool {
        let (pv, nx) = (self.n[a].prev, self.n[a].next);
        if self.a3(pv, a, nx) < 0.0 {
            self.a3(a, b, nx) >= 0.0 && self.a3(a, pv, b) >= 0.0
        } else {
            self.a3(a, b, pv) < 0.0 || self.a3(a, nx, b) < 0.0
        }
    }

    fn cure_local_intersections(&mut self, start: usize) -> usize {
        let mut start = start;
        let mut p = start;
        loop {
            let a = self.n[p].prev;
            let pn = self.n[p].next;
            let b = self.n[pn].next;
            if !self.eq(a, b)
                && intersects(&self.n[a], &self.n[p], &self.n[pn], &self.n[b])
                && self.locally_inside(a, b)
                && self.locally_inside(b, a)
            {
                self.out.push([self.n[a].i, self.n[p].i, self.n[b].i]);
                self.remove(p);
                self.remove(pn);
                p = b;
                start = b;
            }
            p = self.n[p].next;
            if p == start {
                break;
            }
        }
        self.filter_points(p, None)
    }

    fn intersects_polygon(&self, a: usize, b: usize) -> bool {
        let (ai, bi) = (self.n[a].i, self.n[b].i);
        let mut p = a;
        loop {
            let nx = self.n[p].next;
            if self.n[p].i != ai
                && self.n[nx].i != ai
                && self.n[p].i != bi
                && self.n[nx].i != bi
                && intersects(&self.n[p], &self.n[nx], &self.n[a], &self.n[b])
            {
                return true;
            }
            p = nx;
            if p == a {
                return false;
            }
        }
    }

    fn middle_inside(&self, a: usize, b: usize) -> bool {
        let px = (self.n[a].x + self.n[b].x) / 2.0;
        let py = (self.n[a].y + self.n[b].y) / 2.0;
        let mut inside = false;
        let mut p = a;
        loop {
            let (np, nn) = (&self.n[p], &self.n[self.n[p].next]);
            if (np.y > py) != (nn.y > py)
                && nn.y != np.y
                && px < (nn.x - np.x) * (py - np.y) / (nn.y - np.y) + np.x
            {
                inside = !inside;
            }
            p = np.next;
            if p == a {
                return inside;
            }
        }
    }

    fn is_valid_diagonal(&self, a: usize, b: usize) -> bool {
        let (na, nb) = (&self.n[a], &self.n[b]);
        self.n[na.next].i != nb.i
            && self.n[na.prev].i != nb.i
            && !self.intersects_polygon(a, b)
            && ((self.locally_inside(a, b)
                && self.locally_inside(b, a)
                && self.middle_inside(a, b)
                && (self.a3(na.prev, a, nb.prev) != 0.0 || self.a3(a, nb.prev, b) != 0.0))
                || (self.eq(a, b)
                    && self.a3(na.prev, a, na.next) > 0.0
                    && self.a3(nb.prev, b, nb.next) > 0.0))
    }

    /// Split the ring by the diagonal a-b; returns the new node b2 of the
    /// second ring (a2, b2 duplicate a and b).
    fn split_polygon(&mut self, a: usize, b: usize) -> usize {
        let (na, nb) = (self.n[a], self.n[b]);
        let a2 = self.n.len();
        self.n.push(Node { prev: a2, next: a2, steiner: false, ..na });
        let b2 = self.n.len();
        self.n.push(Node { prev: b2, next: b2, steiner: false, ..nb });
        let an = na.next;
        let bp = nb.prev;
        self.n[a].next = b;
        self.n[b].prev = a;
        self.n[a2].next = an;
        self.n[an].prev = a2;
        self.n[b2].next = a2;
        self.n[a2].prev = b2;
        self.n[bp].next = b2;
        self.n[b2].prev = bp;
        b2
    }

    fn split_earcut(&mut self, start: usize) {
        let mut a = start;
        loop {
            let mut b = self.n[self.n[a].next].next;
            while b != self.n[a].prev {
                if self.n[a].i != self.n[b].i && self.is_valid_diagonal(a, b) {
                    let c = self.split_polygon(a, b);
                    let an = self.n[a].next;
                    let a = self.filter_points(a, Some(an));
                    let cn = self.n[c].next;
                    let c = self.filter_points(c, Some(cn));
                    self.earcut_linked(Some(a), 0);
                    self.earcut_linked(Some(c), 0);
                    return;
                }
                b = self.n[b].next;
            }
            a = self.n[a].next;
            if a == start {
                return;
            }
        }
    }

    fn find_hole_bridge(&self, hole: usize, outer: usize) -> Option<usize> {
        let (hx, hy) = (self.n[hole].x, self.n[hole].y);
        let mut qx = f64::NEG_INFINITY;
        let mut m: Option<usize> = None;
        let mut p = outer;
        loop {
            let (np, nn) = (&self.n[p], &self.n[self.n[p].next]);
            if hy <= np.y && hy >= nn.y && nn.y != np.y {
                let x = np.x + (hy - np.y) * (nn.x - np.x) / (nn.y - np.y);
                if x <= hx && x > qx {
                    qx = x;
                    m = Some(if np.x < nn.x { p } else { np.next });
                    if x == hx {
                        return m;
                    }
                }
            }
            p = np.next;
            if p == outer {
                break;
            }
        }
        let mut m = m?;
        let stop = m;
        let (mx, my) = (self.n[m].x, self.n[m].y);
        let mut tan_min = f64::INFINITY;
        p = m;
        loop {
            let np = &self.n[p];
            if hx >= np.x
                && np.x >= mx
                && hx != np.x
                && point_in_triangle(
                    if hy < my { hx } else { qx },
                    hy,
                    mx,
                    my,
                    if hy < my { qx } else { hx },
                    hy,
                    np.x,
                    np.y,
                )
            {
                let tan = (hy - np.y).abs() / (hx - np.x);
                if self.locally_inside(p, hole)
                    && (tan < tan_min
                        || (tan == tan_min
                            && (np.x > self.n[m].x
                                || (np.x == self.n[m].x && self.sector_contains_sector(m, p)))))
                {
                    m = p;
                    tan_min = tan;
                }
            }
            p = self.n[p].next;
            if p == stop {
                break;
            }
        }
        Some(m)
    }

    fn sector_contains_sector(&self, m: usize, p: usize) -> bool {
        self.a3(self.n[m].prev, m, self.n[p].prev) < 0.0 && self.a3(self.n[p].next, m, self.n[m].next) < 0.0
    }

    fn eliminate_hole(&mut self, hole: usize, outer: usize) -> usize {
        let Some(bridge) = self.find_hole_bridge(hole, outer) else {
            return outer;
        };
        let rev = self.split_polygon(bridge, hole);
        let rn = self.n[rev].next;
        self.filter_points(rev, Some(rn));
        let bn = self.n[bridge].next;
        self.filter_points(bridge, Some(bn))
    }

    fn leftmost(&self, start: usize) -> usize {
        let mut p = start;
        let mut left = start;
        loop {
            let (np, nl) = (&self.n[p], &self.n[left]);
            if np.x < nl.x || (np.x == nl.x && np.y < nl.y) {
                left = p;
            }
            p = np.next;
            if p == start {
                return left;
            }
        }
    }
}

/// Triangulate a polygon given as 2D points: `pts[..hole_starts[0]]` is the
/// outer ring, each `hole_starts[k]..hole_starts[k+1]` a hole ring (either
/// winding). Returns triangles as indices into `pts`.
pub fn earcut(pts: &[[f64; 2]], hole_starts: &[usize]) -> Vec<[usize; 3]> {
    let mut out = Vec::new();
    let outer_len = hole_starts.first().copied().unwrap_or(pts.len());
    if outer_len < 3 {
        return out;
    }
    let mut cx = Ctx { n: Vec::with_capacity(pts.len() * 2), out: &mut out };
    let Some(mut outer) = cx.linked_list(pts, 0, outer_len, true) else {
        return out;
    };
    if cx.n[outer].next == cx.n[outer].prev {
        return out;
    }
    if !hole_starts.is_empty() {
        let mut queue = Vec::new();
        for k in 0..hole_starts.len() {
            let s = hole_starts[k];
            let e = hole_starts.get(k + 1).copied().unwrap_or(pts.len());
            if e <= s {
                continue;
            }
            if let Some(list) = cx.linked_list(pts, s, e, false) {
                if list == cx.n[list].next {
                    cx.n[list].steiner = true;
                }
                queue.push(cx.leftmost(list));
            }
        }
        queue.sort_by(|&a, &b| {
            let (na, nb) = (&cx.n[a], &cx.n[b]);
            na.x.partial_cmp(&nb.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(na.y.partial_cmp(&nb.y).unwrap_or(std::cmp::Ordering::Equal))
        });
        for h in queue {
            outer = cx.eliminate_hole(h, outer);
        }
    }
    cx.earcut_linked(Some(outer), 0);
    out
}

#[cfg(test)]
mod tests {
    use super::earcut;

    fn tri_area(pts: &[[f64; 2]], t: &[usize; 3]) -> f64 {
        let (a, b, c) = (pts[t[0]], pts[t[1]], pts[t[2]]);
        ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() * 0.5
    }

    #[test]
    fn square_with_square_hole() {
        let pts = [
            [0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0],
            [3.0, 3.0], [3.0, 7.0], [7.0, 7.0], [7.0, 3.0],
        ];
        let tris = earcut(&pts, &[4]);
        let a: f64 = tris.iter().map(|t| tri_area(&pts, t)).sum();
        assert!((a - 84.0).abs() < 1e-9, "area {a}");
    }

    #[test]
    fn many_holes() {
        let mut pts = vec![[0.0, 0.0], [100.0, 0.0], [100.0, 20.0], [0.0, 20.0]];
        let mut starts = vec![];
        for k in 0..9 {
            starts.push(pts.len());
            let cx = 10.0 + 10.0 * k as f64;
            for s in 0..24 {
                let t = std::f64::consts::TAU * s as f64 / 24.0;
                pts.push([cx + 3.0 * t.cos(), 10.0 + 3.0 * t.sin()]);
            }
        }
        let tris = earcut(&pts, &starts);
        let a: f64 = tris.iter().map(|t| tri_area(&pts, t)).sum();
        let hole = 0.5 * 24.0 * 9.0 * (std::f64::consts::TAU / 24.0).sin();
        assert!((a - (2000.0 - 9.0 * hole)).abs() < 1e-6, "area {a}");
    }
}
