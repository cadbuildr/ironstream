// FILE: ais_group.rs
// occt-ref: AIS_Group, AIS_MultipleConnectedInteractive

/// Display status for an interactive object in a group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AisGroupDisplayStatus {
    #[default]
    Displayed,
    Erased,
    Neutral,
}

/// A member of an AIS group with an optional local transform.
#[derive(Clone, Debug)]
pub struct AisGroupMember {
    pub object_id: u32,
    pub local_transform: Option<[f64; 16]>,  // row-major 4x4
    pub status: AisGroupDisplayStatus,
}

impl AisGroupMember {
    pub fn new(object_id: u32) -> Self {
        Self { object_id, local_transform: None, status: AisGroupDisplayStatus::Displayed }
    }

    pub fn with_transform(mut self, m: [f64; 16]) -> Self {
        self.local_transform = Some(m);
        self
    }

    pub fn has_transform(&self) -> bool { self.local_transform.is_some() }
}

/// A group of interactive objects managed as one unit.
// occt-ref: AIS_Group
#[derive(Clone, Debug, Default)]
pub struct AisGroup {
    pub members: Vec<AisGroupMember>,
    pub is_displayed: bool,
    pub group_name: String,
}

impl AisGroup {
    pub fn new(name: &str) -> Self {
        Self { members: Vec::new(), is_displayed: true, group_name: name.to_string() }
    }

    pub fn add(&mut self, object_id: u32) {
        if !self.members.iter().any(|m| m.object_id == object_id) {
            self.members.push(AisGroupMember::new(object_id));
        }
    }

    pub fn add_with_transform(&mut self, object_id: u32, m: [f64; 16]) {
        if let Some(member) = self.members.iter_mut().find(|m| m.object_id == object_id) {
            member.local_transform = Some(m);
        } else {
            self.members.push(AisGroupMember::new(object_id).with_transform(m));
        }
    }

    pub fn remove(&mut self, object_id: u32) {
        self.members.retain(|m| m.object_id != object_id);
    }

    pub fn contains(&self, object_id: u32) -> bool {
        self.members.iter().any(|m| m.object_id == object_id)
    }

    pub fn nb_objects(&self) -> usize { self.members.len() }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }

    pub fn displayed_ids(&self) -> Vec<u32> {
        self.members.iter()
            .filter(|m| m.status == AisGroupDisplayStatus::Displayed)
            .map(|m| m.object_id)
            .collect()
    }
}

/// Connects multiple shapes at different positions via local transforms.
// occt-ref: AIS_MultipleConnectedInteractive
#[derive(Clone, Debug, Default)]
pub struct AisMultipleConnectedInteractive {
    pub base_object_id: u32,
    pub instances: Vec<(u32, [f64; 16])>,    // (instance_id, 4x4 transform)
    pub next_id: u32,
}

impl AisMultipleConnectedInteractive {
    pub fn new(base_object_id: u32) -> Self {
        Self { base_object_id, instances: Vec::new(), next_id: 1 }
    }

    /// Create a new instance with a translation offset (simplified).
    pub fn connect(&mut self, translation: [f64; 3]) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut m = identity4();
        m[3] = translation[0];
        m[7] = translation[1];
        m[11] = translation[2];
        self.instances.push((id, m));
        id
    }

    pub fn remove_instance(&mut self, instance_id: u32) {
        self.instances.retain(|(id, _)| *id != instance_id);
    }

    pub fn nb_instances(&self) -> usize { self.instances.len() }

    pub fn instance_transform(&self, instance_id: u32) -> Option<&[f64; 16]> {
        self.instances.iter().find(|(id, _)| *id == instance_id).map(|(_, m)| m)
    }
}

fn identity4() -> [f64; 16] {
    [1.0,0.0,0.0,0.0,
     0.0,1.0,0.0,0.0,
     0.0,0.0,1.0,0.0,
     0.0,0.0,0.0,1.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_add_remove() {
        let mut g = AisGroup::new("MyGroup");
        g.add(1);
        g.add(2);
        g.add(1); // dup
        assert_eq!(g.nb_objects(), 2);
        assert!(g.contains(1));
        g.remove(1);
        assert!(!g.contains(1));
        assert_eq!(g.nb_objects(), 1);
    }

    #[test]
    fn group_display_erase() {
        let mut g = AisGroup::new("G");
        assert!(g.is_displayed);
        g.erase();
        assert!(!g.is_displayed);
        g.display();
        assert!(g.is_displayed);
    }

    #[test]
    fn group_with_transform() {
        let mut g = AisGroup::new("G");
        let m = identity4();
        g.add_with_transform(10, m);
        assert!(g.members[0].has_transform());
    }

    #[test]
    fn multiple_connected_connect() {
        let mut mc = AisMultipleConnectedInteractive::new(100);
        let i1 = mc.connect([1.0, 0.0, 0.0]);
        let i2 = mc.connect([0.0, 2.0, 0.0]);
        assert_eq!(mc.nb_instances(), 2);
        let t = mc.instance_transform(i1).unwrap();
        assert!((t[3] - 1.0).abs() < 1e-10);
        let t2 = mc.instance_transform(i2).unwrap();
        assert!((t2[7] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn multiple_connected_remove() {
        let mut mc = AisMultipleConnectedInteractive::new(5);
        let i1 = mc.connect([0.0, 0.0, 0.0]);
        mc.connect([1.0, 0.0, 0.0]);
        mc.remove_instance(i1);
        assert_eq!(mc.nb_instances(), 1);
    }
}
