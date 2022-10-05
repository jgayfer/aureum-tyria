use anyhow::Result;

use super::model::{ItemListings, ItemPrice};

const BASE_URL: &str = "https://api.guildwars2.com/v2";

/// An API client for interacting with the Guild Wars 2 API.
pub struct Client {
    base_url: String,
}

impl Default for Client {
    /// Build a ready to use [Client].
    fn default() -> Self {
        Self {
            base_url: BASE_URL.to_string(),
        }
    }
}

impl Client {
    /// Build a new [Client] with a different base URL.
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Get current buy and sell listings for an item from the trading post.
    pub async fn item_listings(&self, item_id: u32) -> Result<ItemListings> {
        let url = format!("{}/commerce/listings/{}", self.base_url, item_id);
        Ok(reqwest::get(url).await?.json::<ItemListings>().await?)
    }

    /// Get current prices for an item from the trading post.
    ///
    /// For more detailed information about the distribution of list prices, use
    /// [Client::item_listings].
    pub async fn item_prices(&self, item_id: u32) -> Result<ItemPrice> {
        let url = format!("{}/commerce/prices/{}", self.base_url, item_id);
        Ok(reqwest::get(url).await?.json::<ItemPrice>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::model::{Listing, PriceTotal};
    use mockito::mock;

    #[tokio::test]
    async fn test_item_listings() {
        let _m = mock("GET", "/commerce/listings/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": 1,
                    "buys": [
                        { "listings": 5, "unit_price":  10, "quantity": 500 }
                    ],
                    "sells": [
                        { "listings": 12, "unit_price":  25, "quantity": 1000 }
                    ]
                }
                "#,
            )
            .create();

        let item_listings = Client::new(mockito::server_url())
            .item_listings(1)
            .await
            .unwrap();

        let expected_listings = ItemListings {
            id: 1,
            buys: vec![Listing {
                listings: 5,
                unit_price: 10,
                quantity: 500,
            }],
            sells: vec![Listing {
                listings: 12,
                unit_price: 25,
                quantity: 1000,
            }],
        };

        assert_eq!(expected_listings, item_listings);
    }

    #[tokio::test]
    async fn test_item_prices() {
        let _m = mock("GET", "/commerce/prices/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": 1,
                    "whitelisted": false,
                    "buys": {
                        "quantity": 100,
                        "unit_price": 50
                    },
                    "sells": {
                        "quantity": 200,
                        "unit_price": 60
                    }
                }
                "#,
            )
            .create();

        let item = Client::new(mockito::server_url())
            .item_prices(1)
            .await
            .unwrap();

        let expected_item = ItemPrice {
            id: 1,
            whitelisted: false,
            buys: PriceTotal {
                quantity: 100,
                unit_price: 50,
            },
            sells: PriceTotal {
                quantity: 200,
                unit_price: 60,
            },
        };

        assert_eq!(expected_item, item);
    }
}
