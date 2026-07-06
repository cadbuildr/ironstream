// FILE: step_data_free_form_entity.rs
// occt: StepData_FreeFormEntity

use std::cell::RefCell;
use std::rc::Rc;

// Local helper mirroring StepData_Field (external plumbing, subset)
#[derive(Clone, Default)]
pub struct StepDataField {
    kind: i32,
    int_val: i32,
    real_val: f64,
    text: Option<String>,
}

impl StepDataField {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_integer(&mut self, val: i32) {
        self.kind = 1;
        self.int_val = val;
    }
    pub fn integer(&self) -> i32 {
        self.int_val
    }
    pub fn set_real(&mut self, val: f64) {
        self.kind = 5;
        self.real_val = val;
    }
    pub fn real(&self) -> f64 {
        self.real_val
    }
    pub fn set_string(&mut self, val: &str) {
        self.kind = 6;
        self.text = Some(val.to_string());
    }
    pub fn string(&self) -> &str {
        self.text.as_deref().unwrap_or("")
    }
    pub fn is_set(&self) -> bool {
        self.kind != 0
    }
}

pub type FreeFormEntityHandle = Rc<RefCell<StepDataFreeFormEntity>>;

// A Free Form Entity allows to record any kind of STEP parameters,
// in any way of typing. It is implemented with an array of fields.
// A Complex entity can be defined as a chain of FreeFormEntity
// (see next and typed).
pub struct StepDataFreeFormEntity {
    step_type: String,
    fields: Vec<StepDataField>,
    next: Option<FreeFormEntityHandle>,
}

impl StepDataFreeFormEntity {
    // Creates a FreeFormEntity, with no field, no type
    pub fn new() -> Self {
        StepDataFreeFormEntity {
            step_type: String::new(),
            fields: Vec::new(),
            next: None,
        }
    }

    pub fn new_handle() -> FreeFormEntityHandle {
        Rc::new(RefCell::new(Self::new()))
    }

    // Sets the type of an entity; for a complex one, the type of this member
    pub fn set_step_type(&mut self, typenam: &str) {
        self.step_type.clear();
        self.step_type.push_str(typenam);
    }

    // Returns the recorded StepType
    pub fn step_type(&self) -> &str {
        &self.step_type
    }

    // Sets a next member, defining or completing a Complex entity.
    // If last is true, next is set as last of the list;
    // else it is inserted just as next of self.
    // If next is None, next is cleared.
    pub fn set_next(&mut self, next: Option<FreeFormEntityHandle>, last: bool) {
        match next {
            None => self.next = None,
            Some(n) => {
                if self.next.is_none() {
                    self.next = Some(n);
                } else if last {
                    self.next
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .set_next(Some(n), true);
                } else {
                    n.borrow_mut().set_next(self.next.take(), last);
                    self.next = Some(n);
                }
            }
        }
    }

    // Returns the next member of a Complex entity (the last member has none)
    pub fn next(&self) -> Option<FreeFormEntityHandle> {
        self.next.clone()
    }

    // Returns True if a FreeFormEntity is Complex (i.e. has Next)
    pub fn is_complex(&self) -> bool {
        self.next.is_some()
    }

    // Returns the member of which the type name is given, searching
    // from a handle so the head itself can be returned (exact match).
    pub fn typed(this: &FreeFormEntityHandle, typenam: &str) -> Option<FreeFormEntityHandle> {
        if this.borrow().step_type == typenam {
            return Some(Rc::clone(this));
        }
        let next = this.borrow().next.clone();
        match next {
            None => None,
            Some(n) => Self::typed(&n, typenam),
        }
    }

    // Returns the list of types (one type for a simple entity), as is
    pub fn type_list(&self) -> Vec<String> {
        let mut li = vec![self.step_type.clone()];
        let mut next = self.next.clone();
        while let Some(n) = next {
            li.push(n.borrow().step_type.clone());
            next = n.borrow().next.clone();
        }
        li
    }

