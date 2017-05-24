use std::fmt;
use models::Item;

pub struct Slot {
    pub quantity: i32,
    pub item:Item
}

impl Slot {
    pub fn new(item:Item) -> Slot {
        Slot { quantity: 1, item: item }
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let text = if self.quantity == 1 { 
            format!("{}", self.item) 
        } else {
            format!("{}s", self.item)
        };

        let quantity_text = match self.quantity {
            1 => "An".to_string(),
            0 => "No".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            _ => self.quantity.to_string()
        };

        return write!(f, "{} {}", quantity_text, text);
    }
}
