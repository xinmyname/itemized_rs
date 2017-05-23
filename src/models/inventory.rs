use std::collections::HashMap;
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

    pub fn add_item(&self, item: Item) {
        
        if self.slots.contains_key(item.descriptor) {
            panic!("Yes, it's there.");
        } else {
            self.slots.insert(item.descriptor, Slot {} )
        }
    }
}

