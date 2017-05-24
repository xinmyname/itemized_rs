use std::fmt;
use models::Item;

pub struct Slot {
    pub quantity: i32,
    pub item:Item
}

impl Slot {
    pub fn new(item:Item) -> Slot {
        return Slot { quantity: 1, item: item }
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} items", self.quantity)
    }
}
