use anyhow::Result;
use async_trait::async_trait;

use crate::item::ItemPrice;

pub mod http;

#[async_trait]
pub trait PricingProvider {
    /// Fetch the current price information for an item.
    async fn for_item(&self, item_id: u32) -> Result<ItemPrice>;
}
