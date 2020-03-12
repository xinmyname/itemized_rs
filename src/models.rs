use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

use infrastructure::plural_of;


#[derive(PartialEq, Eq, Hash)]
pub struct Descriptor {}

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

pub struct Item {
    pub descriptor: &'static Descriptor,
}

static DEFAULT_DESCRIPTOR: Descriptor = Descriptor {};

impl Item {
    pub fn new() -> Item {
        Item { descriptor: &DEFAULT_DESCRIPTOR }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item")
    }
}

pub struct Slot {
    pub quantity: i32,
    pub item: Item,
}

impl Slot {
    pub fn new(item: Item) -> Slot {
        Slot {
            quantity: 1,
            item: item,
        }
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let text = plural_of(self.item.to_string(), self.quantity);
        let default_quantity_text = self.quantity.to_string();

        let quantity_text = match self.quantity {
            1 => "An",
            0 => "No",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => &default_quantity_text,
        };

        return write!(f, "{} {}", quantity_text, text);
    }
}
