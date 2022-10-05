#[derive(Debug, PartialEq)]
pub struct ItemPrice {
    pub item_id: u32,
    pub supply: u32,
    pub demand: u32,
    pub buy_price: u32,
    pub sell_price: u32,
    pub buy_listings: Vec<ListEntry>,
    pub sell_listings: Vec<ListEntry>,
}

#[derive(Debug, PartialEq)]
pub struct ListEntry {
    pub listing_count: u32,
    pub unit_price: u32,
    pub quantity: u32,
}
