use std::collections::HashMap;
use std::iter::FromIterator;
use models::Item;
use models::Descriptor;
use models::Slot;

pub struct Inventory {
    slots: HashMap<&'static Descriptor, Slot>,
}

impl Inventory {
    pub fn new() -> Inventory {
        return Inventory { slots: HashMap::new() };
    }

    pub fn add_item(&mut self, item: Item) {

        let descriptor = item.descriptor;

        if self.slots.contains_key(descriptor) {

            let slot = self.slots.get_mut(descriptor).unwrap();
            slot.quantity += 1;

        } else {
            self.slots.insert(descriptor, Slot::new(item));
        }
    }

    pub fn slots<'a>(&'a self) -> Vec<&'a Slot> {
        Vec::from_iter(self.slots.values())
    }
}
