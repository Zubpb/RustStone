#![allow(dead_code)]

use super::item::{Item, ItemStack, MAX_STACK_SIZE};
use std::collections::HashMap;

pub enum InventoryError {
    SlotOccupied,
    NoAvailableSlots,
}

pub struct Inventory {
    slots: HashMap<usize, ItemStack>,
    max_slots: usize,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Inventory {
            slots: HashMap::new(),
            max_slots,
        }
    }

    pub fn insert(
        &mut self,
        item: ItemStack,
        slot_index: Option<usize>,
    ) -> Result<(), InventoryError> {
        match slot_index {
            // Used specifies slot
            Some(slot_index) => {
                match self.slots.get(&slot_index) {
                    // Specified slot has this item already append it and add remainder
                    Some(existing) if existing.item.name == item.item.name => {
                        let existing = self.slots.get_mut(&slot_index).unwrap();
                        let space = MAX_STACK_SIZE - existing.amount;

                        if space >= item.amount {
                            existing.amount += item.amount;
                            Ok(())
                        } else {
                            existing.amount = MAX_STACK_SIZE;
                            let remainder = ItemStack {
                                amount: item.amount - space,
                                ..item
                            };
                            return self.insert(remainder, None);
                        }
                    }

                    // Specified slot is fully free
                    None => {
                        self.slots.insert(slot_index, item);
                        Ok(())
                    }

                    // Specified slot has something already?
                    _ => return Err(InventoryError::SlotOccupied),
                }
            }

            // User doesnt specify slot
            None => match self.find_valid_slot(&item) {
                Some(slot_index) => return self.insert(item, Some(slot_index)),
                None => return Err(InventoryError::NoAvailableSlots),
            },
        }
    }

    pub fn remove(&mut self, slot_index: usize, amount: u8) {
        // later
    }

    pub fn transfer_to(&mut self, other: &mut Inventory) {
        // later
    }

    fn find_valid_slot(&self, item: &ItemStack) -> Option<usize> {
        let mut first_free: Option<usize> = None;

        for i in 0..self.max_slots {
            match self.slots.get(&i) {
                Some(existing)
                    if existing.item.name == item.item.name && existing.amount < MAX_STACK_SIZE =>
                {
                    return Some(i);
                }
                None if first_free.is_none() => first_free = Some(i),
                _ => {}
            }
        }

        first_free
    }
}
