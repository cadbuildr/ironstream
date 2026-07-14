// FILE: transfer_actor.rs
// occt: Transfer_ActorOfTransientProcess
// occt-ref: Transfer_Actor, Transfer_Binder
//       Transfer_SimpleBinderOfTransient

/// Binder: maps entities during transfer (read/write operations).
// occt-ref: Transfer_Binder // (abstract base)
#[derive(Clone, Debug)]
pub struct TransferBinder {
    pub binder_id: u32,
    pub source_entity_id: u32,
    pub target_entity_id: Option<u32>,
    pub is_bound: bool,
}

impl TransferBinder {
    pub fn new(bid: u32, source_id: u32) -> Self {
        Self {
            binder_id: bid,
            source_entity_id: source_id,
            target_entity_id: None,
            is_bound: false,
        }
    }

    pub fn bind_target(&mut self, target_id: u32) {
        self.target_entity_id = Some(target_id);
        self.is_bound = true;
    }

    pub fn is_binded(&self) -> bool { self.is_bound }
    pub fn result(&self) -> Option<u32> { self.target_entity_id }
}

/// Simple binder for transient objects.
/// occt: Transfer_SimpleBinderOfTransient
#[derive(Clone, Debug)]
pub struct SimpleBinderOfTransient {
    pub binder: TransferBinder,
    pub result_label: String,  // Simplified: store as label
}

impl SimpleBinderOfTransient {
    pub fn new(bid: u32, source_id: u32) -> Self {
        Self {
            binder: TransferBinder::new(bid, source_id),
            result_label: String::new(),
        }
    }

    pub fn set_result_label(&mut self, label: impl Into<String>) {
        self.result_label = label.into();
    }

    pub fn result_label(&self) -> &str { &self.result_label }
}

/// Actor: converts entities during transfer.
/// occt-note: Transfer_Actor (abstract)
#[derive(Clone, Debug)]
pub struct TransferActor {
    pub actor_id: u32,
    pub actor_name: String,
    pub input_type: String,
    pub output_type: String,
    pub is_active: bool,
}

impl TransferActor {
    pub fn new(aid: u32, name: impl Into<String>, in_type: impl Into<String>, out_type: impl Into<String>) -> Self {
        Self {
            actor_id: aid,
            actor_name: name.into(),
            input_type: in_type.into(),
            output_type: out_type.into(),
            is_active: true,
        }
    }

    pub fn activate(&mut self) { self.is_active = true; }
    pub fn deactivate(&mut self) { self.is_active = false; }

    pub fn handles_input(&self, input_type: &str) -> bool {
        self.is_active && self.input_type == input_type
    }

    /// Execute transfer: stub returns source_id + 1000 as result.
    pub fn execute(&self, source_id: u32) -> u32 {
        if self.is_active { source_id + 1000 } else { 0 }
    }
}

/// Process: manages actors and bindings during transfer.
/// occt: Transfer_ActorOfTransientProcess // (simplified)
#[derive(Clone, Debug, Default)]
pub struct TransferProcess {
    pub actors: Vec<TransferActor>,
    pub bindings: Vec<TransferBinder>,
}

impl TransferProcess {
    pub fn new() -> Self { Self::default() }

    pub fn add_actor(&mut self, actor: TransferActor) {
        self.actors.push(actor);
    }

    pub fn find_actor(&self, input_type: &str) -> Option<&TransferActor> {
        self.actors.iter().find(|a| a.handles_input(input_type))
    }

    pub fn add_binder(&mut self, binder: TransferBinder) {
        self.bindings.push(binder);
    }

    pub fn find_binder(&self, source_id: u32) -> Option<&TransferBinder> {
        self.bindings.iter().find(|b| b.source_entity_id == source_id)
    }

    pub fn transfer(&mut self, source_id: u32, input_type: &str) -> Option<u32> {
        if let Some(actor) = self.find_actor(input_type) {
            let result_id = actor.execute(source_id);
            let mut binder = TransferBinder::new((self.bindings.len() + 1) as u32, source_id);
            binder.bind_target(result_id);
            self.add_binder(binder);
            Some(result_id)
        } else {
            None
        }
    }

    pub fn nb_actors(&self) -> usize { self.actors.len() }
    pub fn nb_bindings(&self) -> usize { self.bindings.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binder_binding() {
        let mut b = TransferBinder::new(1, 10);
        assert!(!b.is_binded());
        b.bind_target(20);
        assert!(b.is_binded());
        assert_eq!(b.result(), Some(20));
    }

    #[test]
    fn simple_binder() {
        let mut sb = SimpleBinderOfTransient::new(1, 100);
        sb.set_result_label("result_label_1");
        assert_eq!(sb.result_label(), "result_label_1");
    }

    #[test]
    fn transfer_actor() {
        let mut a = TransferActor::new(1, "MyActor", "SourceType", "TargetType");
        assert!(a.is_active);
        assert!(a.handles_input("SourceType"));
        let result = a.execute(42);
        assert_eq!(result, 1042);
    }

    #[test]
    fn transfer_process() {
        let mut p = TransferProcess::new();
        p.add_actor(TransferActor::new(1, "Actor1", "Type1", "Type2"));
        p.add_actor(TransferActor::new(2, "Actor2", "Type3", "Type4"));
        assert_eq!(p.nb_actors(), 2);
        let result = p.transfer(50, "Type1");
        assert!(result.is_some());
        assert_eq!(p.nb_bindings(), 1);
    }
}
