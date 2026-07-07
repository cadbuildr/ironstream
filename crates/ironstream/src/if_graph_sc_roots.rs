// FILE: if_graph_sc_roots.rs
// occt: IFGraph_SCRoots

//! Port of OCCT IFGraph_SCRoots: determines the strong components of a
//! (sub-)graph which are roots, i.e. which are not shared (referenced) by
//! any entity outside of themselves.
//!
//! External plumbing (Interface_Graph / Interface_Model) is modeled by a
//! small local `InterfaceGraph` helper: entities are numbered 1..=n and
//! carry "shared" (outgoing reference) edges. The SCRoots behavior itself
//! is real: strong components are computed (Tarjan), then only the
//! components with no incoming reference from another loaded component
//! are kept as parts, matching IFGraph_SCRoots::Evaluate().

/// Local model of Interface_Graph: entities numbered 1..=n, each entity
/// listing the entities it shares (references).
#[derive(Clone, Debug, Default)]
pub struct InterfaceGraph {
    /// shared[i] lists the entities referenced by entity (i+1)
    shared: Vec<Vec<usize>>,
}

impl InterfaceGraph {
    /// Creates a graph with `nb` entities and no references.
    pub fn new(nb: usize) -> Self {
        InterfaceGraph {
            shared: vec![Vec::new(); nb],
        }
    }

    /// Number of entities in the model.
    pub fn nb_entities(&self) -> usize {
        self.shared.len()
    }

    /// Declares that entity `from` shares (references) entity `to` (1-indexed).
    pub fn add_shared(&mut self, from: usize, to: usize) {
        assert!(from >= 1 && from <= self.shared.len(), "bad entity number");
        assert!(to >= 1 && to <= self.shared.len(), "bad entity number");
        self.shared[from - 1].push(to);
    }

    /// Entities shared by entity `num` (1-indexed).
    pub fn shareds(&self, num: usize) -> &[usize] {
        &self.shared[num - 1]
    }
}

/// Determines strong components in a graph which are roots.
/// A root strong component is one which is not referenced from outside itself.
#[derive(Clone, Debug)]
pub struct IfGraphScRoots {
    graph: InterfaceGraph,
    loaded: Vec<usize>,
    /// parts computed by evaluate(); each part is one root strong component
    parts: Vec<Vec<usize>>,
    /// iteration cursor over parts (0-based)
    cursor: usize,
}

impl IfGraphScRoots {
    /// Creates with a Graph, and will analyze:
    /// whole = true: all the contents of the Model
    /// whole = false: sub-parts which will be given later (get_from_*)
    pub fn new(graph: InterfaceGraph, whole: bool) -> Self {
        let loaded = if whole {
            (1..=graph.nb_entities()).collect()
        } else {
            Vec::new()
        };
        IfGraphScRoots {
            graph,
            loaded,
            parts: Vec::new(),
            cursor: 0,
        }
    }

    /// Adds an entity (1-indexed) to the loaded set.
    /// If `shared` is true, all entities it shares (transitively) are added too.
    pub fn get_from_entity(&mut self, entity_num: usize, shared: bool) {
        if !self.loaded.contains(&entity_num) {
            self.loaded.push(entity_num);
        }
        if shared {
            let mut stack = self.graph.shareds(entity_num).to_vec();
            while let Some(next) = stack.pop() {
                if !self.loaded.contains(&next) {
                    self.loaded.push(next);
                    stack.extend_from_slice(self.graph.shareds(next));
                }
            }
        }
    }

    /// Adds a list of entities (without their shared ones).
    pub fn get_from_iter(&mut self, entities: &[usize]) {
        for &e in entities {
            self.get_from_entity(e, false);
        }
    }

    /// Returns entities which were loaded.
    pub fn loaded(&self) -> Vec<usize> {
        self.loaded.clone()
    }

    /// Resets loaded entities and computed parts.
    pub fn reset(&mut self) {
        self.loaded.clear();
        self.parts.clear();
        self.cursor = 0;
    }

