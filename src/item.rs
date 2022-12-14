#[derive(Clone, Debug, PartialEq)]
pub struct ItemPrice {
    pub item_id: u32,
    pub buy_listings: Vec<ListEntry>,
    pub sell_listings: Vec<ListEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListEntry {
    pub listing_count: u32,
    pub unit_price: u32,
    pub quantity: u32,
}

impl ItemPrice {
    /// Total number of units for sale.
    pub fn supply(&self) -> u32 {
        self.sell_listings
            .iter()
            .map(|listing| listing.quantity)
            .sum()
    }

    /// Total number of units ordered.
    pub fn demand(&self) -> u32 {
        self.buy_listings
            .iter()
            .map(|listing| listing.quantity)
            .sum()
    }

    /// Highest buy order price for the item.
    pub fn buy_price(&self) -> u32 {
        self.buy_listings
            .iter()
            .map(|listing| listing.unit_price)
            .max()
            .unwrap_or(0)
    }

    /// Lowest sell order price for the item.
    pub fn sell_price(&self) -> u32 {
        self.sell_listings
            .iter()
            .map(|listing| listing.unit_price)
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![
                ListEntry {
                    listing_count: 10,
                    unit_price: 25,
                    quantity: 10,
                },
                ListEntry {
                    listing_count: 15,
                    unit_price: 50,
                    quantity: 1,
                },
            ],
        };

        assert_eq!(11, item_price.supply());
    }

    #[test]
    fn test_supply_no_sells() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        assert_eq!(0, item_price.supply());
    }

    #[test]
    fn test_demand() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![
                ListEntry {
                    listing_count: 5,
                    unit_price: 10,
                    quantity: 500,
                },
                ListEntry {
                    listing_count: 10,
                    unit_price: 5,
                    quantity: 250,
                },
            ],
            sell_listings: vec![],
        };

        assert_eq!(750, item_price.demand());
    }

    #[test]
    fn test_demand_no_buys() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        assert_eq!(0, item_price.demand());
    }

    #[test]
    fn test_sell_price() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![
                ListEntry {
                    listing_count: 10,
                    unit_price: 25,
                    quantity: 10,
                },
                ListEntry {
                    listing_count: 15,
                    unit_price: 50,
                    quantity: 1,
                },
            ],
        };

        assert_eq!(25, item_price.sell_price());
    }

    #[test]
    fn test_sell_price_no_listings() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        assert_eq!(0, item_price.sell_price());
    }

    #[test]
    fn test_buy_price() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![
                ListEntry {
                    listing_count: 5,
                    unit_price: 10,
                    quantity: 500,
                },
                ListEntry {
                    listing_count: 10,
                    unit_price: 5,
                    quantity: 250,
                },
            ],
            sell_listings: vec![],
        };

        assert_eq!(10, item_price.buy_price());
    }

    #[test]
    fn test_buy_price_no_buys() {
        let item_price = ItemPrice {
            item_id: 1,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        assert_eq!(0, item_price.buy_price());
    }
}
