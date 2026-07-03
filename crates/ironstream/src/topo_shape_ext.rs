// FILE: topo_shape_ext.rs
// occt: BRepTools_History, ShapeExtend_Status, ShapeExtend_BasicMsgRegistrator

/// Extended status of a shape operation.
/// occt: ShapeExtend_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeExtendStatus {
    OK,
    Done1,
    Done2,
    Done3,
    Done4,
    Done5,
    Fail1,
    Fail2,
    Fail3,
    Fail4,
    Fail5,
}

impl Default for ShapeExtendStatus {
    fn default() -> Self { Self::OK }
}

impl ShapeExtendStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done1|Self::Done2|Self::Done3|Self::Done4|Self::Done5)
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail1|Self::Fail2|Self::Fail3|Self::Fail4|Self::Fail5)
    }
}

/// A message registered by shape operations.
/// occt: ShapeExtend_BasicMsgRegistrator
#[derive(Clone, Debug)]
pub struct ShapeMsg {
    pub shape_id: u32,
    pub message: String,
    pub gravity: ShapeMsgGravity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShapeMsgGravity {
    #[default]
    Trace,
    Warning,
    Alarm,
    Fail,
}

/// Collects messages from shape processing operations.
/// occt: ShapeExtend_BasicMsgRegistrator
#[derive(Clone, Debug, Default)]
pub struct ShapeMsgRegistrator {
    pub messages: Vec<ShapeMsg>,
}

impl ShapeMsgRegistrator {
    pub fn new() -> Self { Self::default() }

    pub fn send(&mut self, shape_id: u32, msg: &str, gravity: ShapeMsgGravity) {
        self.messages.push(ShapeMsg { shape_id, message: msg.to_string(), gravity });
    }

    pub fn nb_messages(&self) -> usize { self.messages.len() }

    pub fn has_failures(&self) -> bool {
        self.messages.iter().any(|m| m.gravity == ShapeMsgGravity::Fail)
    }

    pub fn has_warnings(&self) -> bool {
        self.messages.iter().any(|m| m.gravity == ShapeMsgGravity::Warning)
    }

    pub fn clear(&mut self) { self.messages.clear(); }

    pub fn messages_for(&self, shape_id: u32) -> Vec<&ShapeMsg> {
        self.messages.iter().filter(|m| m.shape_id == shape_id).collect()
    }
}

/// History of shape modifications (old→new mappings).
/// occt: BRepTools_History
#[derive(Clone, Debug, Default)]
pub struct BrepToolsHistory {
    pub modifications: Vec<(u32, u32)>,    // (old_id, new_id)
    pub deletions: Vec<u32>,               // deleted shape ids
    pub additions: Vec<u32>,               // added shape ids
}

impl BrepToolsHistory {
    pub fn new() -> Self { Self::default() }

    pub fn add_modification(&mut self, old_id: u32, new_id: u32) {
        self.modifications.push((old_id, new_id));
    }

    pub fn add_deletion(&mut self, id: u32) {
        if !self.deletions.contains(&id) { self.deletions.push(id); }
    }

    pub fn add_addition(&mut self, id: u32) {
        if !self.additions.contains(&id) { self.additions.push(id); }
    }

    pub fn modified(&self, old_id: u32) -> Vec<u32> {
        self.modifications.iter().filter(|(o,_)| *o == old_id).map(|(_,n)| *n).collect()
    }

    pub fn is_removed(&self, id: u32) -> bool { self.deletions.contains(&id) }
    pub fn nb_modifications(&self) -> usize { self.modifications.len() }
    pub fn nb_deletions(&self) -> usize { self.deletions.len() }

    pub fn merge(&mut self, other: &BrepToolsHistory) {
        self.modifications.extend_from_slice(&other.modifications);
        for &id in &other.deletions { self.add_deletion(id); }
        for &id in &other.additions { self.add_addition(id); }
    }

    pub fn clear(&mut self) {
        self.modifications.clear();
        self.deletions.clear();
        self.additions.clear();
    }
}

/// Shape transfer result (maps old to new shape ids).
/// occt: BRepTools_Modifier / Transfer patterns
#[derive(Clone, Debug, Default)]
pub struct BrepShapeMapper {
    pub map: Vec<(u32, u32)>,
}

impl BrepShapeMapper {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, from_id: u32, to_id: u32) {
        if let Some(entry) = self.map.iter_mut().find(|(f,_)| *f == from_id) {
            entry.1 = to_id;
        } else {
            self.map.push((from_id, to_id));
        }
    }

    pub fn find(&self, from_id: u32) -> Option<u32> {
        self.map.iter().find(|(f,_)| *f == from_id).map(|(_,t)| *t)
    }

    pub fn is_bound(&self, from_id: u32) -> bool {
        self.map.iter().any(|(f,_)| *f == from_id)
    }

    pub fn nb_bound(&self) -> usize { self.map.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_done_fail() {
        assert!(ShapeExtendStatus::Done1.is_done());
        assert!(!ShapeExtendStatus::Done1.is_fail());
        assert!(ShapeExtendStatus::Fail2.is_fail());
        assert!(!ShapeExtendStatus::OK.is_done());
        assert!(!ShapeExtendStatus::OK.is_fail());
    }

    #[test]
    fn msg_registrator() {
        let mut r = ShapeMsgRegistrator::new();
        r.send(1, "OK", ShapeMsgGravity::Trace);
        r.send(1, "bad edge", ShapeMsgGravity::Fail);
        r.send(2, "check warning", ShapeMsgGravity::Warning);
        assert_eq!(r.nb_messages(), 3);
        assert!(r.has_failures());
        assert!(r.has_warnings());
        assert_eq!(r.messages_for(1).len(), 2);
    }

    #[test]
    fn history_modifications() {
        let mut h = BrepToolsHistory::new();
        h.add_modification(10, 20);
        h.add_modification(10, 21);
        h.add_deletion(5);
        h.add_deletion(5);  // dup
        assert_eq!(h.modified(10).len(), 2);
        assert!(h.is_removed(5));
        assert!(!h.is_removed(99));
        assert_eq!(h.nb_deletions(), 1);
    }

    #[test]
    fn history_merge() {
        let mut h1 = BrepToolsHistory::new();
        h1.add_modification(1, 10);
        let mut h2 = BrepToolsHistory::new();
        h2.add_modification(2, 20);
        h2.add_deletion(3);
        h1.merge(&h2);
        assert_eq!(h1.nb_modifications(), 2);
        assert_eq!(h1.nb_deletions(), 1);
    }

    #[test]
    fn shape_mapper_bind_find() {
        let mut m = BrepShapeMapper::new();
        m.bind(1, 100);
        m.bind(2, 200);
        assert_eq!(m.find(1), Some(100));
        assert!(m.is_bound(2));
        assert!(!m.is_bound(99));
        m.bind(1, 101);  // override
        assert_eq!(m.find(1), Some(101));
    }
}
