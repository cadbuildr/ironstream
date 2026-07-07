// FILE: step_shape_shape_dimension_representation_item.rs
// occt: StepShape_ShapeDimensionRepresentationItem

use std::sync::Arc;

/// Placeholder types for the various representation item types
pub struct CompoundRepresentationItem {
    id: usize,
}

pub struct DescriptiveRepresentationItem {
    id: usize,
}

pub struct MeasureRepresentationItem {
    id: usize,
}

pub struct Placement {
    id: usize,
}

/// A discriminated union type for shape dimension representation items.
/// Can be one of: CompoundRepresentationItem, DescriptiveRepresentationItem,
/// MeasureRepresentationItem, or Placement.
pub enum ShapeDimensionRepresentationItem {
    /// Case 1: CompoundRepresentationItem
    CompoundRepresentationItem(Arc<CompoundRepresentationItem>),
    /// Case 2: DescriptiveRepresentationItem
    DescriptiveRepresentationItem(Arc<DescriptiveRepresentationItem>),
    /// Case 3: MeasureRepresentationItem
    MeasureRepresentationItem(Arc<MeasureRepresentationItem>),
    /// Case 4: Placement
    Placement(Arc<Placement>),
}

impl ShapeDimensionRepresentationItem {
    /// Create from a CompoundRepresentationItem
    pub fn from_compound(item: Arc<CompoundRepresentationItem>) -> Self {
        ShapeDimensionRepresentationItem::CompoundRepresentationItem(item)
    }

    /// Create from a DescriptiveRepresentationItem
    pub fn from_descriptive(item: Arc<DescriptiveRepresentationItem>) -> Self {
        ShapeDimensionRepresentationItem::DescriptiveRepresentationItem(item)
    }

    /// Create from a MeasureRepresentationItem
    pub fn from_measure(item: Arc<MeasureRepresentationItem>) -> Self {
        ShapeDimensionRepresentationItem::MeasureRepresentationItem(item)
    }

    /// Create from a Placement
    pub fn from_placement(placement: Arc<Placement>) -> Self {
        ShapeDimensionRepresentationItem::Placement(placement)
    }

    /// Get the case number (kind) of this item
    /// 1 -> CompoundRepresentationItem
    /// 2 -> DescriptiveRepresentationItem
    /// 3 -> MeasureRepresentationItem
    /// 4 -> Placement
    pub fn case_num(&self) -> usize {
        match self {
            ShapeDimensionRepresentationItem::CompoundRepresentationItem(_) => 1,
            ShapeDimensionRepresentationItem::DescriptiveRepresentationItem(_) => 2,
            ShapeDimensionRepresentationItem::MeasureRepresentationItem(_) => 3,
            ShapeDimensionRepresentationItem::Placement(_) => 4,
        }
    }

    /// Try to get as a CompoundRepresentationItem
    pub fn as_compound(&self) -> Option<&Arc<CompoundRepresentationItem>> {
        match self {
            ShapeDimensionRepresentationItem::CompoundRepresentationItem(item) => Some(item),
            _ => None,
        }
    }

    /// Try to get as a DescriptiveRepresentationItem
    pub fn as_descriptive(&self) -> Option<&Arc<DescriptiveRepresentationItem>> {
        match self {
            ShapeDimensionRepresentationItem::DescriptiveRepresentationItem(item) => Some(item),
            _ => None,
        }
    }

    /// Try to get as a MeasureRepresentationItem
    pub fn as_measure(&self) -> Option<&Arc<MeasureRepresentationItem>> {
        match self {
            ShapeDimensionRepresentationItem::MeasureRepresentationItem(item) => Some(item),
            _ => None,
        }
    }

    /// Try to get as a Placement
    pub fn as_placement(&self) -> Option<&Arc<Placement>> {
        match self {
            ShapeDimensionRepresentationItem::Placement(item) => Some(item),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_compound() {
        let item = Arc::new(CompoundRepresentationItem { id: 1 });
        let sdri = ShapeDimensionRepresentationItem::from_compound(item);
        assert_eq!(sdri.case_num(), 1);
    }

    #[test]
    fn test_case_num_descriptive() {
        let item = Arc::new(DescriptiveRepresentationItem { id: 2 });
        let sdri = ShapeDimensionRepresentationItem::from_descriptive(item);
        assert_eq!(sdri.case_num(), 2);
    }

    #[test]
    fn test_case_num_measure() {
        let item = Arc::new(MeasureRepresentationItem { id: 3 });
        let sdri = ShapeDimensionRepresentationItem::from_measure(item);
        assert_eq!(sdri.case_num(), 3);
    }

    #[test]
    fn test_case_num_placement() {
        let placement = Arc::new(Placement { id: 4 });
        let sdri = ShapeDimensionRepresentationItem::from_placement(placement);
        assert_eq!(sdri.case_num(), 4);
    }

    #[test]
    fn test_as_compound() {
        let item = Arc::new(CompoundRepresentationItem { id: 10 });
        let sdri = ShapeDimensionRepresentationItem::from_compound(item.clone());
        assert!(sdri.as_compound().is_some());
        assert_eq!(sdri.as_compound().unwrap().id, 10);
        assert!(sdri.as_descriptive().is_none());
    }

    #[test]
    fn test_as_descriptive() {
        let item = Arc::new(DescriptiveRepresentationItem { id: 20 });
        let sdri = ShapeDimensionRepresentationItem::from_descriptive(item.clone());
        assert!(sdri.as_descriptive().is_some());
        assert_eq!(sdri.as_descriptive().unwrap().id, 20);
        assert!(sdri.as_compound().is_none());
    }

    #[test]
    fn test_as_measure() {
        let item = Arc::new(MeasureRepresentationItem { id: 30 });
        let sdri = ShapeDimensionRepresentationItem::from_measure(item.clone());
        assert!(sdri.as_measure().is_some());
        assert_eq!(sdri.as_measure().unwrap().id, 30);
    }

    #[test]
    fn test_as_placement() {
        let placement = Arc::new(Placement { id: 40 });
        let sdri = ShapeDimensionRepresentationItem::from_placement(placement.clone());
        assert!(sdri.as_placement().is_some());
        assert_eq!(sdri.as_placement().unwrap().id, 40);
    }
}
