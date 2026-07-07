// FILE: step_ap214_auto_design_date_and_person_item.rs
// occt: StepAP214_AutoDesignDateAndPersonItem

/// A SelectType that can hold one of several STEP entity types related to dates and persons.
/// This corresponds to the OCCT StepAP214_AutoDesignDateAndPersonItem class.
#[derive(Clone, Debug)]
pub enum StepAP214AutoDesignDateAndPersonItem {
    /// Case 1: AutoDesignOrganizationAssignment
    AutoDesignOrganizationAssignment(usize),
    /// Case 2: Product
    Product(usize),
    /// Case 3: ProductDefinition
    ProductDefinition(usize),
    /// Case 4: ProductDefinitionFormation
    ProductDefinitionFormation(usize),
    /// Case 5: Representation
    Representation(usize),
    /// Case 6: AutoDesignDocumentReference
    AutoDesignDocumentReference(usize),
    /// Case 7: ExternallyDefinedRepresentation
    ExternallyDefinedRepresentation(usize),
    /// Case 8: ProductDefinitionRelationship
    ProductDefinitionRelationship(usize),
    /// Case 9: ProductDefinitionWithAssociatedDocuments
    ProductDefinitionWithAssociatedDocuments(usize),
}

impl StepAP214AutoDesignDateAndPersonItem {
    /// Creates a new SelectType instance.
    pub fn new() -> Self {
        // Default to a null state
        StepAP214AutoDesignDateAndPersonItem::Product(0)
    }

    /// Returns the case number based on the stored variant.
    /// 1-9 correspond to the variants, 0 for null/unknown.
    pub fn case_num(&self) -> i32 {
        match self {
            StepAP214AutoDesignDateAndPersonItem::AutoDesignOrganizationAssignment(_) => 1,
            StepAP214AutoDesignDateAndPersonItem::Product(_) => 2,
            StepAP214AutoDesignDateAndPersonItem::ProductDefinition(_) => 3,
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionFormation(_) => 4,
            StepAP214AutoDesignDateAndPersonItem::Representation(_) => 5,
            StepAP214AutoDesignDateAndPersonItem::AutoDesignDocumentReference(_) => 6,
            StepAP214AutoDesignDateAndPersonItem::ExternallyDefinedRepresentation(_) => 7,
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionRelationship(_) => 8,
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionWithAssociatedDocuments(_) => 9,
        }
    }

    /// Extract AutoDesignOrganizationAssignment (case 1)
    pub fn auto_design_organization_assignment(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::AutoDesignOrganizationAssignment(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract Product (case 2)
    pub fn product(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::Product(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract ProductDefinition (case 3)
    pub fn product_definition(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::ProductDefinition(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract ProductDefinitionFormation (case 4)
    pub fn product_definition_formation(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionFormation(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract Representation (case 5)
    pub fn representation(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::Representation(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract AutoDesignDocumentReference (case 6)
    pub fn auto_design_document_reference(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::AutoDesignDocumentReference(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract ExternallyDefinedRepresentation (case 7)
    pub fn externally_defined_representation(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::ExternallyDefinedRepresentation(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract ProductDefinitionRelationship (case 8)
    pub fn product_definition_relationship(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionRelationship(val) => Some(*val),
            _ => None,
        }
    }

    /// Extract ProductDefinitionWithAssociatedDocuments (case 9)
    pub fn product_definition_with_associated_documents(&self) -> Option<usize> {
        match self {
            StepAP214AutoDesignDateAndPersonItem::ProductDefinitionWithAssociatedDocuments(val) => {
                Some(*val)
            }
            _ => None,
        }
    }
}

impl Default for StepAP214AutoDesignDateAndPersonItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let item = StepAP214AutoDesignDateAndPersonItem::AutoDesignOrganizationAssignment(42);
        assert_eq!(item.case_num(), 1);

        let item = StepAP214AutoDesignDateAndPersonItem::Product(10);
        assert_eq!(item.case_num(), 2);

        let item = StepAP214AutoDesignDateAndPersonItem::ProductDefinition(20);
        assert_eq!(item.case_num(), 3);

        let item = StepAP214AutoDesignDateAndPersonItem::ExternallyDefinedRepresentation(99);
        assert_eq!(item.case_num(), 7);

        let item = StepAP214AutoDesignDateAndPersonItem::ProductDefinitionWithAssociatedDocuments(88);
        assert_eq!(item.case_num(), 9);
    }

    #[test]
    fn test_extraction() {
        let item = StepAP214AutoDesignDateAndPersonItem::Product(123);
        assert_eq!(item.product(), Some(123));
        assert_eq!(item.auto_design_organization_assignment(), None);
        assert_eq!(item.product_definition(), None);

        let item = StepAP214AutoDesignDateAndPersonItem::Representation(456);
        assert_eq!(item.representation(), Some(456));
        assert_eq!(item.case_num(), 5);
    }

    #[test]
    fn test_default() {
        let item = StepAP214AutoDesignDateAndPersonItem::default();
        assert_eq!(item.case_num(), 2); // defaults to Product
    }
}
