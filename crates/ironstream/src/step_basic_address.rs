// FILE: step_basic_address.rs
// occt: StepBasic_Address

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }
}

pub struct StepBasic_Address {
    internal_location: Option<Rc<RefCell<HString>>>,
    street_number: Option<Rc<RefCell<HString>>>,
    street: Option<Rc<RefCell<HString>>>,
    postal_box: Option<Rc<RefCell<HString>>>,
    town: Option<Rc<RefCell<HString>>>,
    region: Option<Rc<RefCell<HString>>>,
    postal_code: Option<Rc<RefCell<HString>>>,
    country: Option<Rc<RefCell<HString>>>,
    facsimile_number: Option<Rc<RefCell<HString>>>,
    telephone_number: Option<Rc<RefCell<HString>>>,
    electronic_mail_address: Option<Rc<RefCell<HString>>>,
    telex_number: Option<Rc<RefCell<HString>>>,
    has_internal_location: bool,
    has_street_number: bool,
    has_street: bool,
    has_postal_box: bool,
    has_town: bool,
    has_region: bool,
    has_postal_code: bool,
    has_country: bool,
    has_facsimile_number: bool,
    has_telephone_number: bool,
    has_electronic_mail_address: bool,
    has_telex_number: bool,
}

impl StepBasic_Address {
    pub fn new() -> Self {
        StepBasic_Address {
            internal_location: None,
            street_number: None,
            street: None,
            postal_box: None,
            town: None,
            region: None,
            postal_code: None,
            country: None,
            facsimile_number: None,
            telephone_number: None,
            electronic_mail_address: None,
            telex_number: None,
            has_internal_location: false,
            has_street_number: false,
            has_street: false,
            has_postal_box: false,
            has_town: false,
            has_region: false,
            has_postal_code: false,
            has_country: false,
            has_facsimile_number: false,
            has_telephone_number: false,
            has_electronic_mail_address: false,
            has_telex_number: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn init(
        &mut self,
        has_internal_location: bool,
        internal_location: Option<Rc<RefCell<HString>>>,
        has_street_number: bool,
        street_number: Option<Rc<RefCell<HString>>>,
        has_street: bool,
        street: Option<Rc<RefCell<HString>>>,
        has_postal_box: bool,
        postal_box: Option<Rc<RefCell<HString>>>,
        has_town: bool,
        town: Option<Rc<RefCell<HString>>>,
        has_region: bool,
        region: Option<Rc<RefCell<HString>>>,
        has_postal_code: bool,
        postal_code: Option<Rc<RefCell<HString>>>,
        has_country: bool,
        country: Option<Rc<RefCell<HString>>>,
        has_facsimile_number: bool,
        facsimile_number: Option<Rc<RefCell<HString>>>,
        has_telephone_number: bool,
        telephone_number: Option<Rc<RefCell<HString>>>,
        has_electronic_mail_address: bool,
        electronic_mail_address: Option<Rc<RefCell<HString>>>,
        has_telex_number: bool,
        telex_number: Option<Rc<RefCell<HString>>>,
    ) {
        self.has_internal_location = has_internal_location;
        self.internal_location = if has_internal_location { internal_location } else { None };
        self.has_street_number = has_street_number;
        self.street_number = if has_street_number { street_number } else { None };
        self.has_street = has_street;
        self.street = if has_street { street } else { None };
        self.has_postal_box = has_postal_box;
        self.postal_box = if has_postal_box { postal_box } else { None };
        self.has_town = has_town;
        self.town = if has_town { town } else { None };
        self.has_region = has_region;
        self.region = if has_region { region } else { None };
        self.has_postal_code = has_postal_code;
        self.postal_code = if has_postal_code { postal_code } else { None };
        self.has_country = has_country;
        self.country = if has_country { country } else { None };
        self.has_facsimile_number = has_facsimile_number;
        self.facsimile_number = if has_facsimile_number { facsimile_number } else { None };
        self.has_telephone_number = has_telephone_number;
        self.telephone_number = if has_telephone_number { telephone_number } else { None };
        self.has_electronic_mail_address = has_electronic_mail_address;
        self.electronic_mail_address = if has_electronic_mail_address {
            electronic_mail_address
        } else {
            None
        };
        self.has_telex_number = has_telex_number;
        self.telex_number = if has_telex_number { telex_number } else { None };
    }

    pub fn set_internal_location(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.internal_location = value.clone();
        self.has_internal_location = value.is_some();
    }

    pub fn unset_internal_location(&mut self) {
        self.internal_location = None;
        self.has_internal_location = false;
    }

    pub fn internal_location(&self) -> Option<Rc<RefCell<HString>>> {
        self.internal_location.clone()
    }

    pub fn has_internal_location(&self) -> bool {
        self.has_internal_location
    }

    pub fn set_street_number(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.street_number = value.clone();
        self.has_street_number = value.is_some();
    }

    pub fn unset_street_number(&mut self) {
        self.street_number = None;
        self.has_street_number = false;
    }

    pub fn street_number(&self) -> Option<Rc<RefCell<HString>>> {
        self.street_number.clone()
    }

    pub fn has_street_number(&self) -> bool {
        self.has_street_number
    }

    pub fn set_street(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.street = value.clone();
        self.has_street = value.is_some();
    }

    pub fn unset_street(&mut self) {
        self.street = None;
        self.has_street = false;
    }

    pub fn street(&self) -> Option<Rc<RefCell<HString>>> {
        self.street.clone()
    }

    pub fn has_street(&self) -> bool {
        self.has_street
    }

    pub fn set_postal_box(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.postal_box = value.clone();
        self.has_postal_box = value.is_some();
    }

    pub fn unset_postal_box(&mut self) {
        self.postal_box = None;
        self.has_postal_box = false;
    }

    pub fn postal_box(&self) -> Option<Rc<RefCell<HString>>> {
        self.postal_box.clone()
    }

    pub fn has_postal_box(&self) -> bool {
        self.has_postal_box
    }

    pub fn set_town(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.town = value.clone();
        self.has_town = value.is_some();
    }

    pub fn unset_town(&mut self) {
        self.town = None;
        self.has_town = false;
    }

    pub fn town(&self) -> Option<Rc<RefCell<HString>>> {
        self.town.clone()
    }

    pub fn has_town(&self) -> bool {
        self.has_town
    }

    pub fn set_region(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.region = value.clone();
        self.has_region = value.is_some();
    }

    pub fn unset_region(&mut self) {
        self.region = None;
        self.has_region = false;
    }

    pub fn region(&self) -> Option<Rc<RefCell<HString>>> {
        self.region.clone()
    }

    pub fn has_region(&self) -> bool {
        self.has_region
    }

    pub fn set_postal_code(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.postal_code = value.clone();
        self.has_postal_code = value.is_some();
    }

    pub fn unset_postal_code(&mut self) {
        self.postal_code = None;
        self.has_postal_code = false;
    }

    pub fn postal_code(&self) -> Option<Rc<RefCell<HString>>> {
        self.postal_code.clone()
    }

    pub fn has_postal_code(&self) -> bool {
        self.has_postal_code
    }

    pub fn set_country(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.country = value.clone();
        self.has_country = value.is_some();
    }

    pub fn unset_country(&mut self) {
        self.country = None;
        self.has_country = false;
    }

    pub fn country(&self) -> Option<Rc<RefCell<HString>>> {
        self.country.clone()
    }

    pub fn has_country(&self) -> bool {
        self.has_country
    }

    pub fn set_facsimile_number(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.facsimile_number = value.clone();
        self.has_facsimile_number = value.is_some();
    }

    pub fn unset_facsimile_number(&mut self) {
        self.facsimile_number = None;
        self.has_facsimile_number = false;
    }

    pub fn facsimile_number(&self) -> Option<Rc<RefCell<HString>>> {
        self.facsimile_number.clone()
    }

    pub fn has_facsimile_number(&self) -> bool {
        self.has_facsimile_number
    }

    pub fn set_telephone_number(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.telephone_number = value.clone();
        self.has_telephone_number = value.is_some();
    }

    pub fn unset_telephone_number(&mut self) {
        self.telephone_number = None;
        self.has_telephone_number = false;
    }

    pub fn telephone_number(&self) -> Option<Rc<RefCell<HString>>> {
        self.telephone_number.clone()
    }

    pub fn has_telephone_number(&self) -> bool {
        self.has_telephone_number
    }

    pub fn set_electronic_mail_address(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.electronic_mail_address = value.clone();
        self.has_electronic_mail_address = value.is_some();
    }

    pub fn unset_electronic_mail_address(&mut self) {
        self.electronic_mail_address = None;
        self.has_electronic_mail_address = false;
    }

    pub fn electronic_mail_address(&self) -> Option<Rc<RefCell<HString>>> {
        self.electronic_mail_address.clone()
    }

    pub fn has_electronic_mail_address(&self) -> bool {
        self.has_electronic_mail_address
    }

    pub fn set_telex_number(&mut self, value: Option<Rc<RefCell<HString>>>) {
        self.telex_number = value.clone();
        self.has_telex_number = value.is_some();
    }

    pub fn unset_telex_number(&mut self) {
        self.telex_number = None;
        self.has_telex_number = false;
    }

    pub fn telex_number(&self) -> Option<Rc<RefCell<HString>>> {
        self.telex_number.clone()
    }

    pub fn has_telex_number(&self) -> bool {
        self.has_telex_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let addr = StepBasic_Address::new();
        assert!(!addr.has_internal_location());
        assert!(!addr.has_street());
        assert!(!addr.has_country());
    }

    #[test]
    fn test_optional_fields() {
        let mut addr = StepBasic_Address::new();
        let street = HString::new("Main St".to_string());
        addr.set_street(Some(street));
        assert!(addr.has_street());
        assert!(addr.street().is_some());

        addr.unset_street();
        assert!(!addr.has_street());
        assert!(addr.street().is_none());
    }

    #[test]
    fn test_init_with_some_fields() {
        let mut addr = StepBasic_Address::new();
        let town = HString::new("Springfield".to_string());
        addr.init(
            true,
            Some(HString::new("Apt 1".to_string())),
            false,
            None,
            false,
            None,
            false,
            None,
            true,
            Some(town),
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
            false,
            None,
        );
        assert!(addr.has_internal_location());
        assert!(addr.has_town());
        assert!(!addr.has_street());
    }
}
