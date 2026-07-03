// FILE: ocaf_func.rs
// occt: TFunction_Function, TFunction_Scope, TFunction_Iterator,
//       TFunction_GraphNode, TFunction_IFunction

/// Execution status of a function.
/// occt: TFunction_ExecutionStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TFunctionExecutionStatus {
    #[default]
    NotExecuted,
    Executing,
    Succeeded,
    Failed,
}

impl TFunctionExecutionStatus {
    pub fn is_done(&self) -> bool { *self == Self::Succeeded }
    pub fn is_failed(&self) -> bool { *self == Self::Failed }
}

/// A function in the OCAF function framework.
/// occt: TFunction_Function
#[derive(Clone, Debug)]
pub struct TFunctionFunction {
    pub label_id: u32,
    pub guid: u64,
    pub status: TFunctionExecutionStatus,
    pub failure_message: Option<String>,
}

impl TFunctionFunction {
    pub fn new(label_id: u32, guid: u64) -> Self {
        Self { label_id, guid, status: TFunctionExecutionStatus::NotExecuted, failure_message: None }
    }

    pub fn set_status(&mut self, s: TFunctionExecutionStatus) { self.status = s; }

    pub fn execute_stub(&mut self) {
        self.status = TFunctionExecutionStatus::Executing;
        self.status = TFunctionExecutionStatus::Succeeded;
    }

    pub fn fail(&mut self, msg: impl Into<String>) {
        self.status = TFunctionExecutionStatus::Failed;
        self.failure_message = Some(msg.into());
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
}

/// A dependency graph node for a function.
/// occt: TFunction_GraphNode
#[derive(Clone, Debug, Default)]
pub struct TFunctionGraphNode {
    pub function_id: u32,
    pub previous_ids: Vec<u32>,
    pub next_ids: Vec<u32>,
    pub status: TFunctionExecutionStatus,
}

impl TFunctionGraphNode {
    pub fn new(function_id: u32) -> Self {
        Self { function_id, ..Self::default() }
    }

    pub fn add_previous(&mut self, id: u32) {
        if !self.previous_ids.contains(&id) { self.previous_ids.push(id); }
    }

    pub fn add_next(&mut self, id: u32) {
        if !self.next_ids.contains(&id) { self.next_ids.push(id); }
    }

    pub fn nb_previous(&self) -> usize { self.previous_ids.len() }
    pub fn nb_next(&self) -> usize { self.next_ids.len() }

    pub fn is_root(&self) -> bool { self.previous_ids.is_empty() }
    pub fn is_leaf(&self) -> bool { self.next_ids.is_empty() }
}

/// Scope holding all functions and their dependency graph.
/// occt: TFunction_Scope
#[derive(Clone, Debug, Default)]
pub struct TFunctionScope {
    pub functions: Vec<TFunctionFunction>,
    pub graph: Vec<TFunctionGraphNode>,
}

impl TFunctionScope {
    pub fn new() -> Self { Self::default() }

    pub fn add_function(&mut self, label_id: u32, guid: u64) -> usize {
        let idx = self.functions.len();
        self.functions.push(TFunctionFunction::new(label_id, guid));
        self.graph.push(TFunctionGraphNode::new(label_id));
        idx
    }

    pub fn nb_functions(&self) -> usize { self.functions.len() }

    pub fn function(&self, idx: usize) -> Option<&TFunctionFunction> { self.functions.get(idx) }
    pub fn function_mut(&mut self, idx: usize) -> Option<&mut TFunctionFunction> { self.functions.get_mut(idx) }

    pub fn add_dependency(&mut self, from_idx: usize, to_idx: usize) {
        if let (Some(from_id), Some(to_id)) = (
            self.functions.get(from_idx).map(|f| f.label_id),
            self.functions.get(to_idx).map(|f| f.label_id),
        ) {
            if let Some(gn) = self.graph.get_mut(from_idx) { gn.add_next(to_id); }
            if let Some(gn) = self.graph.get_mut(to_idx)   { gn.add_previous(from_id); }
        }
    }

    /// Return indices of functions with NotExecuted or Failed status.
    pub fn pending_functions(&self) -> Vec<usize> {
        self.functions.iter().enumerate()
            .filter(|(_, f)| !f.status.is_done())
            .map(|(i, _)| i)
            .collect()
    }

