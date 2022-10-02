use anyhow::Result;
use serde::Deserialize;

const BASE_URL: &str = "https://api.guildwars2.com/v2";

pub struct Client {
    base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.to_string(),
        }
    }
}

impl Client {
    fn new(base_url: String) -> Self {
        Self { base_url }
    }

    async fn item_prices(self, item_id: u32) -> Result<ItemPrice> {
        let url = format!("{}/commerce/prices/{}", self.base_url, item_id);
        Ok(reqwest::get(url).await?.json::<ItemPrice>().await?)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

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
