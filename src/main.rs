use std::env;

mod infrastructure;
mod models;

use infrastructure::item_factory::ItemFactory;
use models::inventory::Inventory;
use models::descriptor;

fn main() {

    let mut count = 1;

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        
        match args[1].parse() {
            Ok(newCount) => count = newCount,
            _ => panic!("Invalid count.")
        }
    }

    let item_factory = ItemFactory{};
    let inventory = Inventory::new();

    while count > 0 {
        inventory.add_item(item_factory.make_item());
        count -= 1;
    }

    println!("{} items", count);
}
