use crate::items::inventory::Inventory;

pub trait Tickable {
    fn tick(&mut self);
}

pub trait Container {
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;
}
