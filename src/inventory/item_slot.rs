#[derive(Debug, Copy, Clone)]
pub struct ItemSlot {
    pub item_type: Option<ItemType>,
    pub count: Option<u32>,
}

impl ItemSlot {
    pub fn new(item_type: ItemType, count: u32) -> Self {
        ItemSlot {
            item_type: Some(item_type),
            count: Some(count),
        }
    }

    pub fn conveyors(count: u32) -> Self {
        Self::new(ItemType::Conveyor, count)
    }

    pub fn empty() -> Self {
        ItemSlot {
            item_type: Option::None,
            count: Option::None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum ItemType {
    Conveyor,
}
