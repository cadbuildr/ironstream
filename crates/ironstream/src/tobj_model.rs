// FILE: tobj_model.rs
// occt: TObj_Model, TObj_Object, TObj_Partition, TObj_TNameContainer,
//       TObj_ObjectIterator

use std::collections::HashMap;

/// Object type classifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TObjType {
    Root,
    Partition,
    Leaf,
    Reference,
}

impl Default for TObjType {
    fn default() -> Self { Self::Leaf }
}

// occt: TObj_Object — base class for all TObj objects
#[derive(Clone, Debug)]
pub struct TObjObject {
    pub id: u32,
    pub name: String,
    pub obj_type: TObjType,
    pub children: Vec<u32>,
    pub references: Vec<u32>,
    pub is_alive: bool,
}

impl TObjObject {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            obj_type: TObjType::Leaf,
            children: Vec::new(),
            references: Vec::new(),
            is_alive: true,
        }
    }

    pub fn add_child(&mut self, child_id: u32) { self.children.push(child_id); }
    pub fn add_reference(&mut self, ref_id: u32) { self.references.push(ref_id); }
    pub fn remove_reference(&mut self, ref_id: u32) { self.references.retain(|&r| r != ref_id); }

    pub fn nb_children(&self) -> usize { self.children.len() }
    pub fn nb_references(&self) -> usize { self.references.len() }
    pub fn has_children(&self) -> bool { !self.children.is_empty() }
    pub fn has_reference(&self, ref_id: u32) -> bool { self.references.contains(&ref_id) }

    pub fn kill(&mut self) { self.is_alive = false; }
    pub fn is_in_model(&self) -> bool { self.is_alive }
}

// occt: TObj_Partition — a named group of objects within a model
#[derive(Clone, Debug)]
pub struct TObjPartition {
    pub id: u32,
    pub name: String,
    pub object_ids: Vec<u32>,
}

impl TObjPartition {
    pub fn new(id: u32, name: &str) -> Self {
        Self { id, name: name.to_owned(), object_ids: Vec::new() }
    }

    pub fn add_object(&mut self, obj_id: u32) { self.object_ids.push(obj_id); }
    pub fn remove_object(&mut self, obj_id: u32) { self.object_ids.retain(|&o| o != obj_id); }
    pub fn nb_objects(&self) -> usize { self.object_ids.len() }
    pub fn is_empty(&self) -> bool { self.object_ids.is_empty() }
    pub fn contains(&self, obj_id: u32) -> bool { self.object_ids.contains(&obj_id) }
}

// occt: TObj_TNameContainer — maps string names to object IDs within a model
#[derive(Clone, Debug, Default)]
pub struct TNameContainer {
    pub names: HashMap<String, u32>,
}

impl TNameContainer {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, name: &str, id: u32) { self.names.insert(name.to_owned(), id); }
    pub fn unbind(&mut self, name: &str) -> bool { self.names.remove(name).is_some() }
    pub fn find(&self, name: &str) -> Option<u32> { self.names.get(name).copied() }
    pub fn is_bound(&self, name: &str) -> bool { self.names.contains_key(name) }
    pub fn nb_names(&self) -> usize { self.names.len() }
    pub fn is_empty(&self) -> bool { self.names.is_empty() }

    pub fn all_names(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.names.keys().map(String::as_str).collect();
        v.sort_unstable();
        v
    }
}

// occt: TObj_Model
#[derive(Clone, Debug, Default)]
pub struct TObjModel {
    pub model_name: String,
    pub objects: HashMap<u32, TObjObject>,
    pub partitions: HashMap<u32, TObjPartition>,
    pub names: TNameContainer,
    pub root_partition_id: Option<u32>,
    next_id: u32,
}

impl TObjModel {
    pub fn new(name: &str) -> Self {
        let mut m = Self { model_name: name.to_owned(), ..Default::default() };
        let root_id = m.create_partition("Root");
        m.root_partition_id = Some(root_id);
        m
    }

