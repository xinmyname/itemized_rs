use std::fmt;
use models::descriptor::Descriptor;

pub struct Item {
    pub descriptor: &'static Descriptor
}

static defaultDescriptor:Descriptor = Descriptor{};

impl Item {
    pub fn new() -> Item {
        return Item { descriptor: &defaultDescriptor }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item")
    }
}
