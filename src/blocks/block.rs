use crate::items::inventory::Inventory;
use super::hopper::Hopper;
use super::chest::Chest;

pub trait Container {
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;
}

pub enum AnyBlock {
    Hopper(Hopper),
    Chest(Chest),
}

impl AnyBlock {
    pub fn as_container(&self) -> &dyn Container {
        match self {
            AnyBlock::Hopper(h) => h,
            AnyBlock::Chest(c)  => c,
        }
    }

    pub fn as_container_mut(&mut self) -> &mut dyn Container {
        match self {
            AnyBlock::Hopper(h) => h,
            AnyBlock::Chest(c)  => c,
        }
    }

    pub fn as_hopper_mut(&mut self) -> Option<&mut Hopper> {
        match self {
            AnyBlock::Hopper(h) => Some(h),
            _ => None,
        }
    }
}
