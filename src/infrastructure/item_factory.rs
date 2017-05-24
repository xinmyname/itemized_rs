use models::Item;

pub struct ItemFactory {

}

impl ItemFactory {
    pub fn make_item(&self) -> Item {
        Item::new()
    }

}
