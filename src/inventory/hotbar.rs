use crate::global_constants::HOTBAR_LENGTH;

#[derive(Debug)]
pub struct Hotbar {
    pub items: [HotbarItemSlot; HOTBAR_LENGTH],
}

impl Hotbar {
    pub fn new(items: [HotbarItemSlot; HOTBAR_LENGTH]) -> Self {
        Hotbar { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Default for Hotbar {
    fn default() -> Self {
        let mut items = [HotbarItemSlot::empty(); HOTBAR_LENGTH];
        items[0] = HotbarItemSlot::new(Some(0));
        Hotbar::new(items)
    }
}

pub struct HotbarIndex {
    index: usize,
}

impl HotbarIndex {
    pub fn new(index: usize) -> Self {
        HotbarIndex { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HotbarItemSlot {
    inventory_index: Option<usize>,
}

impl HotbarItemSlot {
    pub fn new(inventory_index: Option<usize>) -> Self {
        HotbarItemSlot { inventory_index }
    }

    pub fn empty() -> Self {
        HotbarItemSlot::new(Option::None)
    }

    pub fn index(&self) -> Option<usize> {
        self.inventory_index
    }
}