    /// Does the computation: strong components of the loaded sub-graph,
    /// keeping only those which are roots (no incoming reference from
    /// another loaded component).
    pub fn evaluate(&mut self) {
        self.parts.clear();
        self.cursor = 0;

        // --- Tarjan strong components over the loaded sub-graph ---
        let nodes: Vec<usize> = self.loaded.clone();
        let index_of = |num: usize| nodes.iter().position(|&n| n == num);

        let n = nodes.len();
        let mut index = vec![usize::MAX; n];
        let mut lowlink = vec![0usize; n];
        let mut on_stack = vec![false; n];
        let mut stack: Vec<usize> = Vec::new();
        let mut counter = 0usize;
        // component id per node, components listed in completion order
        let mut comp_of = vec![usize::MAX; n];
        let mut components: Vec<Vec<usize>> = Vec::new();

        // iterative Tarjan (avoids recursion depth issues)
        for start in 0..n {
            if index[start] != usize::MAX {
                continue;
            }
            // call stack of (node, next-neighbor-position)
            let mut call: Vec<(usize, usize)> = vec![(start, 0)];
            index[start] = counter;
            lowlink[start] = counter;
            counter += 1;
            stack.push(start);
            on_stack[start] = true;

            while let Some(&mut (v, ref mut pos)) = call.last_mut() {
                let neighbors: Vec<usize> = self
                    .graph
                    .shareds(nodes[v])
                    .iter()
                    .filter_map(|&t| index_of(t))
                    .collect();
                if *pos < neighbors.len() {
                    let w = neighbors[*pos];
                    *pos += 1;
                    if index[w] == usize::MAX {
                        index[w] = counter;
                        lowlink[w] = counter;
                        counter += 1;
                        stack.push(w);
                        on_stack[w] = true;
                        call.push((w, 0));
                    } else if on_stack[w] {
                        lowlink[v] = lowlink[v].min(index[w]);
                    }
                } else {
                    // finished v
                    if lowlink[v] == index[v] {
                        let mut comp = Vec::new();
                        loop {
                            let w = stack.pop().unwrap();
                            on_stack[w] = false;
                            comp_of[w] = components.len();
                            comp.push(nodes[w]);
                            if w == v {
                                break;
                            }
                        }
                        comp.sort_unstable();
                        components.push(comp);
                    }
                    call.pop();
                    if let Some(&mut (parent, _)) = call.last_mut() {
                        lowlink[parent] = lowlink[parent].min(lowlink[v]);
                    }
                }
            }
        }

        // --- keep components without incoming edges from other components ---
        let mut has_incoming = vec![false; components.len()];
        for v in 0..n {
            for &t in self.graph.shareds(nodes[v]) {
                if let Some(w) = index_of(t) {
                    if comp_of[v] != comp_of[w] {
                        has_incoming[comp_of[w]] = true;
                    }
                }
            }
        }
        for (ci, comp) in components.into_iter().enumerate() {
            if !has_incoming[ci] {
                self.parts.push(comp);
            }
        }
        // deterministic order: by smallest entity number
        self.parts.sort_by_key(|p| p[0]);
    }

    /// Returns count of computed parts (root strong components).
    pub fn nb_parts(&self) -> usize {
        self.parts.len()
    }

    /// Sets iteration to its beginning (evaluates).
    pub fn start(&mut self) {
        self.evaluate();
        self.cursor = 0;
    }

    /// Returns True if there are more sub-parts.
    pub fn more(&self) -> bool {
        self.cursor < self.parts.len()
    }

    /// Sets iteration to the next sub-part.
    pub fn next(&mut self) {
        self.cursor += 1;
    }

    /// Returns True if current sub-part is single (one entity).
    pub fn is_single(&self) -> bool {
        self.parts
            .get(self.cursor)
            .map(|p| p.len() == 1)
            .unwrap_or(false)
    }

    /// Returns the first entity of current sub-part.
    pub fn first_entity(&self) -> Option<usize> {
        self.parts.get(self.cursor).and_then(|p| p.first().copied())
    }

    /// Returns current sub-part as vector.
    pub fn entities(&self) -> Vec<usize> {
        self.parts.get(self.cursor).cloned().unwrap_or_default()
    }

