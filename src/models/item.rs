use std::fmt;
use models::Descriptor;

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
