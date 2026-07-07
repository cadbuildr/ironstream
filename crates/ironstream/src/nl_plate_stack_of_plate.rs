// FILE: nl_plate_stack_of_plate.rs
// occt: NLPlate_StackOfPlate, NLPlate_ListIteratorOfStackOfPlate

/// Plate_Plate represents a plate surface element.
#[derive(Clone, Debug)]
pub struct PlatePlate {
    id: i32,
    width: f64,
    height: f64,
    thickness: f64,
}

impl PlatePlate {
    pub fn new(id: i32, width: f64, height: f64, thickness: f64) -> Self {
        PlatePlate {
            id,
            width,
            height,
            thickness,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn volume(&self) -> f64 {
        self.width * self.height * self.thickness
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_List<Plate_Plate>`
pub type NlplateStackOfPlate = Vec<PlatePlate>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_List<Plate_Plate>::Iterator`
pub type NlplateListIteratorOfStackOfPlate = std::vec::IntoIter<PlatePlate>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plate_creation() {
        let plate = PlatePlate::new(1, 10.0, 20.0, 2.0);
        assert_eq!(plate.id(), 1);
        assert_eq!(plate.width(), 10.0);
        assert_eq!(plate.height(), 20.0);
        assert_eq!(plate.thickness(), 2.0);
    }

    #[test]
    fn test_plate_area() {
        let plate = PlatePlate::new(1, 5.0, 4.0, 1.0);
        assert!((plate.area() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_plate_volume() {
        let plate = PlatePlate::new(1, 5.0, 4.0, 2.0);
        assert!((plate.volume() - 40.0).abs() < 1e-10);
    }

    #[test]
    fn test_stack_creation() {
        let stack: NlplateStackOfPlate = Vec::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_push() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        let plate1 = PlatePlate::new(1, 10.0, 20.0, 2.0);
        let plate2 = PlatePlate::new(2, 15.0, 25.0, 3.0);

        stack.push(plate1.clone());
        stack.push(plate2.clone());

        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0].id(), 1);
        assert_eq!(stack[1].id(), 2);
    }

    #[test]
    fn test_stack_access() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        let plate = PlatePlate::new(42, 5.0, 10.0, 1.5);
        stack.push(plate);

        let retrieved = stack.get(0).unwrap();
        assert_eq!(retrieved.id(), 42);
        assert_eq!(retrieved.width(), 5.0);
    }

    #[test]
    fn test_stack_iteration() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        for i in 1..=5 {
            let plate = PlatePlate::new(i, i as f64 * 10.0, i as f64 * 20.0, i as f64);
            stack.push(plate);
        }

        assert_eq!(stack.len(), 5);

        let mut ids = Vec::new();
        for plate in &stack {
            ids.push(plate.id());
        }
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_stack_remove() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        stack.push(PlatePlate::new(1, 10.0, 20.0, 2.0));
        stack.push(PlatePlate::new(2, 15.0, 25.0, 3.0));
        stack.push(PlatePlate::new(3, 20.0, 30.0, 4.0));

        assert_eq!(stack.len(), 3);
        stack.remove(1);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0].id(), 1);
        assert_eq!(stack[1].id(), 3);
    }

    #[test]
    fn test_stack_total_volume() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        stack.push(PlatePlate::new(1, 5.0, 5.0, 1.0));
        stack.push(PlatePlate::new(2, 10.0, 10.0, 2.0));

        let total_volume: f64 = stack.iter().map(|p| p.volume()).sum();
        assert!((total_volume - 225.0).abs() < 1e-10);
    }

    #[test]
    fn test_stack_iterator() {
        let mut stack: NlplateStackOfPlate = Vec::new();

        stack.push(PlatePlate::new(1, 5.0, 10.0, 1.0));
        stack.push(PlatePlate::new(2, 6.0, 12.0, 2.0));

        let into_iter: NlplateListIteratorOfStackOfPlate = stack.into_iter();
        let collected: Vec<PlatePlate> = into_iter.collect();

        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].id(), 1);
        assert_eq!(collected[1].id(), 2);
    }
}
