use std::collections::HashMap;

use anyhow::Result;

use crate::item::ItemPrice;

use super::{ItemPriceRecord, PriceStore};

#[derive(Default)]
pub struct InMemoryPriceStore {
    price_map: HashMap<u32, Vec<ItemPriceRecord>>,
}

impl PriceStore for InMemoryPriceStore {
    fn add(&mut self, item_price: ItemPrice) -> Result<ItemPriceRecord> {
        let price_record = ItemPriceRecord {
            item_price: item_price.clone(),
        };

        if let Some(item_prices) = self.price_map.get_mut(&item_price.item_id) {
            item_prices.push(price_record.clone());
        } else {
            self.price_map
                .insert(item_price.item_id, vec![price_record.clone()]);
        }

        Ok(price_record)
    }

    fn for_item(&self, item_id: u32) -> Result<Vec<ItemPriceRecord>> {
        Ok(match self.price_map.get(&item_id) {
            Some(prices) => prices.clone(),
            None => vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item_returns_record() {
        let mut store = InMemoryPriceStore::default();
        let item_price = ItemPrice {
            item_id: 100,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        let record = store.add(item_price).unwrap();

        assert_eq!(100, record.item_price.item_id);
    }

    #[test]
    fn test_add_item_is_saved() {
        let mut store = InMemoryPriceStore::default();
        let item_price = ItemPrice {
            item_id: 100,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        store.add(item_price).unwrap();

        let saved_prices = store.for_item(100).unwrap();

        assert_eq!(100, saved_prices.first().unwrap().item_price.item_id);
    }
}
