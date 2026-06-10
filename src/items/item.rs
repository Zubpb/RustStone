#[derive(Clone)]
pub struct Item {
    pub name: String,
}

#[derive(Clone)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}

pub const MAX_STACK_SIZE: u8 = 64;