    /// Execute all functions in topological order (BFS from roots).
    pub fn execute_all_stub(&mut self) {
        let n = self.functions.len();
        let mut in_degree: Vec<usize> = self.graph.iter().map(|g| g.nb_previous()).collect();
        let mut queue: Vec<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut visited = 0;

        while let Some(i) = if visited < queue.len() { Some(queue[visited]) } else { None } {
            visited += 1;
            self.functions[i].execute_stub();

            let next_ids: Vec<u32> = self.graph[i].next_ids.clone();
            for nid in next_ids {
                if let Some(j) = self.functions.iter().position(|f| f.label_id == nid) {
                    if in_degree[j] > 0 { in_degree[j] -= 1; }
                    if in_degree[j] == 0 { queue.push(j); }
                }
            }
        }
    }
}

/// Iterator over functions in dependency order.
/// occt: TFunction_Iterator
#[derive(Clone, Debug)]
pub struct TFunctionIterator<'a> {
    scope: &'a TFunctionScope,
    cursor: usize,
}

impl<'a> TFunctionIterator<'a> {
    pub fn new(scope: &'a TFunctionScope) -> Self {
        Self { scope, cursor: 0 }
    }

    pub fn more(&self) -> bool { self.cursor < self.scope.nb_functions() }
    pub fn next(&mut self) { self.cursor += 1; }
    pub fn current(&self) -> Option<&TFunctionFunction> { self.scope.function(self.cursor) }
}

/// IFunction convenience accessor (wraps scope+index).
/// occt: TFunction_IFunction
#[derive(Clone, Debug)]
pub struct TFunctionIFunction<'a> {
    pub scope: &'a TFunctionScope,
    pub index: usize,
}

impl<'a> TFunctionIFunction<'a> {
    pub fn new(scope: &'a TFunctionScope, index: usize) -> Self { Self { scope, index } }

    pub fn function(&self) -> Option<&TFunctionFunction> { self.scope.function(self.index) }
    pub fn is_done(&self) -> bool { self.function().map(|f| f.is_done()).unwrap_or(false) }
    pub fn guid(&self) -> Option<u64> { self.function().map(|f| f.guid) }
    pub fn label_id(&self) -> Option<u32> { self.function().map(|f| f.label_id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_execute() {
        let mut f = TFunctionFunction::new(1, 0xABCD);
        assert!(!f.is_done());
        f.execute_stub();
        assert!(f.is_done());
    }

    #[test]
    fn function_fail() {
        let mut f = TFunctionFunction::new(2, 0x1234);
        f.fail("something went wrong");
        assert!(f.status.is_failed());
        assert_eq!(f.failure_message.as_deref(), Some("something went wrong"));
    }

    #[test]
    fn graph_node_deps() {
        let mut n = TFunctionGraphNode::new(10);
        n.add_previous(5);
        n.add_next(15);
        n.add_next(20);
        n.add_next(15); // dup
        assert_eq!(n.nb_previous(), 1);
        assert_eq!(n.nb_next(), 2);
        assert!(!n.is_root());
        assert!(!n.is_leaf());
    }

    #[test]
    fn scope_add_and_execute() {
        let mut s = TFunctionScope::new();
        let i0 = s.add_function(1, 100);
        let i1 = s.add_function(2, 200);
        s.add_dependency(i0, i1);
        assert_eq!(s.pending_functions().len(), 2);
        s.execute_all_stub();
        assert_eq!(s.pending_functions().len(), 0);
        assert!(s.function(i0).unwrap().is_done());
        assert!(s.function(i1).unwrap().is_done());
    }

    #[test]
    fn iterator_visits_all() {
        let mut s = TFunctionScope::new();
        s.add_function(1, 1);
        s.add_function(2, 2);
        let mut it = TFunctionIterator::new(&s);
        let mut count = 0;
        while it.more() { count += 1; it.next(); }
        assert_eq!(count, 2);
    }

    #[test]
    fn ifunction_accessor() {
        let mut s = TFunctionScope::new();
        let idx = s.add_function(42, 0xDEAD);
        s.function_mut(idx).unwrap().execute_stub();
        let ifc = TFunctionIFunction::new(&s, idx);
        assert!(ifc.is_done());
        assert_eq!(ifc.guid(), Some(0xDEAD));
        assert_eq!(ifc.label_id(), Some(42));
    }
}
