use std::collections::HashMap;
use std::iter::FromIterator;
use models::item::Item;
use models::descriptor::Descriptor;
use models::slot::Slot;

pub struct Inventory {
    slots: HashMap<&'static Descriptor,Slot>
}

impl Inventory {
    pub fn new() -> Inventory {
        return Inventory { slots: HashMap::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        
        if self.slots.contains_key(item.descriptor) {
            panic!("Yes, it's there.");
        } else {
            let slot = Slot{};
            self.slots.insert(item.descriptor, slot );
        }
    }

    pub fn slots<'a>(&'a self) -> Vec<&'a Slot> {
        return Vec::from_iter(self.slots.values());
    }
}