    fn next_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id
    }

    pub fn create_object(&mut self, name: &str) -> u32 {
        let id = self.next_id();
        let obj = TObjObject::new(id, name);
        self.names.bind(name, id);
        self.objects.insert(id, obj);
        if let Some(root) = self.root_partition_id {
            if let Some(p) = self.partitions.get_mut(&root) {
                p.add_object(id);
            }
        }
        id
    }

    pub fn create_partition(&mut self, name: &str) -> u32 {
        let id = self.next_id();
        let p = TObjPartition::new(id, name);
        self.partitions.insert(id, p);
        id
    }

    pub fn find_object(&self, name: &str) -> Option<&TObjObject> {
        self.names.find(name).and_then(|id| self.objects.get(&id))
    }

    pub fn find_object_by_id(&self, id: u32) -> Option<&TObjObject> {
        self.objects.get(&id)
    }

    pub fn remove_object(&mut self, id: u32) -> bool {
        if let Some(obj) = self.objects.get_mut(&id) {
            let name = obj.name.clone();
            obj.kill();
            self.names.unbind(&name);
            true
        } else {
            false
        }
    }

    pub fn nb_objects(&self) -> usize { self.objects.values().filter(|o| o.is_alive).count() }
    pub fn nb_partitions(&self) -> usize { self.partitions.len() }
    pub fn is_empty(&self) -> bool { self.nb_objects() == 0 }

    pub fn all_object_ids(&self) -> Vec<u32> {
        self.objects.values().filter(|o| o.is_alive).map(|o| o.id).collect()
    }
}

// occt: TObj_ObjectIterator — iterates live objects in a partition
pub struct TObjObjectIterator<'a> {
    model: &'a TObjModel,
    ids: Vec<u32>,
    index: usize,
}

impl<'a> TObjObjectIterator<'a> {
    pub fn new(model: &'a TObjModel, partition_id: Option<u32>) -> Self {
        let ids: Vec<u32> = if let Some(pid) = partition_id {
            model.partitions.get(&pid)
                .map(|p| p.object_ids.clone())
                .unwrap_or_default()
        } else {
            model.all_object_ids()
        };
        Self { model, ids, index: 0 }
    }

    pub fn more(&self) -> bool { self.index < self.ids.len() }

    pub fn value(&self) -> Option<&TObjObject> {
        self.ids.get(self.index).and_then(|id| self.model.objects.get(id))
    }

    pub fn next(&mut self) { self.index += 1; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_create_find() {
        let mut m = TObjModel::new("TestModel");
        let id = m.create_object("Widget");
        assert_eq!(m.nb_objects(), 1);
        let obj = m.find_object("Widget").unwrap();
        assert_eq!(obj.id, id);
    }

    #[test]
    fn model_remove_object() {
        let mut m = TObjModel::new("M");
        let id = m.create_object("A");
        assert!(m.remove_object(id));
        assert_eq!(m.nb_objects(), 0);
    }

    #[test]
    fn partition_ops() {
        let mut p = TObjPartition::new(1, "Parts");
        p.add_object(10);
        p.add_object(20);
        assert!(p.contains(10));
        p.remove_object(10);
        assert!(!p.contains(10));
        assert_eq!(p.nb_objects(), 1);
    }

    #[test]
    fn name_container() {
        let mut nc = TNameContainer::new();
        nc.bind("alpha", 1);
        nc.bind("beta", 2);
        assert_eq!(nc.find("alpha"), Some(1));
        assert!(nc.unbind("alpha"));
        assert!(!nc.is_bound("alpha"));
    }

    #[test]
    fn object_iterator() {
        let mut m = TObjModel::new("M");
        m.create_object("A");
        m.create_object("B");
        let mut it = TObjObjectIterator::new(&m, None);
        let mut count = 0;
        while it.more() { count += 1; it.next(); }
        assert_eq!(count, 2);
    }
}
