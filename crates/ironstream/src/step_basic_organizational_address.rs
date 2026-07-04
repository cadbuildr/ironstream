// FILE: step_basic_organizational_address.rs
// occt: StepBasic_OrganizationalAddress

/// Represents a STEP OrganizationalAddress entity extending Address with Organizations and Description.
#[derive(Clone, Debug)]
pub struct StepBasicOrganizationalAddress {
    // Address fields (inherited)
    internal_location: Option<String>,
    street_number: Option<String>,
    street: Option<String>,
    postal_box: Option<String>,
    town: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    facsimile_number: Option<String>,
    telephone_number: Option<String>,
    electronic_mail_address: Option<String>,
    telex_number: Option<String>,
    // OrganizationalAddress fields
    organizations: Vec<String>, // Simplified: using IDs
    description: String,
}

impl StepBasicOrganizationalAddress {
    /// Create a new empty StepBasicOrganizationalAddress.
    pub fn new() -> Self {
        StepBasicOrganizationalAddress {
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
            organizations: Vec::new(),
            description: String::new(),
        }
    }

    /// Initialize all fields.
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        &mut self,
        internal_location: Option<String>,
        street_number: Option<String>,
        street: Option<String>,
        postal_box: Option<String>,
        town: Option<String>,
        region: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        facsimile_number: Option<String>,
        telephone_number: Option<String>,
        electronic_mail_address: Option<String>,
        telex_number: Option<String>,
        organizations: Vec<String>,
        description: String,
    ) {
        self.internal_location = internal_location;
        self.street_number = street_number;
        self.street = street;
        self.postal_box = postal_box;
        self.town = town;
        self.region = region;
        self.postal_code = postal_code;
        self.country = country;
        self.facsimile_number = facsimile_number;
        self.telephone_number = telephone_number;
        self.electronic_mail_address = electronic_mail_address;
        self.telex_number = telex_number;
        self.organizations = organizations;
        self.description = description;
    }

    /// Returns the Organizations list.
    pub fn organizations(&self) -> &[String] {
        &self.organizations
    }

    /// Set the Organizations list.
    pub fn set_organizations(&mut self, organizations: Vec<String>) {
        self.organizations = organizations;
    }

    /// Returns a specific organization by index.
    pub fn organizations_value(&self, index: usize) -> Option<&str> {
        self.organizations.get(index).map(|s| s.as_str())
    }

    /// Returns the number of organizations.
    pub fn nb_organizations(&self) -> usize {
        self.organizations.len()
    }

    /// Returns the Description field.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set the Description field.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns internal location.
    pub fn internal_location(&self) -> Option<&str> {
        self.internal_location.as_deref()
    }

    /// Set internal location.
    pub fn set_internal_location(&mut self, location: String) {
        self.internal_location = Some(location);
    }

    /// Returns street.
    pub fn street(&self) -> Option<&str> {
        self.street.as_deref()
    }

    /// Set street.
    pub fn set_street(&mut self, street: String) {
        self.street = Some(street);
    }

    /// Returns town.
    pub fn town(&self) -> Option<&str> {
        self.town.as_deref()
    }

    /// Set town.
    pub fn set_town(&mut self, town: String) {
        self.town = Some(town);
    }
}

impl Default for StepBasicOrganizationalAddress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let addr = StepBasicOrganizationalAddress::new();
        assert_eq!(addr.nb_organizations(), 0);
        assert_eq!(addr.description(), "");
    }

    #[test]
    fn test_init() {
        let mut addr = StepBasicOrganizationalAddress::new();
        addr.init(
            Some("Room 101".to_string()),
            Some("42".to_string()),
            Some("Main St".to_string()),
            None,
            Some("Springfield".to_string()),
            None,
            Some("12345".to_string()),
            Some("USA".to_string()),
            None,
            Some("555-1234".to_string()),
            None,
            None,
            vec!["ORG-001".to_string(), "ORG-002".to_string()],
            "Main office".to_string(),
        );

        assert_eq!(addr.internal_location(), Some("Room 101"));
        assert_eq!(addr.street(), Some("Main St"));
        assert_eq!(addr.town(), Some("Springfield"));
        assert_eq!(addr.nb_organizations(), 2);
        assert_eq!(addr.organizations_value(0), Some("ORG-001"));
        assert_eq!(addr.description(), "Main office");
    }

    #[test]
    fn test_set_organizations() {
        let mut addr = StepBasicOrganizationalAddress::new();
        addr.set_organizations(vec!["ORG-A".to_string(), "ORG-B".to_string()]);

        assert_eq!(addr.nb_organizations(), 2);
        assert_eq!(addr.organizations_value(1), Some("ORG-B"));
    }

    #[test]
    fn test_setters() {
        let mut addr = StepBasicOrganizationalAddress::new();
        addr.set_internal_location("Office".to_string());
        addr.set_street("Oak Ave".to_string());
        addr.set_town("Boston".to_string());
        addr.set_description("Headquarters".to_string());

        assert_eq!(addr.internal_location(), Some("Office"));
        assert_eq!(addr.street(), Some("Oak Ave"));
        assert_eq!(addr.town(), Some("Boston"));
        assert_eq!(addr.description(), "Headquarters");
    }
}
