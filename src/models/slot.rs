use std::fmt;
use models::Item;
use infrastructure::Pluralizer;

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

        let p = Pluralizer::new();
        let text = p.plural_of(self.item.to_string(), self.quantity);
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