    // Reorders a Complex entity if required, i.e. if member types are
    // not in alphabetic order. Returns false if nothing done (order was
    // OK or simple entity), true plus modified ent if reordered.
    pub fn reorder(ent: &mut FreeFormEntityHandle) -> bool {
        if !ent.borrow().is_complex() {
            return false;
        }
        // check whether already in alphabetic order
        let mut afr = false;
        {
            let mut e1 = Rc::clone(ent);
            loop {
                let e2 = e1.borrow().next.clone();
                match e2 {
                    None => break,
                    Some(e2) => {
                        if e1.borrow().step_type > e2.borrow().step_type {
                            afr = true;
                            break;
                        }
                        e1 = e2;
                    }
                }
            }
        }
        if !afr {
            return false;
        }
        // collect members, sort by type, relink
        let mut members: Vec<FreeFormEntityHandle> = Vec::new();
        let mut cur = Some(Rc::clone(ent));
        while let Some(c) = cur {
            let nx = c.borrow().next.clone();
            members.push(c);
            cur = nx;
        }
        members.sort_by(|a, b| a.borrow().step_type.cmp(&b.borrow().step_type));
        for m in &members {
            m.borrow_mut().next = None;
        }
        for i in (0..members.len() - 1).rev() {
            let nx = Rc::clone(&members[i + 1]);
            members[i].borrow_mut().next = Some(nx);
        }
        *ent = Rc::clone(&members[0]);
        true
    }

    // Sets a count of Fields, from scratch
    pub fn set_nb_fields(&mut self, nb: usize) {
        self.fields = vec![StepDataField::new(); nb];
    }

    // Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    // Returns a field from its rank (1-based), for read-only use
    pub fn field(&self, num: usize) -> Option<&StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&self.fields[num - 1])
    }

    // Returns a field from its rank (1-based), in order to modify it
    pub fn c_field(&mut self, num: usize) -> Option<&mut StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&mut self.fields[num - 1])
    }
}

impl Default for StepDataFreeFormEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_form_entity_new() {
        let mut entity = StepDataFreeFormEntity::new();
        entity.set_step_type("test");
        assert_eq!(entity.step_type(), "test");
        assert_eq!(entity.nb_fields(), 0);
        assert!(!entity.is_complex());
    }

    #[test]
    fn test_fields() {
        let mut entity = StepDataFreeFormEntity::new();
        entity.set_nb_fields(3);
        assert_eq!(entity.nb_fields(), 3);
        entity.c_field(1).unwrap().set_integer(7);
        entity.c_field(2).unwrap().set_real(1.5);
        entity.c_field(3).unwrap().set_string("abc");
        assert_eq!(entity.field(1).unwrap().integer(), 7);
        assert!((entity.field(2).unwrap().real() - 1.5).abs() < 1e-12);
        assert_eq!(entity.field(3).unwrap().string(), "abc");
        assert!(entity.field(4).is_none());
        assert!(entity.field(0).is_none());
    }

    fn make(t: &str) -> FreeFormEntityHandle {
        let h = StepDataFreeFormEntity::new_handle();
        h.borrow_mut().set_step_type(t);
        h
    }

    #[test]
    fn test_complex_chain_and_type_list() {
        let head = make("A");
        head.borrow_mut().set_next(Some(make("B")), true);
        head.borrow_mut().set_next(Some(make("C")), true);
        assert!(head.borrow().is_complex());
        assert_eq!(head.borrow().type_list(), vec!["A", "B", "C"]);

        // insert (not last): becomes direct next of head
        head.borrow_mut().set_next(Some(make("X")), false);
        assert_eq!(head.borrow().type_list(), vec!["A", "X", "B", "C"]);

        // clearing next
        head.borrow_mut().set_next(None, true);
        assert!(!head.borrow().is_complex());
    }

    #[test]
    fn test_typed() {
        let head = make("A");
        head.borrow_mut().set_next(Some(make("B")), true);
        let found = StepDataFreeFormEntity::typed(&head, "B").expect("B must be found");
        assert_eq!(found.borrow().step_type(), "B");
        let head_found = StepDataFreeFormEntity::typed(&head, "A").expect("A must be found");
        assert!(Rc::ptr_eq(&head_found, &head));
        assert!(StepDataFreeFormEntity::typed(&head, "Z").is_none());
    }

    #[test]
    fn test_reorder() {
        // simple entity: nothing done
        let mut simple = make("A");
        assert!(!StepDataFreeFormEntity::reorder(&mut simple));

        // already ordered: nothing done
        let mut ordered = make("A");
        ordered.borrow_mut().set_next(Some(make("B")), true);
        assert!(!StepDataFreeFormEntity::reorder(&mut ordered));
        assert_eq!(ordered.borrow().type_list(), vec!["A", "B"]);

        // out of order: reordered alphabetically, head updated
        let mut ent = make("C");
        ent.borrow_mut().set_next(Some(make("A")), true);
        ent.borrow_mut().set_next(Some(make("B")), true);
        assert!(StepDataFreeFormEntity::reorder(&mut ent));
        assert_eq!(ent.borrow().type_list(), vec!["A", "B", "C"]);
    }
}
