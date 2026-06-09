use super::block::{Container, Tickable};
use crate::items::inventory::Inventory;

pub struct Hopper {
    inventory: Inventory,
}

// Constructor
impl Hopper {
    pub fn new() -> Self {
        Hopper {
            inventory: Inventory::new(5),
        }
    }
}

impl Tickable for Hopper {
    fn tick(&mut self) {
        // later
    }
}

impl Container for Hopper {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}
