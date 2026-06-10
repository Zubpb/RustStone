use super::block::Container;
use crate::items::inventory::Inventory;

pub struct Chest {
    inventory: Inventory,
}

impl Chest {
    pub fn new() -> Self {
        Chest {
            inventory: Inventory::new(27),
        }
    }
}

impl Container for Chest {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}
