use anyhow::Result;

use crate::item::ItemPrice;

pub mod in_memory;

/// Represents a saved [ItemPrice].
#[derive(Debug, Clone)]
pub struct ItemPriceRecord {
    pub item_price: ItemPrice,
}

/// General purpose store for item price data.
pub trait PriceStore {
    /// Add an item price to the store.
    fn add(&mut self, item_price: ItemPrice) -> Result<ItemPriceRecord>;

    /// Get all price records for an item by its ID.
    fn for_item(&self, item_id: u32) -> Result<Vec<ItemPriceRecord>>;
}
