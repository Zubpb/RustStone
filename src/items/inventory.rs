use super::item::{Item, ItemStack, MAX_STACK_SIZE};
use std::collections::HashMap;

pub enum InventoryError {
    Full,
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

    pub fn insert(&mut self, item: ItemStack) -> Result<(), InventoryError> {
        let slot = self.find_slot(&item).ok_or(InventoryError::Full)?;

        if let Some(existing) = self.slots.get_mut(&slot) {
            let space = MAX_STACK_SIZE - existing.amount;
            existing.amount += item.amount.min(space);

            if item.amount > space {
                let remainder = ItemStack { amount: item.amount - space, ..item };
                return self.insert(remainder);
            }
        } else {
            self.slots.insert(slot, item);
        }

        Ok(())
    }

    pub fn peek_first_name(&self) -> Option<&str> {
        for i in 0..self.max_slots {
            if let Some(stack) = self.slots.get(&i) {
                return Some(&stack.item.name);
            }
        }
        None
    }

    pub fn take_one(&mut self, name: &str) -> Option<ItemStack> {
        for i in 0..self.max_slots {
            if let Some(stack) = self.slots.get_mut(&i) {
                if stack.item.name == name {
                    stack.amount -= 1;
                    let taken = ItemStack { item: Item { name: name.to_string() }, amount: 1 };
                    if stack.amount == 0 {
                        self.slots.remove(&i);
                    }
                    return Some(taken);
                }
            }
        }
        None
    }

    pub fn contents_line(&self) -> String {
        let mut parts = Vec::new();
        for i in 0..self.max_slots {
            if let Some(stack) = self.slots.get(&i) {
                parts.push(format!("{} x{}", stack.item.name, stack.amount));
            }
        }
        parts.join(", ")
    }

    pub fn total(&self, item_name: &str) -> u32 {
        self.slots.values()
            .filter(|s| s.item.name == item_name)
            .map(|s| s.amount as u32)
            .sum()
    }

    fn find_slot(&self, item: &ItemStack) -> Option<usize> {
        let mut first_free: Option<usize> = None;

        for i in 0..self.max_slots {
            match self.slots.get(&i) {
                Some(s) if s.item.name == item.item.name && s.amount < MAX_STACK_SIZE => {
                    return Some(i);
                }
                None if first_free.is_none() => {
                    first_free = Some(i);
                }
                _ => {}
            }
        }

        first_free
    }
}
