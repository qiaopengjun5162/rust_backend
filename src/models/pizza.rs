// pizza.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents a request to buy a pizza.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name is required"))]
    pub pizza_name: String,
}

/// Represents a request to update a pizza's URL.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

/// Represents a pizza.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    /// Creates a new instance of Pizza.
    pub fn new(uuid: String, pizza_name: String) -> Self {
        Self { uuid, pizza_name }
    }
}
