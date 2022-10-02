use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemPrice {
    pub id: u32,
    pub whitelisted: bool,
    pub buys: PriceTotal,
    pub sells: PriceTotal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PriceTotal {
    pub quantity: u32,
    pub unit_price: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemListings {
    pub id: u32,
    pub buys: Vec<Listing>,
    pub sells: Vec<Listing>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Listing {
    pub listings: u32,
    pub unit_price: u32,
    pub quantity: u32,
}