    /// Returns True if entity is in one of the computed parts.
    pub fn is_in_part(&self, entity_num: usize) -> bool {
        self.parts.iter().any(|p| p.contains(&entity_num))
    }

    /// Returns 1-based number of the part containing the entity (0 = none).
    pub fn entity_part_num(&self, entity_num: usize) -> usize {
        for (i, p) in self.parts.iter().enumerate() {
            if p.contains(&entity_num) {
                return i + 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Graph: 1->2, 2->3, 3->1 (cycle), 4->2, 5, 6->5
    fn sample_graph() -> InterfaceGraph {
        let mut g = InterfaceGraph::new(6);
        g.add_shared(1, 2);
        g.add_shared(2, 3);
        g.add_shared(3, 1);
        g.add_shared(4, 2);
        g.add_shared(6, 5);
        g
    }

    #[test]
    fn test_new_not_whole_is_empty() {
        let roots = IfGraphScRoots::new(sample_graph(), false);
        assert!(roots.loaded().is_empty());
        assert_eq!(roots.nb_parts(), 0);
    }

    #[test]
    fn test_new_whole_loads_all() {
        let roots = IfGraphScRoots::new(sample_graph(), true);
        assert_eq!(roots.loaded().len(), 6);
    }

    #[test]
    fn test_roots_of_whole_graph() {
        let mut roots = IfGraphScRoots::new(sample_graph(), true);
        roots.start();
        // Root components: {4} (references the cycle) and {6} (references 5).
        // {1,2,3} is referenced by 4; {5} is referenced by 6.
        assert_eq!(roots.nb_parts(), 2);
        assert_eq!(roots.entities(), vec![4]);
        assert!(roots.is_single());
        roots.next();
        assert!(roots.more());
        assert_eq!(roots.entities(), vec![6]);
        roots.next();
        assert!(!roots.more());
    }

    #[test]
    fn test_cycle_alone_is_root() {
        // Load only the cycle: nothing outside references it -> one part {1,2,3}
        let mut roots = IfGraphScRoots::new(sample_graph(), false);
        roots.get_from_iter(&[1, 2, 3]);
        roots.start();
        assert_eq!(roots.nb_parts(), 1);
        assert_eq!(roots.entities(), vec![1, 2, 3]);
        assert!(!roots.is_single());
        assert_eq!(roots.first_entity(), Some(1));
    }

    #[test]
    fn test_get_from_entity_shared() {
        let mut roots = IfGraphScRoots::new(sample_graph(), false);
        // 4 shares 2, which is in the cycle 1-2-3 -> closure is {4,1,2,3}
        roots.get_from_entity(4, true);
        let mut loaded = roots.loaded();
        loaded.sort_unstable();
        assert_eq!(loaded, vec![1, 2, 3, 4]);
        roots.start();
        // Only {4} is a root; the cycle is referenced by 4.
        assert_eq!(roots.nb_parts(), 1);
        assert_eq!(roots.entities(), vec![4]);
    }

    #[test]
    fn test_entity_part_num_and_is_in_part() {
        let mut roots = IfGraphScRoots::new(sample_graph(), true);
        roots.start();
        assert!(roots.is_in_part(4));
        assert!(roots.is_in_part(6));
        assert!(!roots.is_in_part(1));
        assert_eq!(roots.entity_part_num(4), 1);
        assert_eq!(roots.entity_part_num(6), 2);
        assert_eq!(roots.entity_part_num(5), 0);
    }

    #[test]
    fn test_reset() {
        let mut roots = IfGraphScRoots::new(sample_graph(), true);
        roots.start();
        assert!(roots.nb_parts() > 0);
        roots.reset();
        assert!(roots.loaded().is_empty());
        assert_eq!(roots.nb_parts(), 0);
    }

    #[test]
    fn test_isolated_entities_are_roots() {
        let g = InterfaceGraph::new(3); // no references at all
        let mut roots = IfGraphScRoots::new(g, true);
        roots.start();
        assert_eq!(roots.nb_parts(), 3);
        assert!(roots.is_single());
    }
}
