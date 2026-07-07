// FILE: step_data_undefined_entity.rs
// occt: StepData_UndefinedEntity

use std::cell::RefCell;
use std::rc::Rc;

// Local helper mirroring Interface_ParamType (external plumbing, subset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceParamType {
    Integer,
    Real,
    Ident,
    Text,
    Enum,
    Misc,
}

// Local helper mirroring Interface_UndefinedContent (external plumbing):
// records the parameters of an undefined (unrecognized) entity
pub struct InterfaceUndefinedContent {
    params: Vec<(InterfaceParamType, String)>,
}

impl InterfaceUndefinedContent {
    pub fn new() -> Self {
        InterfaceUndefinedContent { params: Vec::new() }
    }

    pub fn add_literal(&mut self, ptype: InterfaceParamType, value: &str) {
        self.params.push((ptype, value.to_string()));
    }

    pub fn nb_params(&self) -> usize {
        self.params.len()
    }

    // 1-based access, as in OCCT
    pub fn param_type(&self, num: usize) -> Option<InterfaceParamType> {
        self.params.get(num.checked_sub(1)?).map(|p| p.0)
    }

    pub fn param_value(&self, num: usize) -> Option<&str> {
        self.params.get(num.checked_sub(1)?).map(|p| p.1.as_str())
    }
}

pub type UndefinedEntityHandle = Rc<RefCell<StepDataUndefinedEntity>>;

// Undefined entity specific to Step Interface, in which StepType is
// defined at each instance, or is a SubList of another one.
// An entity defined by STEP can be "Complex Type" (ANDOR clause):
// modelled as a chain through next()
pub struct StepDataUndefinedEntity {
    step_type: Option<String>,
    content: Rc<RefCell<InterfaceUndefinedContent>>,
    sub: bool,
    next: Option<UndefinedEntityHandle>,
}

impl StepDataUndefinedEntity {
    // creates an Unknown entity
    pub fn new() -> Self {
        StepDataUndefinedEntity {
            step_type: None,
            content: Rc::new(RefCell::new(InterfaceUndefinedContent::new())),
            sub: false,
            next: None,
        }
    }

    // Creates a SubList of an Unknown entity: an Unknown Entity with
    // no Type, but flagged as "SUB" if issub is true
    pub fn new_sub(issub: bool) -> Self {
        let mut e = Self::new();
        e.sub = issub;
        e
    }

    // Returns the UndefinedContent which brings the Parameters
    pub fn undefined_content(&self) -> Rc<RefCell<InterfaceUndefinedContent>> {
        Rc::clone(&self.content)
    }

    // Returns True if an Undefined Entity is SubPart of another one
    pub fn is_sub(&self) -> bool {
        self.sub
    }

    // Returns True if this defines a Multiple Type Entity (see ANDOR)
    pub fn is_complex(&self) -> bool {
        self.next.is_some()
    }

    // For a Multiple Type Entity, returns the Next "Component";
    // None marks the end of the list
    pub fn next(&self) -> Option<UndefinedEntityHandle> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Option<UndefinedEntityHandle>) {
        self.next = next;
    }

    // gives entity type, read from file; "" when not set (null handle
    // in OCCT). For a Complex Type Entity, gives the first Type read,
    // each next() gives its "partial" type
    pub fn step_type(&self) -> &str {
        self.step_type.as_deref().unwrap_or("")
    }

    pub fn set_step_type(&mut self, typenam: &str) {
        self.step_type = Some(typenam.to_string());
    }

    // Mirrors GetFromAnother: copies type, parameters, sub flag and
    // (recursively) the complex chain from another UndefinedEntity
    pub fn get_from_another(&mut self, another: &StepDataUndefinedEntity) {
        self.step_type = Some(another.step_type().to_string());
        let mut cont = InterfaceUndefinedContent::new();
        {
            let other = another.content.borrow();
            for num in 1..=other.nb_params() {
                cont.add_literal(
                    other.param_type(num).unwrap(),
                    other.param_value(num).unwrap(),
                );
            }
        }
        self.content = Rc::new(RefCell::new(cont));
        self.sub = another.is_sub();
        match another.next() {
            Some(n) => {
                let mut copy = StepDataUndefinedEntity::new();
                copy.get_from_another(&n.borrow());
                self.next = Some(Rc::new(RefCell::new(copy)));
            }
            None => self.next = None,
        }
    }
}

impl Default for StepDataUndefinedEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_entity_new() {
        let entity = StepDataUndefinedEntity::new();
        assert_eq!(entity.step_type(), "");
        assert!(!entity.is_sub());
        assert!(!entity.is_complex());
        assert_eq!(entity.undefined_content().borrow().nb_params(), 0);
    }

    #[test]
    fn test_sub_flag() {
        let sub = StepDataUndefinedEntity::new_sub(true);
        assert!(sub.is_sub());
        assert_eq!(sub.step_type(), "");
        let notsub = StepDataUndefinedEntity::new_sub(false);
        assert!(!notsub.is_sub());
    }

    #[test]
    fn test_type_and_content() {
        let mut entity = StepDataUndefinedEntity::new();
        entity.set_step_type("MYSTERY_ENTITY");
        assert_eq!(entity.step_type(), "MYSTERY_ENTITY");

        let content = entity.undefined_content();
        content
            .borrow_mut()
            .add_literal(InterfaceParamType::Integer, "42");
        content
            .borrow_mut()
            .add_literal(InterfaceParamType::Text, "hello");
        assert_eq!(content.borrow().nb_params(), 2);
        assert_eq!(
            content.borrow().param_type(1),
            Some(InterfaceParamType::Integer)
        );
        assert_eq!(content.borrow().param_value(2), Some("hello"));
        assert_eq!(content.borrow().param_value(3), None);
    }

    #[test]
    fn test_complex_chain_and_copy() {
        let mut head = StepDataUndefinedEntity::new();
        head.set_step_type("A");
        let mut second = StepDataUndefinedEntity::new();
        second.set_step_type("B");
        second
            .undefined_content()
            .borrow_mut()
            .add_literal(InterfaceParamType::Real, "1.5");
        head.set_next(Some(Rc::new(RefCell::new(second))));
        assert!(head.is_complex());
        assert_eq!(head.next().unwrap().borrow().step_type(), "B");

        // deep copy through get_from_another
        let mut copy = StepDataUndefinedEntity::new();
        copy.get_from_another(&head);
        assert_eq!(copy.step_type(), "A");
        assert!(copy.is_complex());
        let cnext = copy.next().unwrap();
        assert_eq!(cnext.borrow().step_type(), "B");
        assert_eq!(
            cnext.borrow().undefined_content().borrow().param_value(1),
            Some("1.5")
        );
        // copies are independent handles
        assert!(!Rc::ptr_eq(&cnext, &head.next().unwrap()));
    }
}
