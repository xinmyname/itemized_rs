use std::env;

mod infrastructure;
mod models;

use infrastructure::ItemFactory;
use models::Inventory;

fn main() {

    let mut count = 1;

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {

        match args[1].parse() {
            Ok(arg_count) => count = arg_count,
            _ => panic!("Invalid count."),
        }
    }

    let item_factory = ItemFactory {};
    let mut inventory = Inventory::new();

    while count > 0 {
        inventory.add_item(item_factory.make_item());
        count -= 1;
    }

    for slot in inventory.slots() {
        println!("{}", slot);
    }
}
