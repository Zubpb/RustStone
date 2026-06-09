pub struct Item {
    pub name: String,
}

pub const MAX_STACK_SIZE: u8 = 64;
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}
