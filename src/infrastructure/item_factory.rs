use models::item::Item;

pub struct ItemFactory {

}

impl ItemFactory {
    pub fn make_item(&self) -> Item {
        return Item::new()
    }

}
