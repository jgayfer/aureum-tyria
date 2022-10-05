use anyhow::Result;
use async_trait::async_trait;

use crate::{
    api::{
        model::{ItemListings, Listing},
        Client,
    },
    item::{ItemPrice, ListEntry},
};

use super::PricingProvider;

/// A [PricingProvider] that interacts directly with the Guild Wars 2 API.
pub struct HttpPriceSource {
    client: Client,
}

impl HttpPriceSource {
    /// Create a new [HttpPriceSource] with a specific client.
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Default for HttpPriceSource {
    /// Build a ready to use [HttpPriceSource].
    fn default() -> Self {
        Self {
            client: Client::default(),
        }
    }
}

#[async_trait]
impl PricingProvider for HttpPriceSource {
    async fn for_item(&self, item_id: u32) -> Result<ItemPrice> {
        Ok(self.client.item_listings(item_id).await?.into())
    }
}

impl From<ItemListings> for ItemPrice {
    fn from(item_listings: ItemListings) -> Self {
        Self {
            item_id: item_listings.id,
            supply: total_quantity(&item_listings.sells),
            demand: total_quantity(&item_listings.buys),
            buy_price: first_price(&item_listings.buys),
            sell_price: first_price(&item_listings.sells),
            buy_listings: list_entires(item_listings.buys),
            sell_listings: list_entires(item_listings.sells),
        }
    }
}

impl From<Listing> for ListEntry {
    fn from(listing: Listing) -> Self {
        Self {
            listing_count: listing.listings,
            unit_price: listing.unit_price,
            quantity: listing.quantity,
        }
    }
}

/// Get the total quantity of items in a collection of listings.
fn total_quantity(listings: &Vec<Listing>) -> u32 {
    listings.iter().map(|listing| listing.quantity).sum()
}

/// Get the first price from a collection of listings.
/// Listings are sorted by price in the API, so the first value is always what we want.
fn first_price(listings: &Vec<Listing>) -> u32 {
    listings.get(0).map_or(0, |listing| listing.unit_price)
}

/// Map a collection of listings from the API into our "owned" listing models.
fn list_entires(listings: Vec<Listing>) -> Vec<ListEntry> {
    listings.into_iter().map(|listing| listing.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_for_item() {
        let _m = mock("GET", "/commerce/listings/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": 1,
                    "buys": [
                        { "listings": 5, "unit_price":  10, "quantity": 500 },
                        { "listings": 10, "unit_price":  5, "quantity": 250 }
                    ],
                    "sells": [
                        { "listings": 10, "unit_price":  25, "quantity": 10 },
                        { "listings": 15, "unit_price":  50, "quantity": 1 }
                    ]
                }
                "#,
            )
            .create();

        let client = Client::new(mockito::server_url());
        let source = HttpPriceSource::new(client);

        let item_price = source.for_item(1).await.unwrap();

        let expected_item_price = ItemPrice {
            item_id: 1,
            supply: 11,
            demand: 750,
            buy_price: 10,
            sell_price: 25,
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

        assert_eq!(expected_item_price, item_price);
    }

    #[tokio::test]
    async fn test_for_item_no_listings() {
        let _m = mock("GET", "/commerce/listings/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": 1,
                    "buys": [],
                    "sells": []
                }
                "#,
            )
            .create();

        let client = Client::new(mockito::server_url());
        let source = HttpPriceSource::new(client);

        let item_price = source.for_item(1).await.unwrap();

        let expected_item_price = ItemPrice {
            item_id: 1,
            supply: 0,
            demand: 0,
            buy_price: 0,
            sell_price: 0,
            buy_listings: vec![],
            sell_listings: vec![],
        };

        assert_eq!(expected_item_price, item_price);
    }
}
