mod blocks;
mod core;
mod items;

use blocks::block::Container;
use blocks::hopper::Hopper;
use items::item::{Item, ItemStack};

fn make(name: &str, amount: u8) -> ItemStack {
    ItemStack { item: Item { name: name.to_string() }, amount }
}

fn try_insert(hopper: &mut Hopper, item: ItemStack, slot: Option<usize>) {
    let label = match slot {
        Some(s) => format!("slot {}", s),
        None => "any slot".to_string(),
    };
    let name = item.item.name.clone();
    let amount = item.amount;
    match hopper.inventory_mut().insert(item, slot) {
        Ok(()) => println!("[ok] inserted {} x{} into {}", name, amount, label),
        Err(_)  => println!("[err] failed to insert {} x{} into {}", name, amount, label),
    }
}

fn main() {
    let mut hopper = Hopper::new();

    println!("=== hopper created (5 slots) ===");
    hopper.inventory().debug_print();

    println!("\n--- inserting some items ---");
    try_insert(&mut hopper, make("dirt", 10), None);
    try_insert(&mut hopper, make("cobblestone", 64), None);
    try_insert(&mut hopper, make("dirt", 30), None);   // should stack with existing dirt
    try_insert(&mut hopper, make("iron_ingot", 5), None);
    try_insert(&mut hopper, make("iron_ingot", 5), Some(3)); // force into slot 3

    println!("\n--- inventory after inserts ---");
    hopper.inventory().debug_print();

    println!("\n--- overflow test: dump 60 more dirt (only 24 space left in stack) ---");
    try_insert(&mut hopper, make("dirt", 60), None); // 40 fits, 20 remainder spills to new slot

    println!("\n--- inventory after overflow ---");
    hopper.inventory().debug_print();

    println!("\n--- try to overfill hopper (all 5 slots should be taken now) ---");
    try_insert(&mut hopper, make("sand", 1), None); // should fail — no slots left
    try_insert(&mut hopper, make("sand", 1), Some(0)); // should fail — slot 0 has dirt, not sand

    println!("\n--- final state ---");
    hopper.inventory().debug_print();
}
