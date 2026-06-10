use super::block::Container;
use crate::items::inventory::Inventory;

pub struct Hopper {
    inventory: Inventory,
    pub output_id: Option<usize>,
    pub overflow_id: Option<usize>,
    pub powered: bool,
    pub filter: Option<String>,
}

impl Hopper {
    pub fn new() -> Self {
        Hopper {
            inventory: Inventory::new(5),
            output_id: None,
            overflow_id: None,
            powered: false,
            filter: None,
        }
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
